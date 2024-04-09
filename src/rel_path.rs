use rev_buf_reader::RevBufReader;

use crate::LineNr;
use std::error::Error;
use std::fs::File;
use std::mem;
use std::path::PathBuf;
use std::{
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

#[derive(Debug)]
pub struct RelPath {
    root: PathBuf,
    path: PathBuf,
    rel_path: PathBuf,
}

impl RelPath {
    pub fn new(root: Option<&str>, path: &str) -> Result<Self, Box<dyn Error>> {
        let root = if let Some(root) = root {
            PathBuf::from(root)
        } else {
            std::env::current_dir()?
        };

        if !root.is_absolute() {
            return Err("Root path must be absolute".into());
        }

        if !root.is_dir() {
            return Err("Root path must be an existing directory".into());
        }

        let mut path = PathBuf::from(path);

        if !path.is_absolute() {
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
            rel_path,
        })
    }

    pub fn root(&self) -> &PathBuf {
        &self.root
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn rel_path(&self) -> &PathBuf {
        &self.rel_path
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
    ) -> Result<Box<dyn Iterator<Item = (Result<String, io::Error>, LineNr)>>, Box<dyn Error>> {
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

        Ok(Box::new(lines.zip(numbers)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        env,
        fs::{self, File},
        io::Write,
    };

    fn create_folder(base: &PathBuf, path: &str) -> Result<PathBuf, Box<dyn Error>> {
        let folder = base.join(path);

        match fs::create_dir(&folder) {
            Ok(_) => Ok(folder),
            Err(error) => match error.kind() {
                std::io::ErrorKind::AlreadyExists => Ok(folder),
                _ => Err(error.into()),
            },
        }
    }

    fn rel_path_error(root: &str, path: &str) -> String {
        RelPath::new(Some(root), path).unwrap_err().to_string()
    }

    #[test]
    fn test_rel_path_new() {
        let dir = env::temp_dir();
        let folder = create_folder(&dir, "folder").unwrap();
        let file = folder.join("file.rs");
        let other_file = dir.join("other_file.rs");

        File::create(&file).unwrap();
        File::create(&other_file).unwrap();

        assert_eq!(
            rel_path_error("some_folder", ""),
            "Root path must be absolute"
        );

        assert_eq!(
            rel_path_error(file.to_str().unwrap(), ""),
            "Root path must be an existing directory"
        );

        assert_eq!(
            rel_path_error("/tmp/some_folder", ""),
            "Root path must be an existing directory"
        );

        assert_eq!(
            rel_path_error(folder.to_str().unwrap(), "non_existent.rs"),
            "Path does not exist"
        );

        assert_eq!(
            rel_path_error(folder.to_str().unwrap(), other_file.to_str().unwrap()),
            "Path must be a subpath of the root path"
        );

        let rel_path =
            RelPath::new(Some(folder.to_str().unwrap()), file.to_str().unwrap()).unwrap();

        assert_eq!(*rel_path.root(), folder);
        assert_eq!(*rel_path.path(), file);
        assert_eq!(*rel_path.rel_path(), PathBuf::from("file.rs"));
    }

    fn read_line(root: &str, path: &str, line: LineNr) -> Result<String, Box<dyn Error>> {
        let mut buf_reader = RelPath::new(Some(root), path).unwrap().open(line)?;
        let mut buf = String::new();
        buf_reader.read_line(&mut buf).unwrap();
        Ok(buf)
    }

    #[test]
    fn test_rel_path_open() {
        let dir = env::temp_dir();
        let folder = create_folder(&dir, "folder").unwrap();
        let file = folder.join("file.rs");

        let mut f = File::create(&file).unwrap();
        f.write_all("line1\nline2\nline3\nline4\nline5".as_bytes())
            .unwrap();

        assert_eq!(
            read_line(folder.to_str().unwrap(), "file.rs", 1).unwrap(),
            "line1\n"
        );
        assert_eq!(
            read_line(folder.to_str().unwrap(), "file.rs", 3).unwrap(),
            "line3\n"
        );
        assert_eq!(
            read_line(folder.to_str().unwrap(), "file.rs", 6).unwrap(),
            ""
        );

        let error = read_line(folder.to_str().unwrap(), "file.rs", 7).unwrap_err();
        assert_eq!(error.to_string(), "Line #7 not found");
    }

    fn get_lines(
        root: &str,
        path: &str,
        range: impl ops::RangeBounds<LineNr>,
    ) -> Result<Vec<String>, Box<dyn Error>> {
        let lines = RelPath::new(Some(root), path)?
            .lines(range)?
            .map(|(line, line_nr)| format!("{}: {}", line_nr, line.unwrap()))
            .collect::<Vec<String>>();

        Ok(lines)
    }

    #[test]
    fn test_rel_path_lines() {
        let dir = env::temp_dir();
        let folder = create_folder(&dir, "folder").unwrap();
        let file = folder.join("file.rs");

        let mut f = File::create(&file).unwrap();
        f.write_all("line1\nline2\nline3\nline4\nline5\nline6\nline7\nline8\nline9".as_bytes())
            .unwrap();

        assert_eq!(
            get_lines(folder.to_str().unwrap(), "file.rs", ..).unwrap(),
            vec![
                "1: line1", "2: line2", "3: line3", "4: line4", "5: line5", "6: line6", "7: line7",
                "8: line8", "9: line9"
            ]
        );

        assert_eq!(
            get_lines(folder.to_str().unwrap(), "file.rs", ..5).unwrap(),
            vec!["1: line1", "2: line2", "3: line3", "4: line4"]
        );

        assert_eq!(
            get_lines(folder.to_str().unwrap(), "file.rs", 2..=6).unwrap(),
            vec!["2: line2", "3: line3", "4: line4", "5: line5", "6: line6"]
        );

        assert_eq!(
            get_lines(folder.to_str().unwrap(), "file.rs", 7..).unwrap(),
            vec!["7: line7", "8: line8", "9: line9"]
        );

        assert_eq!(
            get_lines(folder.to_str().unwrap(), "file.rs", 5..1).unwrap(),
            vec!["5: line5", "4: line4", "3: line3", "2: line2"]
        );

        assert_eq!(
            get_lines(folder.to_str().unwrap(), "file.rs", 6..=1).unwrap(),
            vec!["6: line6", "5: line5", "4: line4", "3: line3", "2: line2", "1: line1"]
        );

        assert_eq!(
            get_lines(folder.to_str().unwrap(), "file.rs", 1..=1).unwrap(),
            vec!["1: line1"]
        );

        let error = get_lines(folder.to_str().unwrap(), "file.rs", 0..5).unwrap_err();
        assert_eq!(error.to_string(), "`start` isn't 1-based");

        let error = get_lines(folder.to_str().unwrap(), "file.rs", 1..1).unwrap_err();
        assert_eq!(error.to_string(), "`end` isn't 1-based");
    }
}
