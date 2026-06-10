use crate::system::config::setup_client::{ClientJSON, JsonLOAD};
use crate::system::config::setup_server::{JsonLOADSERVER, ServerJSON};
use crate::system::server::server_tun::Tunnel;
use crate::system::server::server_socket::create;
use crate::system::server::server_tcp::bind_to;
use crate::system::server::server_tcp;
use crate::system::clients::client_tun;
use crate::system::clients::client_socket;
use crate::system::clients::client_tcp;
mod system;
#[tokio::main]
async fn main() {
    let MAGIC_SERVER = "RUSTMACHI Server";
    //server setup
    let config = <ServerJSON as JsonLOADSERVER>::setup_server().unwrap();
    let tunnel = Tunnel::new(MAGIC_SERVER.to_string(),config, 30);
    let socket = create(tunnel);
    let listener = bind_to(socket).unwrap();
    let (stream, useless) = listener.accept().unwrap();
    tokio::spawn(async move {
       server_tcp::handle_client(stream).await;
    });

    //client setup
    let config_client = ClientJSON::setup_client().unwrap();
    let cli_tunnel = client_tun::ClientTunnel::new(config_client, 30, MAGIC_SERVER.to_string());
    let cli_socket = client_socket::create_socket(cli_tunnel);
    let cli_listener = client_tcp::bind_to(cli_socket).unwrap();
    let (cli_stream, useless) = cli_listener.accept().unwrap();
    tokio::spawn(async move{
        client_tcp::handle_client(cli_stream).await;
    });

}