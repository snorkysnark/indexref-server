use std::{fmt::Display, str::FromStr};

use sea_orm::{query::Value, DbErr, TryGetable};
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(transparent)]
pub struct StringVec<T>(Vec<T>)
where
    T: FromStr + ToString;

impl<T> FromStr for StringVec<T>
where
    T: FromStr + ToString,
{
    type Err = <T as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let items: Result<Vec<T>, _> = s.split(',').map(|item| item.parse()).collect();
        Ok(Self(items?))
    }
}

impl<T> Display for StringVec<T>
where
    T: FromStr + ToString,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let joined = self
            .0
            .iter()
            .map(|item| item.to_string())
            .collect::<Vec<_>>()
            .join(",");

        joined.fmt(f)
    }
}

impl<T> From<StringVec<T>> for Value
where
    T: FromStr + ToString,
{
    fn from(value: StringVec<T>) -> Self {
        value.to_string().into()
    }
}

impl<T> TryGetable for StringVec<T>
where
    T: FromStr + ToString,
    <T as FromStr>::Err: ToString,
{
    fn try_get_by<I: sea_orm::ColIdx>(
        res: &sea_orm::QueryResult,
        index: I,
    ) -> Result<Self, sea_orm::TryGetError> {
        Ok(match Option::<String>::try_get_by(res, index)? {
            Some(string) => string
                .parse()
                .map_err(|err: T::Err| DbErr::Custom(err.to_string()))?,
            None => Self(vec![]),
        })
    }
}

impl<T> From<Vec<T>> for StringVec<T>
where
    T: FromStr + ToString,
{
    fn from(value: Vec<T>) -> Self {
        Self(value)
    }
}

impl<T> From<StringVec<T>> for Vec<T>
where
    T: FromStr + ToString,
{
    fn from(value: StringVec<T>) -> Self {
        value.0
    }
}
