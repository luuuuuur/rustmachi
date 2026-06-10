
use tun_rs::{AsyncDevice};
//creates a tun from the JSON FILE
use crate::system::config::setup_client::Setup;
use std::io::Error;
pub struct ClientTunnel{
    pub setup: Setup,
    pub identifier: String,
    pub netmask: u16
}
impl ClientTunnel{
    pub fn new(setup:Setup, netmask: u16, identifier: String) ->Self{
        Self{
            setup, 
            netmask,
            identifier,
        }
    }
    fn get_netmask(&self) -> u16 {
        self.netmask
    }
    fn get_identifier(&self) -> String{
        self.identifier.clone()
    }
}

pub trait CreateTunnel{
    fn netmask(&self) -> String;
    async fn create_device(&self) -> Result<tun_rs::AsyncDevice, Error>;
}

impl CreateTunnel for ClientTunnel{
    //tun that will be listened to
    fn netmask(&self) -> String{
        let mask = u32::MAX << (32 - self.netmask);
        format!("{}.{}.{}.{}",
            (mask) >> 24 & 0xff,
            (mask) >> 16 & 0xff,
            (mask) >> 8 & 0xff,
            mask & 0xff
        )
    }
    async fn create_device(&self)-> Result<AsyncDevice,Error>{
        let _mask = self.netmask();
        match tun_rs::DeviceBuilder::new()
        .name(self.setup.identifier())
        .mtu(1400)
        .ipv4(self.setup.ipv4().to_string(), self.netmask(),None)
        .build_async()
        { 
            Ok(device) => { 
                return Ok(device) 
            }, 
            Err(e) => { 
                return Err(e) 
            } 
        };
    }
}