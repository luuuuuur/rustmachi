//creates a tun from the JSON FILE
use std::net::{Ipv4Addr};
mod setup_client;
use setup_client::Setup;
use std::io::Error;
pub struct ClientTunnel{
    setup: Setup,
    identifier: String,
    netmask: u16
}
impl ClientTunnel{
    fn new(setup:Setup, netmask: u16, identifier: String) ->Self{
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
        self.identifier
    }
}

pub trait CreateTunnel{
    fn netmask(&self) -> String{

    }
    fn create_device(&self) -> Result<Device, Error>{

    }
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
    async fn create_device(&self)-> Result<Device,Error>{
        let mask = self.netmask();
        let device = match DeviceBuilder::new()
        .name(self.setup.client.identifier)
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
}