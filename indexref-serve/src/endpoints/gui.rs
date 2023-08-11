use axum::{
    body::Body,
    extract::State,
    http::Request,
    response::Response,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use hyper::{client::HttpConnector, StatusCode, Uri};

type Client = hyper::client::Client<HttpConnector, Body>;

pub async fn index_html() -> impl IntoResponse {
    Html(include_str!("./index.html"))
}

pub fn static_asset_router() -> Router {
    let client: Client = hyper::Client::builder().build(HttpConnector::new());

    Router::new()
        .route("/*path", get(static_asset_handler))
        .with_state(client)
}

async fn static_asset_handler(
    State(client): State<Client>,
    mut req: Request<Body>,
) -> Result<Response, StatusCode> {
    let path = req.uri().path();
    let path_query = req
        .uri()
        .path_and_query()
        .map(|v| v.as_str())
        .unwrap_or(path);

    let uri = format!("http://localhost:3000/static{}", path_query);
    *req.uri_mut() = Uri::try_from(uri).unwrap();

    Ok(client
        .request(req)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?
        .into_response())
}
