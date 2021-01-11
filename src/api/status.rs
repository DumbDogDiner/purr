use serde::Deserialize;
use reqwest;

use crate::cfg;

#[derive(Debug, Deserialize)]
pub struct ServerStatus {}

pub fn fetch_server_status() -> Result<ServerStatus, ()> {
    let api_endpoint = cfg::get_config().api_endpoint;
    let resp = match reqwest::blocking::get(format!("{}/client/", api_endpoint).as_str()) {
        Ok(resp) => resp.json::<ServerStatus>(),
        Err(_) => panic!("failed to fetch server status"),
    };
    Ok(resp.expect("failed to fetch server status"))
}
