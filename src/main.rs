fn main() {
    let server = Server::new("127.0.0.1:80");
    server.start();
}
#[derive(Debug)]
struct Server {
    addr: String,
}

impl Server {

    fn new<T: Into<String>>(address: T) -> Self {
        Self {
            addr: address.into(),
        }
    }
    fn start(&self) {
        println!("The server will start listening on: {:?}", &self.addr);
    }
}