use std::net::SocketAddrV4;
use tokio::net::{TcpListener,TcpStream};
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
pub async fn handle_client(mut tcp: TcpStream) -> Result<(), Error>{
    
    let mut buffer= [0u8; 1300];
    loop{
        match tcp.read(&mut buffer).await {
            Ok(_bytes) =>{
                tcp.write_all(&buffer[0.._bytes]).await?;
                return Ok(());
            },
            Err(e)=>{
                return Err(e)
            }
            
        }
    }
}