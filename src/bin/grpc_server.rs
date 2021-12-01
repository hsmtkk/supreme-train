use tonic::{transport::Server, Request, Response, Status};

use url_short::url_short_server::{UrlShort, UrlShortServer};
use url_short::{ShortRequest, ShortResponse, ExpandRequest, ExpandResponse};

pub mod url_short {
    tonic::include_proto!("url_short");
}

#[derive(Default)]
struct MyService {}

#[tonic::async_trait]
impl UrlShort for MyService {
    async fn short(
        &self,
        request: Request<ShortRequest>,
    ) -> Result<Response<ShortResponse>, Status>{
        unimplemented!()
    }
    async fn expand(
        &self,
        request: Request<ExpandRequest>,
    ) -> Result<Response<ExpandResponse>, Status>{
        unimplemented!()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let greeter = MyService::default();

    Server::builder()
        .add_service(UrlShortServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
