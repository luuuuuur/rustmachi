//Server flow
//Listener -> Socket(Created from the virtual TUN)
//Sends data through the kernel
use std::io::Error;
use std::net::{TcpListener, SocketAddrV4, TcpStream};

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

//for streams in listener.accept() -> use this function to operate those streams
pub async fn handle_client(mut tcpClient: TcpStream)->Result<(), std::io::Error>{
    let mut buffer= [0u8; 1300];
    loop{
        let bytes_read = tcpClient.read(&mut buffer).await?;
        if bytes_read == 0 {
            return Ok(())
        }
        if bytes_read > 1300 {
            return Err(Error::new(ErrorKind::InvalidData, "Data not supported by the MTU"));
        }
        tcpClient.write_all(&buffer[0..bytes_read]).await?
    }

}
