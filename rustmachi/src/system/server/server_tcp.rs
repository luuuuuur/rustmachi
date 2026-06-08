//Server flow
//Listener -> Socket(Created from the virtual TUN)
//Sends data through the kernel
use std::io::Error;
use std::net::{TcpListener, SocketAddrV4};

//binds and return the listener
pub fn bind_to(socket:SocketAddrV4)-> Result<TcpListener, Error>{
    let tcp_listener = match TcpListener::bind(socket){
        Ok(tcp_listener) =>{
            return Ok(tcp_listener)
        },
        Err(e) =>{
            println!("Failed to create socket: line 17");
            return Err(e)
        }
    };
    
}