//it creates a tunnel based on  the data retrieved in the JSON load file (setup_server.rs)
mod setup_server;
mod server_tcp;
use setup_server::Setup;
use std::net::{TcpStream, SocketAddr, TcpListener, Ipv4Addr};
use tun_rs::DeviceBuilder;
pub struct Tunnel{
    identifier: String,
    setup: Setup,
    netmask: u8
}

pub trait TunnelTrait{
    async fn create_tunnel(&self)->Result<Device,Error>{}
    fn netmask(&self) -> String{}
}
impl Tunnel{
    pub fn new(identifier:String, setup: Setup, netmask: u8)-> Self{
        Self{
            identifier,
            setup,
            netmask,
        }
    }
}
impl TunnelTrait for Tunnel{
    async fn create_tunnel(&self)-> Result<Device,Error>{
        let mask = self.netmask();
        let device = match DeviceBuilder::new()
        .name(self.identifier)
        .mtu(1400)
        .ipv4(self.setup.client.get_ip().to_string(), self.netmask(),None)
        .build_async()
        .await{
            Ok(device) => {
                return Ok(device)
            },
            Err(e) => {
                return Err(e)
            }
        };
    }
    fn netmask(&self) -> String{
        let mask = u32::MAX << (32 - self.netmask);
        format!("{}.{}.{}.{}",
            (mask) >> 24 & 0xff,
            (mask) >> 16 & 0xff,
            (mask) >> 8 & 0xff,
            mask & 0xff
        )
    }
}
