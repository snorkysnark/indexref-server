use std::path::Path;

use eyre::ContextCompat;

pub trait PathExt {
    fn try_to_str(&self) -> eyre::Result<&str>;
    fn extension_str(&self) -> Option<&str>;
}

impl PathExt for Path {
    fn try_to_str(&self) -> eyre::Result<&str> {
        self.to_str()
            .with_context(|| format!("Non-UTF8 path: {}", self.display()))
    }

    fn extension_str(&self) -> Option<&str> {
        self.extension().and_then(|ext| ext.to_str())
    }
}
