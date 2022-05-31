use serde::{Deserialize, Serialize};
use warp::hyper::{Method, Body, Uri, Request};
use warp::hyper::header::{ACCEPT, CONTENT_TYPE, USER_AGENT, AUTHORIZATION, HeaderName, HeaderValue, HeaderMap};
use crate::AcceptDatetimeFormat;

#[repr(C)]
#[derive(Debug)]
pub struct RequestBuilder {
    headers: RequestHeader,
    body: RequestBody,
    query: RequestQuery,
}

impl RequestBuilder {
    #[inline(always)]
    pub fn new() -> Self {
        RequestBuilder {
            headers: RequestHeader::new(),
            body: RequestBody::new(),
            query: RequestQuery::new(),
        }
    }
    #[inline(always)]
    pub(crate) fn set_type(&mut self, is_stream: bool) -> &mut RequestBuilder {
        if !is_stream {
            self.api()
        } else {
            self.stream()
        }
    }
    pub(crate) fn api(&mut self) -> &mut RequestBuilder {
        self.headers().insert(USER_AGENT, "Mozilla/5.0 (Windows NT 6.1; WOW64; rv:40.0) Gecko/20100101 Firefox/40.1".try_into().unwrap()).insert(CONTENT_TYPE, "application/json".try_into().unwrap()).insert(ACCEPT, "application/json".try_into().unwrap());
        self
    }
    pub(crate) fn stream(&mut self) -> &mut RequestBuilder {
        self.headers().insert(USER_AGENT, "Mozilla/5.0 (Windows NT 6.1; WOW64; rv:40.0) Gecko/20100101 Firefox/40.1".try_into().unwrap()).insert(CONTENT_TYPE, "application/octet-stream".try_into().unwrap());
        self
    }
    #[inline(always)]
    pub fn headers(&mut self) -> &mut RequestHeader {
        &mut self.headers
    }
    #[inline(always)]
    pub fn body(&mut self) -> &mut RequestBody {
        &mut self.body
    }
    #[inline(always)]
    pub fn query(&mut self) -> &mut RequestQuery {
        &mut self.query
    }
    #[inline(always)]
    pub(crate) fn build(&mut self, uri: &str, method: Method) -> Request<Body> {
        let mut request = Request::new(self.body().build());
        *request.method_mut() = method.to_owned();
        *request.headers_mut() = self.headers().build();
        *request.uri_mut() = self.query().build(uri);
        request
    }
}

#[repr(C)]
#[derive(Debug, Serialize)]
pub struct RequestBody {}

impl RequestBody {
    #[inline(always)]
    pub(crate) fn new() -> RequestBody {
        RequestBody {}
    }
    #[inline(always)]
    fn to_vec(&self) -> Option<Vec<u8>> {
        match serde_json::to_vec(&self) {
            Ok(response) => { if response.len() > 2 { Some(response) } else { None } },
            Err(_) => None
        }
    }
    #[inline(always)]
    pub(crate) fn build(&self) -> Body {
        if let Some(body) = self.to_vec() {
            Body::from(body)
        } else {
            Body::empty()
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct RequestHeader {
    map: HeaderMap,
}

impl RequestHeader {
    #[inline(always)]
    pub(crate) fn new() -> RequestHeader {
        RequestHeader {
            map: HeaderMap::new()
        }
    }
    #[inline(always)]
    pub(crate) fn insert(&mut self, key: HeaderName, value: HeaderValue) -> &mut RequestHeader {
        self.map.entry(key).or_insert(value);
        self
    }
    #[inline(always)]
    pub fn insert_str(&mut self, key: &str, value: &str) -> &mut RequestHeader {
        self.map.entry(key.parse::<HeaderName>().unwrap()).or_insert(value.try_into().unwrap());
        self
    }
    #[allow(unused)]
    #[inline(always)]
    pub fn with_authorization(&mut self, value: &'static str) -> &mut RequestHeader {
        self.insert(AUTHORIZATION, format!("Bearer {}", value).try_into().unwrap());
        self
    }
    #[inline(always)]
    pub fn is_authorization_exist(&self) -> bool {
        match self.map.get(AUTHORIZATION) {
            None => false,
            Some(_) => true
        }
    }
    #[allow(unused)]
    #[inline(always)]
    pub fn with_accept_datetime_format(&mut self, value: &AcceptDatetimeFormat) -> &mut RequestHeader {
        self.insert_str("AcceptDatetimeFormat", serde_json::to_string(value).unwrap().as_str());
        self
    }
    #[inline(always)]
    pub(crate) fn build(&self) -> HeaderMap {
        self.map.to_owned()
    }
}

#[repr(C)]
#[derive(Debug, Serialize)]
pub struct RequestQuery {
    #[serde(rename = "instruments", skip_serializing_if = "Option::is_none")]
    instruments: Option<String>,
    #[serde(rename = "snapshot", skip_serializing_if = "Option::is_none")]
    snapshot: Option<bool>,
}

impl RequestQuery {
    #[inline(always)]
    pub(crate) fn new() -> RequestQuery {
        RequestQuery {
            instruments: None,
            snapshot: None,
        }
    }
    #[inline(always)]
    pub fn with_instruments(&mut self, x: &str) -> &mut RequestQuery {
        self.instruments = Some(x.into());
        self
    }
    #[inline(always)]
    pub fn with_snapshot(&mut self, x: bool) -> &mut RequestQuery {
        self.snapshot = Some(x);
        self
    }
    #[inline(always)]
    fn to_string(&self) -> Option<String> {
        match serde_urlencoded::to_string(&self) {
            Ok(response) => { if response.len() > 0 { Some(format!("?{}", response)) } else { None } },
            Err(_) => None
        }
    }
    #[inline(always)]
    pub(crate) fn build(&self, uri: &str) -> Uri {
        if let Some(query) = &self.to_string() {
            format!("{}{}", uri, &query).parse().unwrap()
        } else {
            // format!("{}", uri).parse::<Uri>().unwrap()
            uri.parse().unwrap()
        }
    }
}
