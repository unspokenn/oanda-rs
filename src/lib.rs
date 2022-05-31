#![crate_type = "lib"]

// pub(crate) mod macros;
mod serdes;
mod client;
mod models;
// mod traits;
mod request;
mod requests;
mod response;


// pub(crate) use traits::*;
pub(crate) use serdes::*;
pub use models::*;
pub use request::*;
pub use requests::*;
pub use response::*;
pub use client::Client;

// pub type Result<T> = std::result::Result<T, std::convert::Infallible>;
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
pub const OANDA_HOST: &'static str = "https://api-fxpractice.oanda.com";
pub const OANDA_STREAM_HOST: &'static str = "https://stream-fxpractice.oanda.com";
// use warp::hyper::client::Client;
//
// pub trait RequestPathsTrait {
//     fn new() -> Self;
//     fn get_uri(&self) -> &String {
//         &self.uri
//     }
//     fn get_method(&self) -> &String {
//         &self.method
//     }
// }


// pub fn get() -> Result<(), Box<dyn std::error::Error>> {
//     let uri = Uri::builder().scheme("https").authority("stream-fxpractice.oanda.com").path_and_query("/").build().unwrap();
//
//
//     let target_stream = TcpStream::connect("example.com:80").await?;
//
//     let (mut request_sender, connection) = conn::handshake(target_stream).await?;
//
//     // spawn a task to poll the connection and drive the HTTP state
//     tokio::spawn(async move {
//         if let Err(e) = connection.await {
//             eprintln!("Error in connection: {}", e);
//         }
//     });
//
//     let request = Request::builder()
//     // We need to manually add the host header because SendRequest does not.header("Host", "/v3/accounts/!/pricing/stream").method("GET").body(Body::from(""))?;
//     let response = request_sender.send_request(request).await?;
//     assert!(response.status() == StatusCode::OK);
//
//     // To send via the same connection again, it may not work as it may not be ready,
//     // so we have to wait until the request_sender becomes ready.
//     request_sender.ready().await?;
//     let request = Request::builder().header("Host", "example.com").method("GET").body(Body::from(""))?;
//     let response = request_sender.send_request(request).await?;
//     assert!(response.status() == StatusCode::OK);
//     Ok(())
// }
