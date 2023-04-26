use std::path::Path;

use super::{AppError, AppResult};

pub trait TryToStr {
    fn try_to_str(&self) -> AppResult<&str>;
}

impl TryToStr for Path {
    fn try_to_str(&self) -> AppResult<&str> {
        self.to_str()
            .ok_or_else(|| AppError::NonUtf8Path(self.to_owned()))
    }
}
