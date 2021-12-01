use url_short::url_short_client::UrlShortClient;
use url_short::{ShortRequest, ExpandRequest};

pub mod url_short {
    tonic::include_proto!("url_short");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = UrlShortClient::connect("http://[::1]:50051").await?;

    let req = tonic::Request::new(ShortRequest {
        url: "http://www.example.com".to_string(),
    });
    let resp = client.short(req).await?;
    println!("{:?}", resp);

    let shorten = resp.into_inner().shorten;
    let req = tonic::Request::new(ExpandRequest{shorten});
    let resp = client.expand(req).await?;
    println!("{:?}", resp);

    Ok(())
}
