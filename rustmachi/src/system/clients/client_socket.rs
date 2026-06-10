use std::net::{SocketAddrV4};
use crate::system::clients::client_tun::ClientTunnel;


pub fn create_socket(client: ClientTunnel) -> SocketAddrV4{
    let socket = SocketAddrV4::new(client.setup.ipv4(), client.setup.port());
    socket
}