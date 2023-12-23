use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct NetworkInfo {
    version: i32,
    subversion: String,
    protocolversion: i32,
    localservices: String,
    localservicesnames: Vec<String>,
    localrelay: bool,
    timeoffset: i32,
    networkactive: bool,
    connections: i32,
    connections_in: i32,
    connections_out: i32,
    networks: Vec<Network>,
    relayfee: f64,
    incrementalfee: f64,
    localaddresses: Vec<Address>,
    warnings: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Network {
    name: String,
    limited: bool,
    reachable: bool,
    proxy: String,
    proxy_randomize_credentials: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Address {
    address: String,
    port: i32,
    score: i32,
}
