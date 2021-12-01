use axum::extract;
use axum::response;
use serde::{Serialize, Deserialize};
use url_short::url_short_client::UrlShortClient;
use url_short::{ShortRequest, ExpandRequest};

pub mod url_short {
    tonic::include_proto!("url_short");
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let grpc_server = std::env::var("GRPC_SERVER").expect("GRPC_SERVER env var must be defined");
    let grpc_url = format!("http://{}:50051", grpc_server);

    let mut grpc_client = UrlShortClient::connect(grpc_url).await?;

    let app = axum::Router::new()
        .route("/short", axum::routing::post(short))
        .route("/expand/:shorten", axum::routing::get(expand))
        .layer(axum::AddExtensionLayer::new(grpc_client));

    axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Deserialize)]
struct Req {
    url: String,
}

#[derive(Serialize)]
struct Resp {
    shorten: String,
}

async fn short(extract::Extension(grpc_client): extract::Extension<UrlShortClient<T>>, extract::Json(payload): extract::Json<Req>) -> response::Json<Resp> {
    let url = payload.url;
    let req = tonic::Request::new(ShortRequest {url});
    let resp = grpc_client.short(req).await?;
    let shorten = resp.into_inner().shorten;
    response::Json(Resp{shorten})
}

async fn expand(extract::Extension(grpc_client): extract::Extension<UrlShortClient<T>>, extract::Path(shorten): extract::Path<&str>) {
    let req = tonic::Request::new(ExpandRequest{shorten});
    let resp = grpc_client.expand(req).await?;
    let url = resp.into_inner().url;
    unimplemented!()
}
