#[macro_use]
extern crate rocket;

mod address;
mod block;
mod blockchain;
mod blockstats;
mod network;
mod routes;
mod rpc_client;
mod transaction;
mod utils;

use anyhow::{anyhow, Result};
use rocket::{
    fs::{relative, FileServer},
    serde::Deserialize,
};
use rocket_dyn_templates::{Engines, Template};

use crate::rpc_client::RpcClient;
use crate::utils::{FormatHashFilter, FormatNumberFilter, FormatTimeFilter};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct Config {
    rpc_url: String,
    rpc_user: String,
    rpc_password: String,
}

#[rocket::main]
async fn main() -> Result<()> {
    let figment = rocket::Config::figment();

    let config: Config = figment
        .extract()
        .map_err(|e| anyhow!("failed to parse configuration: {e}"))?;

    let rpc_client = RpcClient::new(config.rpc_url, config.rpc_user, config.rpc_password).await?;

    let template = Template::custom(|engines: &mut Engines| {
        engines
            .tera
            .register_filter("format_hash", FormatHashFilter);
        engines
            .tera
            .register_filter("format_number", FormatNumberFilter);
        engines
            .tera
            .register_filter("format_time", FormatTimeFilter);
    });

    let rocket = rocket::build();

    rocket
        .attach(template)
        .manage(rpc_client)
        .mount("/", FileServer::from(relative!("/public")))
        .mount(
            "/",
            routes![
                routes::about,
                routes::index,
                routes::view_block,
                routes::view_transaction,
                routes::view_address,
                routes::search
            ],
        )
        .register(
            "/",
            catchers![routes::not_found, routes::internal_server_error],
        )
        .launch()
        .await
        .map_err(|e| anyhow!("rocket failed to launch: {e}"))?;

    Ok(())
}
