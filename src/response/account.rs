use serde::{Deserialize, Serialize};
use crate::AccountProperties;

#[repr(C)]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ListAccountsResponse200Header {
    #[serde(rename = "RequestID", skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

#[repr(C)]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ListAccountsResponse200Body {
    #[serde(rename = "accounts", skip_serializing_if = "Option::is_none")]
    pub accounts: Option<Vec<AccountProperties>>,
}

impl ListAccountsResponse200Body {
    pub fn get_id(&self, account_index: usize) -> crate::Result<String> {
        if let Some(ref accounts) = self.accounts {
            if let Some(ref account) = accounts.get(account_index) {
                if let Some(ref id) = account.id {
                    return Ok(id.to_owned())
                }
            }
        }
        Err(std::io::Error::new(std::io::ErrorKind::Other, "account response error!").into())
    }
}
