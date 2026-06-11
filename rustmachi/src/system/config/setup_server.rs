use std::error::Error;
use std::net::Ipv4Addr;
use serde::Deserialize;
use std::fs;
#[derive(Deserialize)]
pub struct ServerJSON{
    virtual_ip: [u8;4],
    port: u16,
    ssh_ip: [u8; 4],
    ssh_port: u16
}
#[derive(Deserialize)]
pub struct Setup{
    #[serde(rename="Server")]
    server:ServerJSON
}

impl ServerJSON{
    pub fn new(virtual_ip: [u8;4], port: u16, ssh_ip: [u8;4], ssh_port: u16)->Self{
        Self{
            virtual_ip,
            port,
            ssh_ip,
            ssh_port
        }
    }
}
impl Setup{
    pub fn get_ip(&self) -> std::net::Ipv4Addr {
        Ipv4Addr::from_octets(self.server.virtual_ip)
    }
    pub fn get_port(&self) -> u16{
        self.server.port
    }
    pub fn get_sship(&self)-> Ipv4Addr{
        Ipv4Addr::from(self.server.ssh_ip)
    }
    pub fn get_sshport(&self)-> u16{
        self.server.ssh_port
    }
}
pub trait JsonLOADSERVER{
    fn setup_server()->Result<Setup, Box<dyn Error>>{
        let json_content = fs::read_to_string("./setup.json")?;
        let setup: Setup = serde_json::from_str(&json_content)?;
        Ok(setup)
    }
}
impl JsonLOADSERVER for ServerJSON{}