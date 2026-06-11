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
    println!("Iniciando servidor...");
    let config = <ServerJSON as JsonLOADSERVER>::setup_server().unwrap();
    let ssh_ip = config.get_sship();
    let ssh_port = config.get_sshport();
    let tunnel = Tunnel::new(MAGIC_SERVER.to_string(), config, 30);
    println!("Creando TUN...");
    let _device = tunnel.create_tunnel().await?;
    println!("TUN creado");
    let socket = create(tunnel);
    println!("Socket creado...");
    let listener = bind_to(socket).await?;
    println!("Listener creado...escuchando en {:?}", socket);
    let (stream, addr) = listener.accept().await?;
    println!("Esperando conexiones...");
    let ssh_stream = TcpStream::connect((ssh_ip, ssh_port)).await?;
    println!("Conexion!: {}",addr);
    server_tcp::handle_client(stream, ssh_stream).await?;
    
    Ok(())
}

async fn run_client() -> Result<(), std::io::Error> {
    println!("Iniciando cliente...");
    let config_client = ClientJSON::setup_client().unwrap();
    let server_ip = config_client.get_real();
    let server_port = config_client.get_real_port();
    println!("Conectando a {:?}:{}", server_ip, server_port);
    let cli_tunnel = client_tun::ClientTunnel::new(config_client, 30, "RUSTMACHI Client".to_string());
    let cli_device = cli_tunnel.create_device().await?;
    println!("TUN creado");
    let stream = TcpStream::connect((server_ip, server_port)).await?;
    println!("Conectado al servidor");
    
    client_tcp::handle_client(stream, cli_device).await?;
    
    Ok(())
}