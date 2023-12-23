use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct BlockchainInfo {
    pub chain: String,
    pub blocks: i64,
    pub headers: i64,
    pub bestblockhash: String,
    pub difficulty: f64,
    pub time: i64,
    pub mediantime: i64,
    pub verificationprogress: f64,
    pub initialblockdownload: bool,
    pub chainwork: String,
    pub size_on_disk: i64,
    pub pruned: bool,
    pub warnings: String,
}
