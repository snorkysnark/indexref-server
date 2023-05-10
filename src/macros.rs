// Convert and enum from and to &'static str
macro_rules! from_to_str {
    ($vis:vis $name:ident { $($variant:path => $variant_str:literal),* $(,)? }) => {
        $vis fn $name(&self) -> &'static str {
            match self {
                $($variant => $variant_str),*
            }
        }

        ::paste::paste!{
            $vis fn [<from_ $name>](str: &str) -> Option<Self> {
                match str {
                    $($variant_str => Some($variant)),*,
                    _ => None,
                }
            }
        }
    };
}

pub(crate) use from_to_str;
