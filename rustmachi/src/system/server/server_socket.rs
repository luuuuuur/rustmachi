//creates the Tcp Socket for the TCP stream
//tcp sockets creates from the json file
//so we need the data from the setup_server
use std::net::{Ipv4Addr, SocketAddrV4};
use crate::system::server::server_tun::Tunnel;


//get json data -> creates socket
pub fn create(tunnel: Tunnel) -> SocketAddrV4{
    let socket: SocketAddrV4 = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), tunnel.setup.get_port());
    socket
}

