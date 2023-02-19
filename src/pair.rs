use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pair {
    pub chain_id: String,
    pub dex_id: String,
    pub url: String,
    #[serde(default)]
    pub labels: Vec<String>,

    pub pair_address: String,
    pub base_token: Token,
    pub quote_token: Token,

    pub price_native: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub price_usd: Option<String>,
    pub txns: Timed<Transactions>,

    pub volume: Timed<f64>,
    pub price_change: Timed<f64>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub liquidity: Option<Liquidity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fdv: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pair_created_at: Option<u64>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Token {
    pub address: Option<String>,
    pub name: Option<String>,
    pub symbol: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Transactions {
    pub buys: u64,
    pub sells: u64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Liquidity {
    pub usd: f64,
    pub base: f64,
    pub quote: f64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Timed<T> {
    pub m5: T,
    pub h1: T,
    pub h6: T,
    pub h24: T,
}

// interface Pair {
//   chainId: string;
//   dexId: string;
//   url: string;
//   pairAddress: string;
//   baseToken: {
//     address: string;
//     name: string;
//     symbol: string;
//   };
//   quoteToken: {
//     symbol: string;
//   };
//   priceNative: string;
//   priceUsd?: string;
//   txns: {
//     m5: {
//       buys: number;
//       sells: number;
//     };
//     h1: {
//       buys: number;
//       sells: number;
//     };
//     h6: {
//       buys: number;
//       sells: number;
//     };
//     h24: {
//       buys: number;
//       sells: number;
//     };
//   };
//   volume: {
//     m5: number;
//     h1: number;
//     h6: number;
//     h24: number;
//   };
//   priceChange: {
//     m5: number;
//     h1: number;
//     h6: number;
//     h24: number;
//   };
//   liquidity?: {
//     usd?: number;
//     base: number;
//     quote: number;
//   };
//   fdv?: number;
//   pairCreatedAt?: number;
// }
