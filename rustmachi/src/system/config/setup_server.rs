use std::error::Error;
use std::net::Ipv4Addr;
use serde::Deserialize;
use std::fs;
#[derive(Deserialize)]
pub struct ServerJSON{
    virtual_ip: Ipv4Addr,
    port: u16,
    ssh_ip: Ipv4Addr,
    ssh_port: u16,
    identifier: String
}
pub struct Setup{
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
}
impl Setup{
    pub fn get_ip(&self) -> std::net::Ipv4Addr {
        &self.server.virtual_ip
    }
    pub fn get_port(&self) -> u16{
        &self.server.port
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