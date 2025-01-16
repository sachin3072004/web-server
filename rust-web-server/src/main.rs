#![allow(dead_code)]
mod server;
mod http;
mod website_handler;
use  website_handler::WebsiteHandler;
use server::Server;
use std::env;
use http::method::Method;
use http::request::Request;

fn main() {
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    println!("public path:{}",public_path);
    let s = Server::new(String::from("127.0.0.1:8080"));
    s.run(WebsiteHandler::new(public_path));
    //let r = Request{path:"Path",query_string:None, method:Method::GET};

}
