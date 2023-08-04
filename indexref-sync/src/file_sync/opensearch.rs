use entity::NodePresentaion;
use eyre::Result;
use opensearch::{BulkOperation, BulkOperations, BulkParts, DeleteByQueryParts, OpenSearch};
use serde_json::json;

pub async fn delete_by_file_id(file_id: i32) -> Result<()> {
    let response: serde_json::Value = OpenSearch::default()
        .delete_by_query(DeleteByQueryParts::Index(&["nodes"]))
        .body(json!({ "query": { "match": { "file_id": file_id } } }))
        .send()
        .await?
        .json()
        .await?;

    println!("{}", serde_json::to_string_pretty(&response)?);

    Ok(())
}

pub async fn add_documents(nodes: Vec<NodePresentaion>) -> Result<()> {
    if nodes.len() > 0 {
        let mut ops = BulkOperations::new();
        for node in nodes.into_iter() {
            let id = node.id();
            ops.push(BulkOperation::index(node).id(id.to_string()))?;
        }

        let response: serde_json::Value = OpenSearch::default()
            .bulk(BulkParts::Index("nodes"))
            .body(vec![ops])
            .send()
            .await?
            .json()
            .await?;

        println!("{}", serde_json::to_string_pretty(&response)?);
    }

    Ok(())
}
