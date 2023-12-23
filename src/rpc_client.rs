use anyhow::{anyhow, Result};
use rocket::serde::{
    json::{json, serde_json},
    Deserialize, Serialize,
};

use crate::address::ValidateAddress;
use crate::block::Block;
use crate::blockchain::BlockchainInfo;
use crate::blockstats::BlockStats;
use crate::network::NetworkInfo;
use crate::transaction::Transaction;

pub struct RpcClient {
    client: reqwest::Client,
    rpc_url: String,
    rpc_user: String,
    rpc_password: String,
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(crate = "rocket::serde")]
pub enum VerbosityLevel {
    HexDecoded = 1,
    Detailed = 2,
}

impl VerbosityLevel {
    pub fn as_int(&self) -> i32 {
        *self as i32
    }
}

impl RpcClient {
    pub async fn new(rpc_url: String, rpc_user: String, rpc_password: String) -> Result<Self> {
        let client = RpcClient {
            client: reqwest::Client::new(),
            rpc_url,
            rpc_user,
            rpc_password,
        };

        Ok(client)
    }

    async fn rpc_call<T: for<'de> Deserialize<'de>>(
        &self,
        method: &str,
        params: &[serde_json::Value],
    ) -> Result<T> {
        let req_body = json!({
            "jsonrpc": "2.0",
            "id": "bitcoin-explorer",
            "method": method,
            "params": params,
        });

        let response = self
            .client
            .post(&self.rpc_url)
            .basic_auth(&self.rpc_user, Some(&self.rpc_password))
            .json(&req_body)
            .send()
            .await?;

        let resp_body = response.json::<serde_json::Value>().await?;

        let result = serde_json::from_value(resp_body["result"].clone())?;

        Ok(result)
    }

    async fn batch_rpc_call(
        &self,
        batch: Vec<(&str, Vec<serde_json::Value>)>,
    ) -> Result<Vec<serde_json::Value>> {
        let req_body: Vec<_> = batch
            .into_iter()
            .enumerate()
            .map(|(id, (method, params))| {
                json!({
                    "jsonrpc": "2.0",
                    "id": id.to_string(),
                    "method": method,
                    "params": params,
                })
            })
            .collect();

        let response = self
            .client
            .post(&self.rpc_url)
            .basic_auth(&self.rpc_user, Some(&self.rpc_password))
            .json(&req_body)
            .send()
            .await?;

        let resp_body: Vec<serde_json::Value> = response.json().await?;

        let mut results = Vec::new();

        for entry in resp_body {
            if entry
                .get("error")
                .and_then(serde_json::Value::as_object)
                .is_some()
            {
                return Err(anyhow!("error in batch response: {:?}", entry));
            }
            results.push(entry["result"].clone());
        }

        Ok(results)
    }

    pub async fn get_network_info(&self) -> Result<NetworkInfo> {
        self.rpc_call("getnetworkinfo", &[]).await
    }

    pub async fn get_blockchain_info(&self) -> Result<BlockchainInfo> {
        self.rpc_call("getblockchaininfo", &[]).await
    }

    pub async fn get_block_hash(&self, height: i64) -> Result<String> {
        self.rpc_call("getblockhash", &[json!(height)]).await
    }

    pub async fn get_block(&self, hash: &str) -> Result<Block> {
        self.rpc_call("getblock", &[json!(hash)]).await
    }

    pub async fn get_block_stats(&self, height: i64) -> Result<BlockStats> {
        self.rpc_call("getblockstats", &[json!(height)]).await
    }

    pub async fn get_raw_transaction(&self, txid: &str) -> Result<Transaction> {
        self.rpc_call(
            "getrawtransaction",
            &[json!(txid), json!(VerbosityLevel::Detailed.as_int())],
        )
        .await
    }

    pub async fn get_raw_transactions(&self, txids: &[String]) -> Result<Vec<Transaction>> {
        let batch = txids
            .iter()
            .map(|txid| {
                (
                    "getrawtransaction",
                    vec![json!(txid), json!(VerbosityLevel::HexDecoded.as_int())],
                )
            })
            .collect::<Vec<_>>();

        let results = self.batch_rpc_call(batch).await?;

        let transactions = results
            .into_iter()
            .map(|result| {
                serde_json::from_value::<Transaction>(result)
                    .map_err(|e| anyhow!("failed to parse transaction: {e}"))
            })
            .collect::<Result<Vec<Transaction>>>()?;

        Ok(transactions)
    }

    pub async fn validate_address(&self, address: &str) -> Result<ValidateAddress> {
        self.rpc_call("validateaddress", &[json!(address)]).await
    }
}
