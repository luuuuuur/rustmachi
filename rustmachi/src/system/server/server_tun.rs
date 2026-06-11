//it creates a tunnel based on  the data retrieved in the JSON load file (setup_server.rs)
use tun_rs::AsyncDevice;
use std::io::{Error};
use tun_rs::DeviceBuilder;
use crate::system::config::setup_server::Setup;
pub struct Tunnel{
    identifier: String,
    pub setup:Setup,
    netmask: u8
}

pub trait TunnelTrait{
    async fn create_tunnel(&self)->Result<AsyncDevice, Error>;
    fn netmask(&self) -> String;
}
impl Tunnel{
    pub fn new(identifier:String, setup:Setup, netmask: u8)-> Self{
        Self{
            identifier,
            setup,
            netmask,
        }
    }
}
impl TunnelTrait for Tunnel{
    async fn create_tunnel(&self) -> Result<AsyncDevice, Error> {
        let device = DeviceBuilder::new()
            .name(self.identifier.clone())
            .mtu(1400)
            .ipv4(self.setup.get_ip(), self.netmask(), None)
            .build_async()?;
        Ok(device)
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
