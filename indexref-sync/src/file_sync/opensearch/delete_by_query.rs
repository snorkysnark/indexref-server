use opensearch::{DeleteByQueryParts, OpenSearch};
use serde::{Deserialize, Serialize};
use serde_json::{json, Map as JsonMap, Value as JsonValue};
use thiserror::Error;

// Full: https://opensearch.org/docs/latest/api-reference/document-apis/delete-by-query/#response
#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteByQueryResponse {
    failures: Vec<JsonValue>,
    #[serde(flatten)]
    other: JsonMap<String, JsonValue>,
}

#[derive(Debug, Error)]
pub enum DeleteByQueryError {
    #[error("opensearch error: {0}")]
    OpenSearchError(#[from] opensearch::Error),
    #[error("response has failures: {0:#?}")]
    HasFailures(DeleteByQueryResponse),
}

pub async fn delete_by_query(
    client: &OpenSearch,
    index: &str,
    query: impl Serialize,
) -> Result<(), DeleteByQueryError> {
    let response: DeleteByQueryResponse = client
        .delete_by_query(DeleteByQueryParts::Index(&[index]))
        .body(json!({ "query": query }))
        .send()
        .await?
        .json()
        .await?;

    if response.failures.is_empty() {
        Ok(())
    } else {
        Err(DeleteByQueryError::HasFailures(response))
    }
}
