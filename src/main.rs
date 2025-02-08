
use HTTP_SERVER::server::Server;
use HTTP_SERVER::http::Method;
fn main() {
    let get = Method::GET;
    let delete = Method::DELETE;
    let post = Method::POST;
    let put = Method::PUT;
    let server = Server::new("127.0.0.1:8080");
    server.start();
}
