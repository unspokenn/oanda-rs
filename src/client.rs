use futures::{future::BoxFuture, FutureExt, TryStreamExt};
use crate::Result;

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
    pub client: warp::hyper::client::Client<hyper_tls::HttpsConnector<warp::hyper::client::HttpConnector>>,
    uri_params: hashbrown::HashMap<&'static str, String>,
    app_key: &'static str
}

impl Client {
    pub fn new(app_key: &'static str, demo: bool) -> Client {
        let mut connector = warp::hyper::client::HttpConnector::new();
        connector.set_keepalive(Some(std::time::Duration::from_secs(90)));
        connector.set_reuse_address(true);
        connector.set_connect_timeout(Some(std::time::Duration::from_secs(5)));
        connector.set_nodelay(true);
        connector.enforce_http(false);
        Client {
            base_url: BaseUrl::new(demo),
            client: warp::hyper::client::Client::builder().build::<_, warp::hyper::Body>(hyper_tls::HttpsConnector::new_with_connector(connector)),
            uri_params: hashbrown::HashMap::new(),
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
        let exp = regex::Regex::new(r"\{\{|}}|\{([^}]+)}").unwrap();
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
    pub async fn price_stream(&mut self, mut request: crate::RequestBuilder) -> tokio_util::io::StreamReader<futures::stream::MapErr<futures::stream::IntoStream<warp::hyper::Body>, fn(warp::hyper::Error) -> std::io::Error>, warp::hyper::body::Bytes> {
        use warp::hyper::body::HttpBody as _;
        if !request.headers().is_authorization_exist() {
            request.headers().with_authorization(self.app_key);
        }
        let request = request.build(self.get_stream_uri("/v3/accounts/{accountID}/pricing/stream").as_str(), warp::hyper::Method::GET);
        let resp = self.client.request(request).await.unwrap();

        tokio_util::io::StreamReader::new(resp.into_body().into_stream().map_err(hyper_to_io))
    }
}

fn hyper_to_io(_hyper: warp::hyper::Error) -> std::io::Error {
    todo!()
}

pub(crate) trait ResponseTrait {
    fn into_json<T>(self) -> BoxFuture<'static, Result<T>> where for<'de> T: serde::de::Deserialize<'de>;
}

impl ResponseTrait for warp::hyper::Response<warp::hyper::Body> {
    #[inline(always)]
    fn into_json<T>(self) -> BoxFuture<'static, Result<T>> where for<'de> T: serde::de::Deserialize<'de> {
        async move {
            let body_bytes = warp::hyper::body::to_bytes(self.into_body()).await?;
            Ok(serde_json::from_slice::<T>(&body_bytes[..])?)
        }.boxed()
    }
}

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

requests!( list_account("/v3/accounts", GET) -> ListAccountsResponse200Body,
           get_instruments("/v3/accounts/{accountID}/instruments", GET) -> GetAccountInstrumentsResponse200Body);
