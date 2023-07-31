use camino::Utf8PathBuf;
use sea_orm::{
    sea_query::{ArrayType, Nullable, ValueType},
    ColumnType, TryGetable, Value,
};
use serde::Serialize;
use std::{ops::Deref, path::PathBuf};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(transparent)]
pub struct PathBufSql(pub Utf8PathBuf);

// SeaOrm type traits

impl From<PathBufSql> for Value {
    fn from(value: PathBufSql) -> Self {
        value.0.to_string().into()
    }
}

impl TryGetable for PathBufSql {
    fn try_get_by<I: sea_orm::ColIdx>(
        res: &sea_orm::QueryResult,
        index: I,
    ) -> Result<Self, sea_orm::TryGetError> {
        Ok(Self(String::try_get_by(res, index)?.into()))
    }
}

impl ValueType for PathBufSql {
    fn try_from(v: Value) -> Result<Self, sea_orm::sea_query::ValueTypeErr> {
        Ok(Self(<String as ValueType>::try_from(v)?.into()))
    }

    fn type_name() -> String {
        stringify!(PathBufSql).to_owned()
    }

    fn array_type() -> ArrayType {
        ArrayType::String
    }

    fn column_type() -> ColumnType {
        ColumnType::String(None)
    }
}

impl Nullable for PathBufSql {
    fn null() -> Value {
        Value::String(None)
    }
}

// From/to PathBuf conversion

impl From<Utf8PathBuf> for PathBufSql {
    fn from(value: Utf8PathBuf) -> Self {
        Self(value)
    }
}

impl From<PathBufSql> for Utf8PathBuf {
    fn from(value: PathBufSql) -> Self {
        value.0
    }
}

impl TryFrom<PathBuf> for PathBufSql {
    type Error = <Utf8PathBuf as TryFrom<PathBuf>>::Error;

    fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
        Ok(Self(value.try_into()?))
    }
}

impl Deref for PathBufSql {
    type Target = Utf8PathBuf;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
