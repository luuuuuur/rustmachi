use std::net::{TcpListener, TcpStream, SocketAddrV4};
use std::io::{Error, Read, Write};
pub fn bind_to(socket:SocketAddrV4)-> Result<TcpListener, Error>{
    let _tcp_listener = match TcpListener::bind(socket){
        Ok(tcp_listener) =>{
            return Ok(tcp_listener)
        },
        Err(e) =>{
            println!("Failed to create socket: line 17");
            return Err(e)
        }
    };
    
}
//Needs the tcp.accept as param
pub async fn handle_client(mut tcp: TcpStream) -> Result<(), Error>{
    let mut buffer= [0u8; 1300];
    loop{
        match tcp.read(&mut buffer) {
            Ok(_bytes) =>{
                tcp.write_all(&buffer[0.._bytes]);
                return Ok(());
            },
            Err(e)=>{
                return Err(e)
            }
            
        }
    }
}