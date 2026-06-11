use std::net::SocketAddrV4;
use tokio::net::{TcpListener,TcpStream};
use tun_rs::AsyncDevice;
use std::io::Error;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
pub async fn bind_to(socket:SocketAddrV4)-> Result<TcpListener, Error>{
    match TcpListener::bind(socket).await{
        Ok(l) =>{
            Ok(l)
        },
        Err(e) =>{
            return Err(e)
        }
    }
    
}
//Needs the tcp.accept as param
pub async fn handle_client(mut tcp: TcpStream, device: AsyncDevice) -> Result<(), Error>{
    let mut tun_buffer = [0u8; 1300];
    let mut tcp_buffer = [0u8; 1300];

    loop {
        tokio::select! {
            Ok(n) = device.recv(&mut tun_buffer) => {
                tcp.write_all(&tun_buffer[..n]).await?;
            }
            Ok(n) = tcp.read(&mut tcp_buffer) => {
                if n == 0 { break; }
                device.send(&tcp_buffer[..n]).await?;
            }
        }
    }
    Ok(())
}