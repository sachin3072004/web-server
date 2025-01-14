use crate::http::{Request,Response, StatusCode};
use std::io::{Read,Write};
use std::net::{TcpListener};    
use std::convert::TryFrom;
#[derive(Debug)]
pub struct Server{
        address: String,
    }

    impl Server{
        pub fn new(s:String)->Server{
            Server{address:s}
        }

        pub fn run(&self){
            println!("Listening {}",self.address);
            let listener = TcpListener::bind(self.address.as_str()).unwrap();
            loop {
                match listener.accept() {
                    Ok((mut stream,_))=> {
                        let mut buffer = [0;1024];
                        match stream.read(&mut buffer){
                            Ok(_)=>{
                                println!("Received a request:{}",String::from_utf8_lossy(&buffer));
                                let response = match Request::try_from(&buffer[..]){
                                    Ok(request)=>{
                                        dbg!(request);
                                        Response::new(StatusCode::Ok, 
                                                        Some("<h1>It Works!!</h>".to_string())) 
                                    },
                                    Err(e)=>{
                                        Response::new(StatusCode::BadRequest, None)

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
