use std::time::Duration;
use futures::{future::BoxFuture, FutureExt, TryStreamExt};
// use futures::TryStreamExt;
use hashbrown::HashMap;
use hyper_tls::HttpsConnector;
use regex::Regex;
use serde::Deserialize;
// use tokio::runtime::Handle;
use warp::hyper::{Response, Body};
// use tokio::io::{stdout, AsyncWriteExt as _};
use warp::hyper::client::{HttpConnector, Client as HyperClient};
use warp::Reply;
use crate::Result;
// use tokio::io::{self, AsyncWriteExt as _, AsyncBufReadExt};
// use tokio::runtime::Handle;
// use warp::hyper::body::Bytes;
use crate::response::Streams;
// use futures::io::BufReader;
// use futures::stream::IntoAsyncRead;
// type BoxedResult<T> = Result<T>;
// type CalcFn = Box<dyn Fn(i32, i32) -> Pin<Box<dyn Future<Output = BoxedResult<i32>>>>>;

// async fn get_account_id(mut client: &Client, account_index: usize) -> BoxedResult<String> {
//     match client.list_account(RequestBuilder::new()).await?.get_id(account_index) {
//         None => Err(Error::new(std::io::ErrorKind::Other, "oh no!").into()),
//         Some(id) => Ok(id)
//     }
// }

#[repr(C)]
#[derive(Debug, Clone)]
struct BaseUrl {
    pub(crate) oanda_host: &'static str,
    pub(crate) oanda_stream_host: &'static str,
}

impl BaseUrl {
    fn new(is_demo: bool) -> Self {
        if is_demo {
            BaseUrl {
                oanda_host: "https://api-fxpractice.oanda.com",
                oanda_stream_host: "https://stream-fxpractice.oanda.com",
            }
        } else {
            BaseUrl {
                oanda_host: "https://api-fxtrade.oanda.com",
                oanda_stream_host: "https://stream-fxtrade.oanda.com",
            }
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Client {
    base_url: BaseUrl,
    pub client: HyperClient<HttpsConnector<HttpConnector>>,
    uri_params: HashMap<&'static str, String>,
    app_key: &'static str
}

impl Client {
    pub fn new(app_key: &'static str, demo: bool) -> Client {
        let mut connector = HttpConnector::new();
        connector.set_keepalive(Some(Duration::from_secs(90)));
        connector.set_reuse_address(true);
        connector.set_connect_timeout(Some(Duration::from_secs(5)));
        connector.set_nodelay(true);
        connector.enforce_http(false);
        Client {
            base_url: BaseUrl::new(demo),
            client: HyperClient::builder().build::<_, Body>(HttpsConnector::new_with_connector(connector)),
            uri_params: HashMap::new(),
            app_key
        }
    }
    #[inline(always)]
    fn uri_params(&mut self, key: &'static str) -> &String {
        if !self.uri_params.contains_key(key) {
            let value = self.map_uri_params(key).unwrap();
            self.uri_params.insert(key, value);
        }
        self.uri_params.get(key).unwrap()
    }
    fn map_uri_params(&mut self, key: &'static str) -> Result<String> {
        match key {
            "accountID" => { futures::executor::block_on(self.list_account(crate::RequestBuilder::new())).unwrap().get_id(1) },
            _ => Err(std::io::Error::new(std::io::ErrorKind::Other, "param not exist!").into()),
        }
    }
    #[inline(always)]
    fn get_uri(&mut self, path: &'static str) -> String {
        let exp = Regex::new(r"\{\{|}}|\{([^}]+)}").unwrap();
        let mut uri = path.to_string();
        for capture in exp.captures_iter(path) {
            if let Some(matched) = capture.get(1) {
                uri = path.replace(format!("{{{}}}", matched.as_str()).as_str(), self.uri_params(matched.as_str()));
            }
        }
        uri
    }
    #[inline(always)]
    fn get_api_uri(&mut self, path: &'static str) -> String {
        format!("{}{}", &self.base_url.oanda_host.to_owned(), self.get_uri(path).to_owned())
    }
    #[inline(always)]
    fn get_stream_uri(&mut self, path: &'static str) -> String {
        format!("{}{}", &self.base_url.oanda_stream_host.to_owned(), self.get_uri(path).to_owned())
    }
    pub async fn test(&mut self) -> Result<()> {
        use warp::hyper::body::HttpBody as _;
        use tokio::io::AsyncBufReadExt; // for read_line
        use futures::stream::StreamExt; // for map_err
        let mut request = crate::RequestBuilder::new();
        if !request.headers().is_authorization_exist() {
            request.headers().with_authorization(self.app_key);
        }
        request.query().with_instruments("EUR_USD,USD_CAD");
        let request = request.build(self.get_stream_uri("/v3/accounts/{accountID}/pricing/stream").as_str(), warp::hyper::Method::GET);
        let mut resp = self.client.request(request).await?;

        println!("Response: {}", resp.status());
        let mut body = tokio_util::io::StreamReader::new(resp.into_body().into_stream().map_err(hyper_to_io));

        let mut line = String::new();
        loop {
            line.clear();
            let len = body.read_line(&mut line).await?;
            if len == 0 {
                // Reached end of file.
                break;
            }
            if let Ok(resp) = serde_json::from_slice::<crate::ClientPrice>(&line.as_bytes()) {
                println!("{:#?}", resp);
            }
            if let Ok(resp) = serde_json::from_slice::<crate::PricingHeartbeat>(&line.as_bytes()) {
                println!("{:#?}", resp);
            }else {
                break;
            }
        }
        Ok(())
    }
}

fn hyper_to_io(_hyper: warp::hyper::Error) -> std::io::Error {
    todo!()
}

pub(crate) trait ResponseTrait {
    fn into_json<T>(self) -> BoxFuture<'static, Result<T>> where for<'de> T: serde::de::Deserialize<'de>;
}

impl ResponseTrait for Response<Body> {
    #[inline(always)]
    fn into_json<T>(self) -> BoxFuture<'static, Result<T>> where for<'de> T: serde::de::Deserialize<'de> {
        async move {
            let body_bytes = warp::hyper::body::to_bytes(self.into_body()).await?;
            Ok(serde_json::from_slice::<T>(&body_bytes[..])?)
        }.boxed()
    }
}

// pub(crate) trait ResponseTrait {
//     fn into_json<T>(self) -> Result<T> where for<'de> T: serde::de::Deserialize<'de>;
//     // fn into_reader(self) -> tokio_util::io::StreamReader<MapErr<Body, fn(warp::hyper::Error) -> Error>, Bytes>;
// }
//
// impl ResponseTrait for Response<Body> {
//     #[inline(always)]
//     fn into_json<T>(self) -> Result<T> where for<'de> T: serde::de::Deserialize<'de> {
//         let body_bytes = futures::executor::block_on(warp::hyper::body::to_bytes(self.into_body()))?;
//         Ok(serde_json::from_slice::<T>(&body_bytes[..])?)
//     }
//     // #[inline(always)]
//     // fn into_reader(self) -> tokio_util::io::StreamReader<MapErr<Body, fn(warp::hyper::Error) -> Error>, Bytes>  {
//     //     tokio_util::io::StreamReader::new(self.into_body())
//     //     // let a = warp::hyper::body::aggregate(self.body_mut());
//     //     // let _ = IntoAsyncRead::from(self.body_mut().into_async_read());
//     //     // BufReader::new()
//     //     // loop {
//     //     //     line.clear();
//     //     //     let len = futures::executor::block_on(body.read_line(&mut line)).unwrap();
//     //     //     if len == 0 {
//     //     //         // Reached end of file.
//     //     //         break;
//     //     //     }
//     //     //     futures::executor::block_on(io::stdout().write_all(line.as_bytes()))?;
//     //     //     // let mut de = serde_json::Deserializer::from_reader(line.as_bytes());
//     //     //     // let u = crate::StreamPricingResponse200Body::deserialize(&mut de)?;
//     //     //     // io::stdout().write_all(u).await?;
//     //     //     // responses.push(u);
//     //     //     // let mut rdr = serde_json::from_reader::<R, T>(line.as_bytes());
//     //     //
//     //     //     // let mut rdr = csv::Reader::from_reader(line.as_bytes());
//     //     //     // match rdr {
//     //     //     //     Ok(line) => Ok(line),
//     //     //     //     Err(err) => Err(err.into()),
//     //     //     // break;
//     //     //     // }
//     //     // }
//     //     // Ok(responses)
//     //
//     // }
// }

macro_rules! requests {
  ( $( $func:ident ($path:expr, $method:ident) -> $response:ident ),* ) => {
         impl Client {
             $(
               pub async fn $func( &mut self, mut request: crate::RequestBuilder) -> crate::Result<crate::$response> {
                   if !request.headers().is_authorization_exist() {
                       request.headers().with_authorization(self.app_key);
                   }
                   let request = request.api().build(self.get_api_uri($path).as_str(), warp::hyper::Method::$method);
                   self.client.request(request).await?.into_json::<crate::$response>().await
               }
             )*
         }
    };
}
macro_rules! stream_requests {
  ( $( $func:ident ($path:expr, $method:ident) -> $response:ident ),* ) => {
         impl Client {
             $(
               pub async fn $func( &mut self, mut request: crate::RequestBuilder) -> crate::Result<()> {
                   use warp::hyper::body::HttpBody as _;
                    use tokio::io::{stdout, AsyncWriteExt as _};
                   if !request.headers().is_authorization_exist() {
                       request.headers().with_authorization(self.app_key);
                   }
                   request.query().with_instruments("EUR_USD,USD_CAD");
                   let request = request.stream().build(self.get_stream_uri($path).as_str(), warp::hyper::Method::$method);
                   println!("{:#?}", request);
                   let mut resp = self.client.request(request).await?;
                   println!("Response: {}", resp.status());

                   while let Some(chunk) = resp.body_mut().data().await {
                        stdout().write_all(&chunk?).await?;
                    }
                   Ok(())
                   // let mut line = String::new();
                   // let body = tokio_util::io::StreamReader::new(res.into_body().map_err(hyper_to_io));
                   //
                   // loop {
                   //      line.clear();
                   //      let len = body.read_line(&mut line).await?;
                   //      if len == 0 {
                   //          // Reached end of file.
                   //          break;
                   //      }
                   //      io::stdout().write_all(line.as_bytes()).await?;
                   //      // let mut de = serde_json::Deserializer::from_reader(line.as_bytes());
                   //      // let u = crate::StreamPricingResponse200Body::deserialize(&mut de)?;
                   //      // io::stdout().write_all(u).await?;
                   //      // responses.push(u);
                   //      // let mut rdr = serde_json::from_reader::<R, T>(line.as_bytes());
                   //
                   //      // let mut rdr = csv::Reader::from_reader(line.as_bytes());
                   //      // match rdr {
                   //      //     Ok(line) => Ok(line),
                   //      //     Err(err) => Err(err.into()),
                   //      // break;
                   //      // }
                   //  }
               }
             )*
         }
    };
}

requests!( list_account("/v3/accounts", GET) -> ListAccountsResponse200Body);
stream_requests!( price_stream("/v3/accounts/{accountID}/pricing/stream", GET) -> StreamPricingResponse200Body);

// #[repr(C)]
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct Paths {
//     #[serde(rename = "accountID", skip_serializing_if = "Option::is_none")]
//     account_id: Option<String>,
// }
//
// impl Paths {
//     #[inline(always)]
//     pub fn new() -> Paths {
//         Paths {
//             account_id: None
//         }
//     }
//     #[inline(always)]
//     pub fn add_account_id(&mut self, account_id: String) -> &mut Paths {
//         self.account_id = Some(account_id);
//         self
//     }
//     // #[inline(always)]
//     // pub(crate) fn replace(&self, mut uri: String) -> String {
//     //     let mut json = serde_json::to_string(&self).expect("serialize issue");
//     //     json = json[1..json.len() - 1].to_string();
//     //     if json.len() > 0 {
//     //         for part in json.split(",") {
//     //             uri = match part.split_once(":") {
//     //                 Some((key, value)) => uri.replace(format!("{{{}}}", key), value),
//     //                 None => uri
//     //             };
//     //         }
//     //     }
//     //     uri
//     // }
// }

// pub(crate) fn client2() -> Result<(), Box<dyn std::error::Error>> {
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
//         // We need to manually add the host header because SendRequest does not.header("Host", "/v3/accounts/!/pricing/stream").method("GET").body(Body::from(""))?;
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
