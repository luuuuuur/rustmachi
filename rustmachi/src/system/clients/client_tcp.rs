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
    let mut tun_buf = [0u8; 1200];
    let mut tcp_buf = [0u8; 1200];
    let mut len_buf = [0u8; 2];
    let (mut tcp_read, mut tcp_write) = tcp.into_split();

    loop {
        tokio::select! {
            // TUN → TCP
            Ok(n) = device.recv(&mut tun_buf) => {
                let len = (n as u16).to_be_bytes();
                tcp_write.write_all(&len).await?;
                tcp_write.write_all(&tun_buf[..n]).await?;
            }
            // TCP → TUN
            Ok(_) = tcp_read.read_exact(&mut len_buf) => {
                let len = u16::from_be_bytes(len_buf) as usize;
                tcp_read.read_exact(&mut tcp_buf[..len]).await?;
                device.send(&tcp_buf[..len]).await?;
            }
        }
    }
}