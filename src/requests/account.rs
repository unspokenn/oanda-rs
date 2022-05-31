impl ListAccountsRequest {
    pub fn new() -> ListAccountsRequest {
        ListAccountsRequest {
            request: Request::new(),
            headers: RequestHead::new(),
            body: RequestBody::new(),
            query: RequestQuery::new("/v3/accounts", "GET"),
        }
    }

    pub fn with_authorization(mut self, x: String) -> ListAccountsRequest {
        self.headers = self.headers.with_authorization( x);
        self
    }

    pub fn build(mut self, client: Client) -> Result<warp::hyper::Request<warp::hyper::Body>, Box<dyn Error>> {
        self.request.query(&self.query.replace_uri(&client.paths())).headers(&self.headers).build(&self.body)?
    }

    pub fn remote(self, client: Client) -> Result<ListAccountsResponse, Box<dyn Error>> {
        let request = self.build(client).method("GET").body(Body::from(&self.body))?;
        let request = self.build_request(, "GET");
        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, hyper::Body>(https);
        let response = client.request(request).await?;


        let response = request_sender.send_request(request).await?;


        let res = client.reqwest.get(&url).query(&self.query).bearer_auth(&client.authentication).send();
        match res {
            Err(e) => Err(Box::new(e)),
            Ok(response) => match response.json::<ListAccountsResponse>() {
                Err(e) => Err(Box::new(e)),
                Ok(j) => Ok(j),
            },
        }
    }
}


pub mod list_accounts {
    use crate::{Request, Client};

    use serde::{Deserialize, Serialize};
    use std::error::Error;
    use warp::hyper::Body;
    use crate::models::{AccountProperties, RequestBody, RequestHead, RequestQuery};



    pub type ListAccountsResponse = ListAccountsResponse200Body;

    #[repr(C)]
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ListAccountsResponse200Header {
        #[serde(rename = "RequestID", skip_serializing_if = "Option::is_none")]
        pub request_id: Option<String>,
    }

    #[repr(C)]
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ListAccountsResponse200Body {
        #[serde(rename = "accounts", skip_serializing_if = "Option::is_none")]
        pub accounts: Option<Vec<AccountProperties>>,
    }

    impl ListAccountsResponse200Body {
        pub fn get_id(self) -> String {
            let accounts = self.accounts.expect("Did not find 'accounts' field in response");
            let account = accounts.get(0).expect("Did not find an 'account' in the 'accounts' field");
            return account.id.expect("Did not find an 'id' field in the 'account'").to_string();
        }
    }
}

pub mod get_account {
    use crate::Client;

    use serde::{Deserialize, Serialize};
    use std::error::Error;
    use crate::models::Account;

    #[derive(Debug, Serialize, Deserialize)]
    struct RequestHead {
        #[serde(rename = "Authorization", skip_serializing_if = "Option::is_none")]
        pub authorization: Option<String>,

        #[serde(rename = "AcceptDatetimeFormat", skip_serializing_if = "Option::is_none")]
        pub accept_datetime_format: Option<String>,
    }

    impl RequestHead {
        fn new() -> RequestHead {
            RequestHead {
                authorization: None,
                accept_datetime_format: None,
            }
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct RequestPath {
        #[serde(rename = "accountID", skip_serializing_if = "Option::is_none")]
        pub account_id: Option<String>,
    }

    impl RequestPath {
        fn new() -> RequestPath {
            RequestPath { account_id: None }
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct RequestBody {}

    impl RequestBody {
        fn new() -> RequestBody {
            RequestBody {}
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct RequestQuery {}

    impl RequestQuery {
        fn new() -> RequestQuery {
            RequestQuery {}
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct GetAccountRequest {
        #[serde(skip_serializing)]
        uri: String,
        header: RequestHead,
        body: RequestBody,
        path: RequestPath,
        query: RequestQuery,
    }

    impl GetAccountRequest {
        pub fn new() -> GetAccountRequest {
            GetAccountRequest {
                uri: String::from("/v3/accounts/{accountID}"),
                header: RequestHead::new(),
                body: RequestBody::new(),
                path: RequestPath::new(),
                query: RequestQuery::new(),
            }
        }
        pub fn with_uri(mut self, x: String) -> Self {
            self.uri = x;
            self
        }

        pub fn with_account_id(mut self, x: String) -> Self {
            self.path.account_id = Some(x);
            self
        }

        pub fn with_authorization(mut self, x: String) -> Self {
            self.header.authorization = Some(x);
            self
        }

        pub fn with_accept_datetime_format(mut self, x: String) -> Self {
            self.header.accept_datetime_format = Some(x);
            self
        }

        pub fn remote(self, client: &Client) -> Result<GetAccountResponse, Box<dyn Error>> {
            let uri = self.uri.clone().replace("{accountID}", &self.path.account_id.unwrap());
            let url = format!("https://{host}{uri}", host = client.host, uri = uri);
            let res = client.reqwest.get(&url).query(&self.query).bearer_auth(&client.authentication).send();
            match res {
                Err(e) => Err(Box::new(e)),
                Ok(response) => match response.json::<GetAccountResponse>() {
                    Err(e) => Err(Box::new(e)),
                    Ok(j) => Ok(j),
                },
            }
        }
    }

    pub type GetAccountResponse = GetAccountResponse200Body;

    #[derive(Debug, Serialize, Deserialize)]
    pub struct GetAccountResponse200Header {
        #[serde(rename = "RequestID", skip_serializing_if = "Option::is_none")]
        pub request_id: Option<String>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct GetAccountResponse200Body {
        #[serde(rename = "account", skip_serializing_if = "Option::is_none")]
        pub account: Option<Account>,
        #[serde(rename = "lastTransactionID", skip_serializing_if = "Option::is_none")]
        pub last_transaction_id: Option<String>,
    }
}

pub mod stream_pricing {
    use crate::Client;

    use serde::{Deserialize, Serialize};
    use std::error::Error;
    use crate::models::{ClientPrice, PricingHeartbeat};

    #[derive(Debug, Serialize, Deserialize)]
    struct RequestHead {
        #[serde(rename = "Authorization", skip_serializing_if = "Option::is_none")]
        pub authorization: Option<String>,

        #[serde(rename = "AcceptDatetimeFormat", skip_serializing_if = "Option::is_none")]
        pub accept_datetime_format: Option<String>,
    }

    impl RequestHead {
        fn new() -> RequestHead {
            RequestHead {
                authorization: None,
                accept_datetime_format: None,
            }
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct RequestPath {
        #[serde(rename = "accountID", skip_serializing_if = "Option::is_none")]
        pub account_id: Option<String>,
    }

    impl RequestPath {
        fn new() -> RequestPath {
            RequestPath { account_id: None }
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct RequestBody {}

    impl RequestBody {
        fn new() -> RequestBody {
            RequestBody {}
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct RequestQuery {
        #[serde(rename = "instruments", skip_serializing_if = "Option::is_none")]
        pub instruments: Option<Vec<String>>,

        #[serde(rename = "snapshot", skip_serializing_if = "Option::is_none")]
        pub snapshot: Option<bool>,
    }

    impl RequestQuery {
        fn new() -> RequestQuery {
            RequestQuery {
                instruments: None,
                snapshot: None,
            }
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct StreamPricingRequest {
        #[serde(skip_serializing)]
        uri: String,
        header: RequestHead,
        body: RequestBody,
        path: RequestPath,
        query: RequestQuery,
    }

    impl StreamPricingRequest {
        pub fn new() -> StreamPricingRequest {
            StreamPricingRequest {
                uri: String::from("/v3/accounts/{accountID}/pricing/stream"),
                header: RequestHead::new(),
                body: RequestBody::new(),
                path: RequestPath::new(),
                query: RequestQuery::new(),
            }
        }
        pub fn with_uri(mut self, x: String) -> Self {
            self.uri = x;
            self
        }

        pub fn with_account_id(mut self, x: String) -> Self {
            self.path.account_id = Some(x);
            self
        }

        pub fn with_authorization(mut self, x: String) -> Self {
            self.header.authorization = Some(x);
            self
        }

        pub fn with_accept_datetime_format(mut self, x: String) -> Self {
            self.header.accept_datetime_format = Some(x);
            self
        }

        pub fn with_instruments(mut self, x: Vec<String>) -> Self {
            self.query.instruments = Some(x);
            self
        }

        pub fn with_snapshot(mut self, x: bool) -> Self {
            self.query.snapshot = Some(x);
            self
        }

        pub fn remote(self, client: &Client) -> Result<StreamPricingResponse, Box<dyn Error>> {
            let uri = self.uri.clone().replace("{accountID}", &self.path.account_id.unwrap());
            let url = format!("https://{host}{uri}", host = client.host, uri = uri);
            let res = client.reqwest.get(&url).query(&self.query).bearer_auth(&client.authentication).send();
            match res {
                Err(e) => Err(Box::new(e)),
                Ok(response) => match response.json::<StreamPricingResponse>() {
                    Err(e) => Err(Box::new(e)),
                    Ok(j) => Ok(j),
                },
            }
        }
    }


}



