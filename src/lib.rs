#![crate_type = "lib"]

mod serdes;
mod client;
mod models;
mod request;
mod response;

pub(crate) use serdes::*;
pub use models::*;
pub use request::*;
pub use response::*;
pub use client::Client;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
