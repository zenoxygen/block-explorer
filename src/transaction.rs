use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Transaction {
    pub txid: String,
    pub hash: String,
    pub version: i32,
    pub size: i64,
    pub vsize: i64,
    pub weight: i64,
    pub locktime: u64,
    pub vin: Vec<Vin>,
    pub vout: Vec<Vout>,
    pub hex: String,
    pub blockhash: String,
    pub confirmations: i64,
    pub time: i64,
    pub blocktime: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Vin {
    pub txid: Option<String>,
    pub vout: Option<i64>,
    pub coinbase: Option<String>,
    pub txinwitness: Option<Vec<String>>,
    pub sequence: Option<u64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Vout {
    pub value: f64,
    pub n: i32,
    #[serde(rename = "scriptPubKey")]
    pub script_pub_key: ScriptPubKey,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ScriptPubKey {
    pub asm: String,
    pub hex: String,
    pub req_sigs: Option<i32>,
    #[serde(rename = "type")]
    pub type_: String,
    pub address: Option<String>,
    pub desc: Option<String>,
}
