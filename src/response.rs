use crate::Pair;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Type alias for `Result<T, ClientError>`
pub type Result<T = (), E = ClientError> = std::result::Result<T, E>;

/// A Dexscreener error.
#[derive(Debug, Error)]
pub enum ClientError {
    #[error("Invalid address: {0}")]
    InvalidAddress(String),

    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    #[error(transparent)]
    Url(#[from] url::ParseError),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PairResponse {
    // #[serde(rename = "schemaVersion")]
    // pub schema_version: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pairs: Option<Vec<Pair>>,
}
