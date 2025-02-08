use std::net::TcpListener;
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
                //self.handle_connections(listener);
            }
            Err(e) => {
                eprintln!("❌ Error in binding: {}", e);
                }
            }
        }
    }
