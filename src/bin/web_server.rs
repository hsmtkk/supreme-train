use axum::extract;
use axum::response;
use nanoid::nanoid;
use r2d2_memcache::MemcacheConnectionManager;
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

async fn short_url(extract::Extension(pool): extract::Extension<r2d2::Pool<MemcacheConnectionManager>>, extract::Json(payload): extract::Json<Req>) -> response::Json<Resp> {
    let url = payload.url;
    let shorten = nanoid!(8);
    let conn = pool.get().expect("failed to get connection");
    conn.set(&shorten, &url, 0).expect("failed to set key&value");
    response::Json(Resp{shorten})
}
