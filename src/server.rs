use std::io::Read;
use std::net::{TcpListener, TcpStream};
use crate::http::Request;
use std::convert::TryFrom;
use std::convert::TryInto;
#[derive(Debug)]
pub struct Server {
    addr: String,
}
impl Server {
    pub fn new<T: Into<String>>(address: T) -> Self {
        Self {
            addr: address.into(),
        }
    }
    pub fn start(&self) {
        println!("Staring Server....");
        let listener = TcpListener::bind(&self.addr);
        match listener {
            Ok(listener) => {
                println!("✅ Server started on {}", self.addr);
                self.handle_connections(&listener);
            }
            Err(e) => {
                eprintln!("❌ Error in binding: {}", e);
                }
            }
        }

    pub fn handle_connections(&self,tcp_listener: &TcpListener){
            println!(" Server is now receiving incoming connections");
            for stream in tcp_listener.incoming(){
                match stream {
                    Ok(stream) => {
                        let client_connection = stream.peer_addr();
                        match client_connection {
                            Ok(client_connection) => {
                                let conncection_ip = client_connection;
                                println!("Connection Received from {:?}", conncection_ip);

                            }
                            Err(e)=> {
                                eprintln!("There is an error {:?}",e);
                            }
                        }
                        self.handle_client(stream);

                    }
                    Err(e)=> {
                        eprintln!("Error in receiving Stream {:?}", e);
                    },
                }
            }
    }
    pub fn handle_client(&self, mut tcp_stream: TcpStream){
        println!("Handling client {:?}", tcp_stream.peer_addr());
        let mut buffer = [0u8;1024];
        let message = tcp_stream.read(&mut buffer);
        match message {
            Ok(message) => {
                println!("Bytes Received");
                let bytes = String::from_utf8_lossy(&buffer);
                match Request::try_from(&buffer[..message]){
                    Ok(request) => {},
                    Err(e) => println!("Failed to parse"),
                }
                println!("This is the message {}",bytes);
            }
            Err(e)=>{
                eprintln!("Error {:?}",e);
            }
        }

    }
    }

