use chrono::NaiveDateTime;
use serde::Serializer;

fn date_to_string(date: &NaiveDateTime) -> String {
    format!("{}", date.format("%Y-%m-%d %H:%M:%S"))
}

#[allow(dead_code)]
pub fn human_readable<S>(date: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&date_to_string(&date))
}

#[allow(dead_code)]
pub fn human_readable_opt<S>(opt: &Option<NaiveDateTime>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match opt.as_ref() {
        Some(date) => serializer.serialize_some(&date_to_string(date)),
        None => serializer.serialize_none(),
    }
}
