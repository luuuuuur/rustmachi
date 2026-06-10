//Server flow
//Listener -> Socket(Created from the virtual TUN)
//Sends data through the kernel
use std::io::{Error, Read, Write};
use std::net::{TcpListener, SocketAddrV4, TcpStream};

//binds and return the listener
pub fn bind_to(socket:SocketAddrV4)-> Result<TcpListener, Error>{
    match TcpListener::bind(socket){
        Ok(tcp_listener) =>{
            return Ok(tcp_listener)
        },
        Err(e) =>{
            println!("Failed to create socket: line 17");
            return Err(e)
        }
    };
    
}

//for streams in listener.accept() -> use this function to operate those streams
pub async fn handle_client(mut tcp: TcpStream)->Result<(), std::io::Error>{
    let mut buffer= [0u8; 1300];
    loop{
        match tcp.read(&mut buffer){
            Ok(_bytes)=>{
                tcp.write_all(&buffer[0.._bytes]);
                return Ok(())
            },
            Err(e) => {
                return Err(e);
            }
        }
    }

}
