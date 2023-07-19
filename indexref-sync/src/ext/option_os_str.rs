use std::ffi::OsStr;

pub trait OptionOsStrExt<'a> {
    fn to_str(self) -> Option<&'a str>;
}

impl<'a> OptionOsStrExt<'a> for Option<&'a OsStr> {
    fn to_str(self) -> Option<&'a str> {
        self.and_then(|os_str| os_str.to_str())
    }
}
