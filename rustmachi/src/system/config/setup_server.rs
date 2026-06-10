use std::error::Error;
use std::net::Ipv4Addr;
use serde::Deserialize;
use std::fs;
#[derive(Deserialize)]
pub struct ServerJSON{
    ipv4: Ipv4Addr,
    port: u16,
    ssh_ip: Ipv4Addr,
    ssh_port: u16
}
pub struct Setup{
    #[serde(rename="Server")]
    server:ServerJSON
}

impl ServerJSON{
    pub fn new(ipv4: Ipv4Addr, port: u16, ssh_ip: Ipv4Addr, ssh_port: u16)->Self{
        Self{
            ipv4,
            port,
            ssh_ip,
            ssh_port
        }
    }
}
impl Setup{
    pub fn get_ip(&self) -> std::net::Ipv4Addr {
        self.setup.server.ipv4
    }
    pub fn get_port(&self) -> u16{
        self.setup.server.port
    }
    pub fn get_sship(&self)-> Ipv4Addr{
        self.setup.server.ssh_ip
    }
    pub fn get_sshport(&self)-> u16{
        self.setup.server.ssh_port
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