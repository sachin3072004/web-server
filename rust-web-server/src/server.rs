use crate::http::{Request,Response, StatusCode, ParseError};
use std::io::{Read,Write};
use std::net::{TcpListener};    
use std::convert::TryFrom;

pub trait Handler {
    fn handle_request(&mut self, request: &Request)->Response;
    fn handle_bad_request(&mut self, e: &ParseError)->Response{
        println!("Failed to Parse Request {}",e);
        Response::new(StatusCode::BadRequest, None)
    }
}

#[derive(Debug)]
pub struct Server{
        address: String,
    }

    impl Server{
        pub fn new(s:String)->Server{
            Server{address:s}
        }

        pub fn run(&self, mut handler: impl Handler){
            println!("Listening {}",self.address);
            let listener = TcpListener::bind(self.address.as_str()).unwrap();
            loop {
                match listener.accept() {
                    Ok((mut stream,_))=> {
                        let mut buffer = [0;1024];
                        match stream.read(&mut buffer){
                            Ok(_)=>{
                                let response = match Request::try_from(&buffer[..]){
                                    Ok(request)=>{
                                        handler.handle_request(&request)
                                    },
                                    Err(e)=>{
                                        handler.handle_bad_request(&e)
                                    }
                                };
                                if let Err(e) =  response.send(&mut stream){
                                    println!("Failed to send response");
                                }

                            },
                            Err(e)=>println!("Failed to read from connection:{}",e)
                        }
                    }
                    Err(e)=>println!("Faled to establish connection :{}",e),
                }
            }
        }
    }
