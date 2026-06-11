use tokio::net::TcpStream;
use crate::system::config::setup_client::{ClientJSON, JsonLOAD};
use crate::system::config::setup_server::{JsonLOADSERVER, ServerJSON};
use crate::system::server::server_tun::Tunnel;
use crate::system::server::server_socket::create;
use crate::system::server::server_tcp::bind_to;
use crate::system::server::server_tcp;
use crate::system::clients::client_tun;
use crate::system::clients::client_socket;
use crate::system::clients::client_tcp;
use crate::system::clients::client_tun::CreateTunnel;
use crate::system::server::server_tun::TunnelTrait;
use std::env;
mod system;
#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let mode = args.get(1).expect("Uso: rustmachi [server|client]");
    
    match mode.as_str() {
        "server" => run_server().await?,
        "client" => run_client().await?,
        _ => panic!("Modo inválido")
    }
    
    Ok(())
}


async fn run_server() -> Result<(), std::io::Error> {
    let MAGIC_SERVER = "RUSTMACHI";
    let config = <ServerJSON as JsonLOADSERVER>::setup_server().unwrap();
    let ssh_ip = config.get_sship();
    let ssh_port = config.get_sshport();
    let tunnel = Tunnel::new(MAGIC_SERVER.to_string(), config, 30);
    let _device = tunnel.create_tunnel().await?;
    let socket = create(tunnel);
    let listener = bind_to(socket).await?;
    let (stream, _) = listener.accept().await?;
    let ssh_stream = TcpStream::connect((ssh_ip, ssh_port)).await?;
    tokio::spawn(async move {
        server_tcp::handle_client(stream, ssh_stream).await.ok();
    });
    Ok(())
}

async fn run_client() -> Result<(), std::io::Error> {
    let MAGIC_SERVER = "RUSTMACHI";
    let config_client = ClientJSON::setup_client().unwrap();
    let cli_tunnel = client_tun::ClientTunnel::new(config_client, 30, MAGIC_SERVER.to_string());
    let _cli_device = cli_tunnel.create_device().await?;
    let cli_socket = client_socket::create_socket(cli_tunnel);
    let cli_listener = client_tcp::bind_to(cli_socket).await?;
    let (cli_stream, _) = cli_listener.accept().await?;
    tokio::spawn(async move {
        client_tcp::handle_client(cli_stream).await.ok();
    });
    Ok(())
}