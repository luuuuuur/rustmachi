//Server flow
//Listener -> Socket(Created from the virtual TUN)
//Sends data through the kernel
use std::io::Error;
use std::net::{SocketAddrV4};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tun_rs::AsyncDevice;
use tokio::net::TcpStream;
use tokio::net::TcpListener;
//binds and return the listener
pub async fn bind_to(socket:SocketAddrV4)-> Result<TcpListener, Error>{
    TcpListener::bind(socket).await
}

//for streams in listener.accept() -> use this function to operate those streams
pub async fn handle_client(mut tcp: TcpStream, device: AsyncDevice) -> Result<(), Error>{
    let mut tun_buffer = [0u8; 1200];
    let mut tcp_buffer = [0u8; 1200];

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
