use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct ValidateAddress {
    pub isvalid: bool,
    pub address: String,
    #[serde(rename = "scriptPubKey")]
    pub script_pub_key: String,
    pub isscript: bool,
    pub iswitness: bool,
}
