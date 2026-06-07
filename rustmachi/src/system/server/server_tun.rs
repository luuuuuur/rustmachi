//it creates a tunnel based on  the data retrieved in the JSON load file (setup_server.rs)
mod setup_server;
use setup_server::ServerJSON;

use std::net::{TcpStream, SocketAddr};

pub struct Tunnel{
    identifier: String,
    virtual_ip: u32,
    netmask: u8
}

pub trait TunnelTrait{
    fn redirect(TcpSocket: SocketAddr)->(){
    
    }
    fn configure_tunnel(tunnel: Tunnel) ->Result<(), BoxError>{

    }
}
impl Tunnel{
    pub fn new(identifier:String, virtual_ip: u32, netmask: u8)-> Tunnel{
        Self{
            identifier,
            virtual_address,
            netmask
        }
    }
}
impl TunnelTrait for Tunnel{}
