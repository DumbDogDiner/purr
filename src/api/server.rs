use std::error::Error;
use serde::{Deserialize, Debug}

#[derive(Deserialize, Debug)]
pub struct SftpDetails {
    pub ip: String,
    pub port: u16
}

#[derive(Deserialize, Debug)]
pub struct Server {
    pub server_owner: bool,
    pub identifier: String,
    pub uuid: String,
    pub name: String,
    pub node: String,
    pub sftp_details: &SftpDetails,
    pub description: String
}

impl Server {
    fn start() -> Result<(), Error> {
        Ok(())
    }
    
    fn stop() -> Result<(), Error> {
        Ok(())
    }

    fn restart() -> Result<(), Error> {
        Ok(())
    }
}

pub fn get_servers() -> Result<Vec<Server>, Error> {
    
}

// Fetches a server by its name.
pub fn get_server_by_name(name: &str) -> Result<Server, Error> {
    Ok(Server {})
}
