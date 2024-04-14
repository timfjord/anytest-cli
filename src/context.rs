use std::{error::Error, path::PathBuf};

use crate::{LineNr, RelPath};

#[derive(Debug)]
pub struct Context {
    rel_path: RelPath,
    line_nr: Option<LineNr>,
}

impl Context {
    pub fn new(
        root: Option<&str>,
        path: &str,
        line_nr: Option<LineNr>,
    ) -> Result<Self, Box<dyn Error>> {
        let rel_path = RelPath::new(root, path)?;

        Ok(Self { rel_path, line_nr })
    }

    pub fn root(&self) -> &PathBuf {
        &self.rel_path.root()
    }

    pub fn path(&self) -> &PathBuf {
        &self.rel_path.path()
    }

    pub fn path_str(&self) -> &str {
        &self.rel_path.path().to_str().unwrap_or_default()
    }

    pub fn rel(&self) -> &PathBuf {
        &self.rel_path.rel()
    }

    pub fn line(&self) -> Option<LineNr> {
        self.line_nr
    }
}
