//Server flow
//Listener -> Socket(Created from the virtual TUN)
//Sends data through the kernel
use std::io::Error;
use std::net::{SocketAddrV4};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::net::TcpListener;
//binds and return the listener
pub async fn bind_to(socket:SocketAddrV4)-> Result<TcpListener, Error>{
    TcpListener::bind(socket).await
}

//for streams in listener.accept() -> use this function to operate those streams
pub async fn handle_client(tcp: TcpStream, ssh_stream: TcpStream) -> Result<(), std::io::Error> {
    let mut tcp_buffer = [0u8; 1300];
    let mut ssh_buffer = [0u8; 1300];
    let (mut tcp_read, mut tcp_write) = tcp.into_split();
    let (mut ssh_read, mut ssh_write) = ssh_stream.into_split();
    
    loop {
        tokio::select! {
            n = tcp_read.read(&mut tcp_buffer) => {
                match n? {
                    0 => break, // conexión cerrada
                    bytes => ssh_write.write_all(&tcp_buffer[..bytes]).await?,
                }
            }
            n = ssh_read.read(&mut ssh_buffer) => {
                match n? {
                    0 => break,
                    bytes => tcp_write.write_all(&ssh_buffer[..bytes]).await?,
                }
            }
        }
    }
    
    Ok(())
}
