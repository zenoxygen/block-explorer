use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Block {
    pub hash: String,
    pub confirmations: i64,
    pub height: i64,
    pub version: i32,
    #[serde(rename = "versionHex")]
    pub version_hex: String,
    pub merkleroot: String,
    pub time: u64,
    pub mediantime: u64,
    pub nonce: u64,
    pub bits: String,
    pub difficulty: f64,
    pub chainwork: String,
    #[serde(rename = "nTx")]
    pub n_tx: i64,
    pub previousblockhash: Option<String>,
    pub nextblockhash: Option<String>,
    pub strippedsize: i64,
    pub size: i64,
    pub weight: i64,
    pub tx: Vec<String>,
}
