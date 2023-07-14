use std::fmt::Display;

pub trait ResultExt<OK, ERR> {
    fn ok_log_errors(self) -> Option<OK>;
}

impl<OK, ERR> ResultExt<OK, ERR> for Result<OK, ERR>
where
    ERR: Display,
{
    fn ok_log_errors(self) -> Option<OK> {
        match self {
            Ok(e) => Some(e),
            Err(err) => {
                eprintln!("{err}");
                None
            }
        }
    }
}
