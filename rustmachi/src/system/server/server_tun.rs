//it creates a tunnel based on  the data retrieved in the JSON load file (setup_server.rs)
mod setup_server;
mod server_tcp;
use setup_server::ServerJSON;
use std::net::{TcpStream, SocketAddr, TcpListener, Ipv4Addr};
use server_tcp::bind_to;
use tun_rs::DeviceBuilder;


pub struct Tunnel{
    identifier: String,
    virtual_ip: Ipv4Addr,
    netmask: u8
}

pub trait TunnelTrait{
    async fn create_tunnel(tun: Tunnel)->Result<Device,Error>{}
}
impl Tunnel{
    pub fn new(identifier:String, virtual_ip: IPv4Addr, netmask: u8)-> Tunnel{
        Self{
            identifier,
            virtual_ip,
            netmask
        }
    }
}
impl TunnelTrait for Tunnel{
    async fn create_tunnel(tun: Tunnel)-> Result<Device,Error>{
        let device = match DeviceBuilder::new()
        .name(tun.identifier)
        .mtu(1500)
        .ipv4(tun.virtual_ip, "255.255.255.0",None)
        .build_async()
        .await{
            Ok(device) => {
                return device
            },
            Err(e) => {
                return Err(e)
            }
        };
    }
}
