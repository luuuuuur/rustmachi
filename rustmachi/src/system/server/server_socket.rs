//creates the Tcp Socket for the TCP stream
//tcp sockets creates from the json file
//so we need the data from the setup_server
use std::net::{SocketAddrV4};
use crate::system::server::server_tun::Tunnel;


//get json data -> creates socket
pub fn create(tunnel: Tunnel) -> SocketAddrV4{
    let socket: SocketAddrV4 = SocketAddrV4::new(tunnel.setup.get_ip(), tunnel.setup.get_port());
    socket
}

