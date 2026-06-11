use std::error::Error;
use std::net::Ipv4Addr;
use serde::Deserialize;
use std::fs;
#[derive(Deserialize,Debug)]
pub struct ClientJSON{
    pub virtual_ip: [u8;4],
    pub port: u16,
    pub identifier: String,
    pub server_real_ip: [u8;4],
    pub server_real_port: u16
    
}
#[derive(Deserialize)]
pub struct Setup{
    #[serde(rename="Client")]
    client: ClientJSON
}

impl ClientJSON{
    pub fn new(virtual_ip:[u8;4],port:u16, identifier: String, server_real_ip: [u8;4], server_real_port: u16) -> Self{
        Self{
            virtual_ip,
            port,
            identifier,
            server_real_ip,
            server_real_port
        }
    }
}
impl Setup{
    pub fn ipv4(&self) -> std::net::Ipv4Addr {
        Ipv4Addr::from_octets(self.client.virtual_ip)
    }
    pub fn port(&self) -> u16 {
        self.client.port
    }
    pub fn identifier(&self) -> String {
        self.client.identifier.clone()  
    }
    pub fn get_real(&self)-> Ipv4Addr{
        Ipv4Addr::from_octets(self.client.server_real_ip)
    }
    pub fn get_real_port(&self) ->u16{
        self.client.server_real_port
    }
}
pub trait JsonLOAD{
    fn setup_client()->Result<Setup, Box<dyn Error>>{
        let json_content = fs::read_to_string("./setup.json")?;
        let setup: Setup = serde_json::from_str(&json_content)?;
        Ok(setup)
    }
}

impl JsonLOAD for ClientJSON{}