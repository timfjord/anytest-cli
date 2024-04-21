use rev_buf_reader::RevBufReader;

use crate::LineNr;
use std::error::Error;
use std::fs::File;
use std::mem;
use std::path::PathBuf;
use std::{
    env,
    io::{self, BufRead, Seek},
    ops,
};

/// A line numbers range. 1-based.
struct LRange {
    start: LineNr,
    step: isize,
    end: Option<LineNr>,
}

impl LRange {
    const MIN: u8 = 1;

    fn new(start: LineNr, step: isize, end: Option<LineNr>) -> Result<Self, Box<dyn Error>> {
        if start < Self::MIN.into() {
            return Err("`start` isn't 1-based".into());
        }

        if let Some(end) = end {
            if end < Self::MIN.into() {
                return Err("`end` isn't 1-based".into());
            }
        }

        Ok(Self { start, step, end })
    }

    fn try_from_range<T: Into<LineNr> + Clone>(
        value: &impl ops::RangeBounds<T>,
    ) -> Result<Self, Box<dyn Error>> {
        let start: LineNr = match value.start_bound() {
            ops::Bound::Included(start) => start.clone().into(),
            ops::Bound::Excluded(start) => start.clone().into(),
            ops::Bound::Unbounded => 1,
        };

        let end: Option<LineNr> = match value.end_bound() {
            ops::Bound::Included(end) => Some(end.clone().into()),
            ops::Bound::Excluded(end) => {
                let end: LineNr = end.clone().into();

                if start <= end {
                    Some(end - 1)
                } else {
                    Some(end + 1)
                }
            }
            ops::Bound::Unbounded => None,
        };

        let mut step = 1;
        if let Some(end) = end {
            if start > end {
                step = -1;
            }
        }

        Self::new(start, step, end)
    }

    fn start(&self) -> LineNr {
        self.start
    }

    fn forward_to(&self) -> LineNr {
        if self.is_asc() {
            self.start()
        } else {
            self.start() + 1
        }
    }

    fn is_asc(&self) -> bool {
        self.step > 0
    }

    fn is_desc(&self) -> bool {
        self.step < 0
    }
}

impl Iterator for LRange {
    type Item = LineNr;

    fn next(&mut self) -> Option<Self::Item> {
        let n = self.step + isize::try_from(self.start).ok()?;
        let start = LineNr::try_from(n).ok()?;

        if let Some(end) = self.end {
            if (self.is_asc() && start > end + 1) || (self.is_desc() && start < end - 1) {
                return None;
            }
        }

        Some(mem::replace(&mut self.start, start))
    }
}

type LineWithNr = (String, LineNr);

#[derive(Debug)]
pub struct RelPath {
    root: PathBuf,
    path: PathBuf,
    rel: PathBuf,
}

impl RelPath {
    pub fn new(root: Option<&str>, path: &str) -> Result<Self, Box<dyn Error>> {
        let mut rel_root: Option<PathBuf> = None;
        let mut root = if let Some(root) = root {
            PathBuf::from(root)
        } else {
            env::current_dir()?
        };

        if !root.is_absolute() {
            rel_root = Some(root.clone());
            root = env::current_dir()?.join(root);
        }

        if !root.is_dir() {
            return Err("Root path must be an existing directory".into());
        }

        let mut path = PathBuf::from(path);

        if !path.is_absolute() {
            if let Some(rel) = rel_root {
                path = path
                    .clone()
                    .strip_prefix(&rel)
                    .unwrap_or(&path)
                    .to_path_buf();
            }
            path = root.join(path);
        }

        if !path.exists() {
            return Err("Path does not exist".into());
        }

        let rel_path = path
            .clone()
            .strip_prefix(&root)
            .map_err(|_| "Path must be a subpath of the root path")?
            .to_path_buf();

        Ok(Self {
            root,
            path,
            rel: rel_path,
        })
    }

    pub fn root(&self) -> &PathBuf {
        &self.root
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn rel(&self) -> &PathBuf {
        &self.rel
    }

    pub fn rel_str(&self) -> &str {
        self.rel().to_str().unwrap_or_default()
    }

    /// Opens the file and advances to the passed line.
    pub fn open(&self, line: LineNr) -> Result<io::BufReader<File>, io::Error> {
        let file = File::open(self.path())?;
        let mut buf_reader = io::BufReader::new(file);
        let mut current_line = 1;
        let mut buf = String::new();

        while current_line < line {
            if buf_reader.read_line(&mut buf)? == 0 {
                return Err(io::Error::new(
                    io::ErrorKind::UnexpectedEof,
                    format!("Line #{} not found", line),
                ));
            }
            buf.clear();
            current_line += 1;
        }

        Ok(buf_reader)
    }

    pub fn lines(
        &self,
        range: impl ops::RangeBounds<LineNr>,
    ) -> Result<Box<dyn Iterator<Item = LineWithNr>>, Box<dyn Error>> {
        let numbers: LRange = LRange::try_from_range(&range)?;
        let mut buffer = self.open(numbers.forward_to())?;

        let lines: Box<dyn Iterator<Item = Result<String, io::Error>>> = if numbers.is_desc() {
            let position = buffer.stream_position()?;
            let mut rev_reader = RevBufReader::new(buffer.into_inner());

            rev_reader.seek(io::SeekFrom::Start(position))?;

            Box::new(rev_reader.lines())
        } else {
            Box::new(buffer.lines())
        };

        Ok(Box::new(lines.map(Result::unwrap_or_default).zip(numbers)))
    }

    pub fn file(&self, path: &str) -> Result<Self, Box<dyn Error>> {
        Self::new(Some(self.root().to_str().ok_or("Invalid root path")?), path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    fn rel_path_error(root: &str, path: &str) -> String {
        RelPath::new(Some(root), path).unwrap_err().to_string()
    }

    #[test]
    fn test_rel_path_new() {
        let folder = "tests/fixtures/folder/subfolder";
        let file = "file.txt";
        let rel = format!("{}/{}", folder, file);
        let other_file = env::current_dir()
            .unwrap()
            .join("tests/fixtures/folder/file.txt")
            .to_str()
            .unwrap()
            .to_string();

        assert_eq!(
            rel_path_error("tests/fixtures/folder/file.txt", ""),
            "Root path must be an existing directory"
        );

        assert_eq!(
            rel_path_error("/tmp/s0me_f0lDer", ""),
            "Root path must be an existing directory"
        );

        assert_eq!(
            rel_path_error(folder, "non_existent.rs"),
            "Path does not exist"
        );

        assert_eq!(
            rel_path_error(folder, &other_file),
            "Path must be a subpath of the root path"
        );

        let rel_path = RelPath::new(Some(folder), "file.txt").unwrap();

        assert!(rel_path.root().ends_with(&folder));
        assert!(rel_path.path().ends_with(&rel));
        assert_eq!(*rel_path.rel(), PathBuf::from(file));

        let rel_path = RelPath::new(Some(folder), &rel).unwrap();

        assert!(rel_path.root().ends_with(&folder));
        assert!(rel_path.path().ends_with(&rel));
        assert_eq!(*rel_path.rel(), PathBuf::from(file));
    }

    fn read_line(line: LineNr) -> Result<String, Box<dyn Error>> {
        let mut buf_reader = RelPath::new(Some("tests/fixtures/folder"), "file.txt")
            .unwrap()
            .open(line)?;
        let mut buf = String::new();
        buf_reader.read_line(&mut buf).unwrap();
        Ok(buf)
    }

    #[test]
    fn test_rel_path_open() {
        assert_eq!(read_line(1).unwrap(), "line1\n");
        assert_eq!(read_line(3).unwrap(), "line3\n");
        assert_eq!(read_line(10).unwrap(), "");

        let error = read_line(11).unwrap_err();
        assert_eq!(error.to_string(), "Line #11 not found");
    }

    fn get_lines(range: impl ops::RangeBounds<LineNr>) -> Result<Vec<LineWithNr>, Box<dyn Error>> {
        let lines = RelPath::new(Some("tests/fixtures/folder"), "file.txt")?
            .lines(range)?
            .collect::<Vec<LineWithNr>>();

        Ok(lines)
    }

    #[test]
    fn test_rel_path_lines() {
        assert_eq!(
            get_lines(..).unwrap(),
            vec![
                (String::from("line1"), 1),
                (String::from("line2"), 2),
                (String::from("line3"), 3),
                (String::from("line4"), 4),
                (String::from("line5"), 5),
                (String::from("line6"), 6),
                (String::from("line7"), 7),
                (String::from("line8"), 8),
                (String::from("line9"), 9)
            ]
        );

        assert_eq!(
            get_lines(..5).unwrap(),
            vec![
                (String::from("line1"), 1),
                (String::from("line2"), 2),
                (String::from("line3"), 3),
                (String::from("line4"), 4)
            ]
        );

        assert_eq!(
            get_lines(2..=6).unwrap(),
            vec![
                (String::from("line2"), 2),
                (String::from("line3"), 3),
                (String::from("line4"), 4),
                (String::from("line5"), 5),
                (String::from("line6"), 6)
            ]
        );

        assert_eq!(
            get_lines(7..).unwrap(),
            vec![
                (String::from("line7"), 7),
                (String::from("line8"), 8),
                (String::from("line9"), 9)
            ]
        );

        assert_eq!(
            get_lines(5..1).unwrap(),
            vec![
                (String::from("line5"), 5),
                (String::from("line4"), 4),
                (String::from("line3"), 3),
                (String::from("line2"), 2)
            ]
        );

        assert_eq!(
            get_lines(6..=1).unwrap(),
            vec![
                (String::from("line6"), 6),
                (String::from("line5"), 5),
                (String::from("line4"), 4),
                (String::from("line3"), 3),
                (String::from("line2"), 2),
                (String::from("line1"), 1)
            ]
        );

        assert_eq!(get_lines(1..=1).unwrap(), vec![(String::from("line1"), 1)]);

        let error = get_lines(0..5).unwrap_err();
        assert_eq!(error.to_string(), "`start` isn't 1-based");

        let error = get_lines(1..1).unwrap_err();
        assert_eq!(error.to_string(), "`end` isn't 1-based");
    }
}
