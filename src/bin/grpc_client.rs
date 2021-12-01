use url_short::url_short_client::UrlShortClient;
use url_short::{ShortRequest, ExpandRequest};

pub mod url_short {
    tonic::include_proto!("url_short");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = UrlShortClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(ShortRequest {
        url: "Tonic".into(),
    });

    let response = client.short(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
