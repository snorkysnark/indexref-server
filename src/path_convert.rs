use std::path::{Path, PathBuf};

use relative_path::RelativePathBuf;

#[justerror::Error]
pub enum PathConvertError {
    PathDiffError { path: PathBuf, base: PathBuf },
    RelativePathError(#[from] relative_path::FromPathError),
}

pub trait ToRelativePath {
    fn to_relative_path(&self, base: impl AsRef<Path>)
        -> Result<RelativePathBuf, PathConvertError>;
}

impl<T: AsRef<Path>> ToRelativePath for T {
    fn to_relative_path(
        &self,
        base: impl AsRef<Path>,
    ) -> Result<RelativePathBuf, PathConvertError> {
        Ok(RelativePathBuf::from_path(
            pathdiff::diff_paths(self.as_ref(), base.as_ref()).ok_or_else(|| {
                PathConvertError::PathDiffError {
                    path: self.as_ref().to_owned(),
                    base: base.as_ref().to_owned(),
                }
            })?,
        )?)
    }
}
