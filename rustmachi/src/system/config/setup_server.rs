use std::error::Error;
use std::net::Ipv4Addr;
use serde::Deserialize;
use std::fs;
#[derive(Deserialize)]
pub struct ServerJSON{
    virtual_ip: Ipv4Addr,
    port: u16,
    identifier: String
}
struct Setup{
    #[serde(rename="Server")]
    server:ServerJSON
}

impl ServerJSON{
    pub fn new(ipv4:u32,port:u16,identifier:String){
        Self{
            ipv4,
            port,
            identifier
        }
    }
    pub fn ipv4(&self) -> std::net::Ipv4Addr {
        self.ipv4
    }
    pub fn port(&self) -> u16 {
        self.port
    }
    pub fn identifier(&self) -> &str {
        &self.identifier
    }
}

pub trait JsonLOAD{
    fn setup_server()->Result<ServerJSON, Box<dyn Error>>{
        let json_content = fs::read_to_string("setup.json")?;
        let setup: Setup = serde_json::from_str(&json_content)?;

        Ok(setup.server)
    }
}

impl JsonLOAD for ServerJSON{}