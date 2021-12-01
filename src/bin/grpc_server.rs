use r2d2::Pool;
use r2d2_redis::RedisConnectionManager;
use r2d2_redis::redis::Commands;
use tonic::{transport::Server, Request, Response, Status};
use url_short::url_short_server::{UrlShort, UrlShortServer};
use url_short::{ShortRequest, ShortResponse, ExpandRequest, ExpandResponse};

pub mod url_short {
    tonic::include_proto!("url_short");
}

struct MyService {
    pool: Pool<RedisConnectionManager>,
}

impl MyService {
    fn new(pool:Pool<RedisConnectionManager>) -> MyService {
        MyService{pool}
    }
}

#[tonic::async_trait]
impl UrlShort for MyService {
    async fn short(
        &self,
        request: Request<ShortRequest>,
    ) -> Result<Response<ShortResponse>, Status>{
        let sreq: ShortRequest = request.into_inner();
        let shorten = nanoid::nanoid!(8);
        let mut conn = self.pool.get().unwrap();
        let _ : () = conn.set(&shorten, sreq.url).unwrap();
        Ok(Response::new(ShortResponse{shorten}))
    }
    async fn expand(
        &self,
        request: Request<ExpandRequest>,
    ) -> Result<Response<ExpandResponse>, Status>{
        let ereq: ExpandRequest = request.into_inner();
        let mut conn = self.pool.get().unwrap();
        let url: String = conn.get(ereq.shorten).unwrap();
        Ok(Response::new(ExpandResponse{url}))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let redis_host = std::env::var("REDIS_HOST").expect("REDIS_HOST env var must be defined");
    let redis_url = format!("redis://{}", redis_host);
    let manager = RedisConnectionManager::new(redis_url).expect("failed to init manager");
    let pool = Pool::builder().build(manager).expect("failed to init connection pool");

    let greeter = MyService::new(pool);
    let addr = "[::1]:50051".parse()?;

    Server::builder()
        .add_service(UrlShortServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
