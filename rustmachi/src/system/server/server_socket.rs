//creates the Tcp Socket for the TCP stream
//tcp sockets creates from the json file
//so we need the data from the setup_server
use std::net::{Ipv4Addr, SocketAddrV4};
mod setup_server;
mod server_tun;
use server_tun::Tunnel;
use setup_server::ServerJSON;


//get json data -> creates socket
pub fn create(json_data: ServerJSON, tunnel: Tunnel) -> SocketAddrV4{
    let socket: SocketAddrV4 = SocketAddrV4::new(tunnel.virtual_ip, ServerJSON.port);
    socket
}

