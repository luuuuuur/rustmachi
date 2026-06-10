//creates the Tcp Socket for the TCP stream
//tcp sockets creates from the json file
//so we need the data from the setup_server
use std::net::{Ipv4Addr, SocketAddrV4};
mod setup_server;
mod server_tun;
use setup_server::Setup;


//get json data -> creates socket
pub fn create(setup: Setup) -> SocketAddrV4{
    let socket: SocketAddrV4 = SocketAddrV4::new(setup.server.get_ip(), setup.server.get_port());
    socket
}

