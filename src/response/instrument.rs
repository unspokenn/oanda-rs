use serde::{Deserialize, Serialize};
use crate::Instrument;

/// The list of tradeable instruments for the Account has been provided.
#[repr(C)]
#[derive(Debug, Serialize, Deserialize)]
pub struct GetAccountInstrumentsResponse200Header {
    #[serde(rename = "RequestID", skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// The list of tradeable instruments for the Account has been provided.
#[repr(C)]
#[derive(Debug, Serialize, Deserialize)]
pub struct GetAccountInstrumentsResponse200Body {
    #[serde(rename = "instruments", skip_serializing_if = "Option::is_none")]
    pub instruments: Option<Vec<Instrument>>,
    #[serde(rename = "lastTransactionID", skip_serializing_if = "Option::is_none")]
    pub last_transaction_id: Option<String>,
}

impl GetAccountInstrumentsResponse200Body {
    pub fn name_join_string(&self) -> crate::Result<String> {
        if let Some(ref instruments) = self.instruments {
            return Ok(instruments.iter().map(|instrument| instrument.name.to_owned().unwrap()).collect::<Vec<String>>().join(","));
        }
        Err(std::io::Error::new(std::io::ErrorKind::Other, "instrument response error!").into())
    }
}
