use std::ops::Deref;

use relative_path::RelativePathBuf;
use sea_orm::{
    sea_query::{ArrayType, Nullable, ValueType},
    ColumnType, TryGetable, Value,
};
use serde::Serialize;

// SQL-compatible wrapper around RelativePathBuf
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(transparent)]
pub struct RelativePathSql(RelativePathBuf);

impl Deref for RelativePathSql {
    type Target = RelativePathBuf;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<RelativePathBuf> for RelativePathSql {
    fn from(value: RelativePathBuf) -> Self {
        Self(value)
    }
}

impl From<RelativePathSql> for RelativePathBuf {
    fn from(value: RelativePathSql) -> Self {
        value.0
    }
}

impl From<RelativePathSql> for Value {
    fn from(value: RelativePathSql) -> Self {
        value.0.into_string().into()
    }
}

impl TryGetable for RelativePathSql {
    fn try_get_by<I: sea_orm::ColIdx>(
        res: &sea_orm::QueryResult,
        index: I,
    ) -> Result<Self, sea_orm::TryGetError> {
        Ok(Self(String::try_get_by(res, index)?.into()))
    }
}

impl ValueType for RelativePathSql {
    fn try_from(v: Value) -> Result<Self, sea_orm::sea_query::ValueTypeErr> {
        Ok(Self(<String as ValueType>::try_from(v)?.into()))
    }

    fn type_name() -> String {
        stringify!(RelativePathSql).to_owned()
    }

    fn array_type() -> ArrayType {
        ArrayType::String
    }

    fn column_type() -> ColumnType {
        ColumnType::String(None)
    }
}

impl Nullable for RelativePathSql {
    fn null() -> Value {
        Value::String(None)
    }
}
