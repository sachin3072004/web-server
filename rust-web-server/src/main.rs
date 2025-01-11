mod server;
mod http;
use server::Server;

use http::method::Method;
use http::request::Request;

fn main() {
    let s = Server::new(String::from("127.0.0.1:8080"));
    s.run();
    let r = Request{path:String::from("Path"),query:None, method:Method::GET};

}
