use std::error::Error;
use std::net::Ipv4Addr;
use serde::Deserialize;
use std::fs;
#[derive(Deserialize,Debug)]
pub struct ClientJSON{
    virtual_ip: Ipv4Addr,
    port: u16,
    identifier: String
}
#[derive(Deserialize)]
struct Setup{
    #[serde(rename="Client")]
    client: ClientJSON
}

impl ClientJSON{
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
    fn setup_client()->Result<ClientJSON, Box<dyn Error>>{
        let json_content = fs::read_to_string("setup.json")?;
        let setup: Setup = serde_json::from_str(&json_content)?;
        Ok(setup.client)
    }
}

impl JsonLOAD for ClientJSON{}