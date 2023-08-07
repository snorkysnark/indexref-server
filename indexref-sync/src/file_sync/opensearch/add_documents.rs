use entity::NodePresentaion;
use opensearch::{BulkOperation, BulkOperations, BulkParts, OpenSearch};
use serde::{Deserialize, Serialize};
use serde_json::{Map as JsonMap, Value as JsonValue};
use thiserror::Error;

// Full: https://opensearch.org/docs/latest/api-reference/document-apis/bulk/
#[derive(Debug, Serialize, Deserialize)]
pub struct BulkApiResponse {
    errors: bool,
    #[serde(flatten)]
    other: JsonMap<String, JsonValue>,
}

#[derive(Debug, Error)]
pub enum BulkApiError {
    #[error("opensearch error: {0}")]
    OpenSearchError(#[from] opensearch::Error),
    #[error("response has errors: {0:#?}")]
    HasErrors(BulkApiResponse),
}

pub trait RecordId {
    type Id: Into<String>;

    fn record_id(&self) -> Self::Id;
}

impl RecordId for NodePresentaion {
    type Id = String;

    fn record_id(&self) -> Self::Id {
        self.id().to_string()
    }
}

pub async fn add_documents<T>(client: &OpenSearch, nodes: Vec<T>) -> Result<(), BulkApiError>
where
    T: Serialize + RecordId,
{
    if nodes.is_empty() {
        return Ok(());
    }

    let mut ops = BulkOperations::new();
    for node in nodes.into_iter() {
        let id = node.record_id();
        ops.push(BulkOperation::index(node).id(id))?;
    }

    let response: BulkApiResponse = client
        .bulk(BulkParts::Index("nodes"))
        .body(vec![ops])
        .send()
        .await?
        .json()
        .await?;

    if !response.errors {
        Ok(())
    } else {
        Err(BulkApiError::HasErrors(response))
    }
}
