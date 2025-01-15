#![allow(dead_code)]
mod server;
mod http;
mod website_handler;
use  website_handler::WebsiteHandler;
use server::Server;

use http::method::Method;
use http::request::Request;

fn main() {
    let s = Server::new(String::from("127.0.0.1:8080"));
    s.run(WebsiteHandler);
    //let r = Request{path:"Path",query_string:None, method:Method::GET};

}
