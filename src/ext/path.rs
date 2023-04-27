use std::path::Path;

use crate::result::{AppError, AppResult};

pub trait PathExt {
    fn try_to_str(&self) -> AppResult<&str>;
    fn extension_str(&self) -> Option<&str>;
}

impl PathExt for Path {
    fn try_to_str(&self) -> AppResult<&str> {
        self.to_str()
            .ok_or_else(|| AppError::NonUtf8Path(self.to_owned()))
    }

    fn extension_str(&self) -> Option<&str> {
        self.extension().and_then(|ext| ext.to_str())
    }
}
