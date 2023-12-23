use anyhow::Result;
use rocket::{response::Redirect, serde::json::json, State};
use rocket_dyn_templates::Template;

use crate::rpc_client::RpcClient;

#[catch(404)]
pub async fn not_found() -> Result<Template, Redirect> {
    Ok(Template::render("404", json!({})))
}

#[catch(500)]
pub async fn internal_server_error() -> Result<Template, Redirect> {
    Ok(Template::render("500", json!({})))
}

#[get("/")]
pub async fn index(rpc_client: &State<RpcClient>) -> Result<Template, Redirect> {
    let blockchain_info = rpc_client.get_blockchain_info().await.map_err(|e| {
        eprintln!("failed to get blockchain info: {e}");
        Redirect::to("/")
    })?;

    let current_height = blockchain_info.blocks;
    let mut blocks = Vec::new();

    for height in (current_height - 5..=current_height).rev() {
        let block_hash = rpc_client.get_block_hash(height).await.map_err(|e| {
            eprintln!("failed to get block hash: {e}");
            Redirect::to("/")
        })?;

        let block = rpc_client.get_block(&block_hash).await.map_err(|e| {
            eprintln!("failed to get block: {e}");
            Redirect::to("/")
        })?;

        blocks.push(block);
    }

    Ok(Template::render("index", json!({"blocks": blocks})))
}

#[get("/about")]
pub async fn about(rpc_client: &State<RpcClient>) -> Result<Template, Redirect> {
    let network_info = rpc_client.get_network_info().await.map_err(|e| {
        eprintln!("failed to get network info: {e}");
        Redirect::to("/")
    })?;

    let blockchain_info = rpc_client.get_blockchain_info().await.map_err(|e| {
        eprintln!("failed to get blockchain info: {e}");
        Redirect::to("/")
    })?;

    Ok(Template::render(
        "about",
        json!({
            "blockchain_info": blockchain_info,
            "network_info": network_info
        }),
    ))
}

#[get("/block/<block_hash>")]
pub async fn view_block(
    rpc_client: &State<RpcClient>,
    block_hash: &str,
) -> Result<Template, Redirect> {
    let block = rpc_client.get_block(block_hash).await.map_err(|e| {
        eprintln!("failed to get block: {e}");
        Redirect::to("/")
    })?;

    let block_stats = rpc_client
        .get_block_stats(block.height)
        .await
        .map_err(|e| {
            eprintln!("failed to get block stats: {e}");
            Redirect::to("/")
        })?;

    Ok(Template::render(
        "block",
        json!({
            "block": block,
            "block_stats": block_stats,
        }),
    ))
}

#[get("/tx/<tx_id>")]
pub async fn view_transaction(
    rpc_client: &State<RpcClient>,
    tx_id: &str,
) -> Result<Template, Redirect> {
    let tx = rpc_client.get_raw_transaction(tx_id).await.map_err(|e| {
        eprintln!("failed to get raw transaction: {e}");
        Redirect::to("/")
    })?;

    let is_coinbase = tx.vin.first().map_or(false, |vin| vin.coinbase.is_some());

    let mut total_input_value = 0.0;
    let total_output_value: f64 = tx.vout.iter().map(|vout| vout.value).sum();

    let mut tx_fee = 0.0;
    let mut tx_fee_rate = 0.0;

    let mut input_addresses = Vec::new();

    if !is_coinbase {
        // Extract the transaction IDs from the input vector of the transaction
        let input_txids: Vec<String> = tx.vin.iter().filter_map(|vin| vin.txid.clone()).collect();

        // Fetch details of input transactions using their txids
        let input_txs = rpc_client
            .get_raw_transactions(&input_txids)
            .await
            .map_err(|e| {
                eprintln!("failed to get raw transactions: {e}");
                Redirect::to("/")
            })?;

        // Iterate over each input and its corresponding transaction
        for (vin, input_tx) in tx.vin.iter().zip(input_txs.iter()) {
            let vout_index = vin.vout.unwrap_or(0) as usize;

            // Extract the address from the output of the input transaction
            let address = input_tx
                .vout
                .get(vout_index)
                .and_then(|vout| vout.script_pub_key.address.clone());

            // Store the extracted address
            input_addresses.push(address);

            // Sum up the total input value from these transactions
            total_input_value += input_tx
                .vout
                .get(vout_index)
                .map(|vout_entry| vout_entry.value)
                .unwrap_or(0.0);
        }

        // Calculate the transaction fee
        tx_fee = total_input_value - total_output_value;

        // Calculate the transaction fee rate
        tx_fee_rate = if tx.vsize > 0 {
            tx_fee / (tx.vsize as f64)
        } else {
            0.0
        };
    }

    Ok(Template::render(
        "transaction",
        json!({
            "tx": tx,
            "tx_fee": tx_fee,
            "tx_fee_rate": tx_fee_rate,
            "total_input_value": total_input_value,
            "total_output_value": total_output_value,
            "input_addresses": input_addresses,
        }),
    ))
}

#[get("/address/<address>")]
pub async fn view_address(
    rpc_client: &State<RpcClient>,
    address: &str,
) -> Result<Template, Redirect> {
    let address_info = rpc_client.validate_address(address).await.map_err(|e| {
        eprintln!("failed to validate address: {e}");
        Redirect::to("/")
    })?;

    if !address_info.isvalid {
        return Err(Redirect::to("/"));
    }

    Ok(Template::render(
        "address",
        json!({
            "address_info": address_info,
        }),
    ))
}

#[get("/search?<query>")]
pub async fn search(rpc_client: &State<RpcClient>, query: String) -> Result<Redirect, Template> {
    if let Ok(height) = query.parse::<i64>() {
        if let Ok(block_hash) = rpc_client.get_block_hash(height).await {
            return Ok(Redirect::to(uri!(view_block(block_hash))));
        }
    }

    if query.len() == 64 {
        if rpc_client.get_block(&query).await.is_ok() {
            return Ok(Redirect::to(uri!(view_block(query))));
        } else if rpc_client.get_raw_transaction(&query).await.is_ok() {
            return Ok(Redirect::to(uri!(view_transaction(query))));
        }
    }

    Ok(Redirect::to(uri!(view_address(query))))
}
