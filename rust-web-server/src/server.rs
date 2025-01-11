use crate::http::request::Request;
use std::io::Read;
use std::net::{TcpListener};    
use std::convert::TryFrom;
pub struct Server{
        address: String,
    }

    impl Server{
        pub fn new(s:String)->Server{
            Server{address:s}
        }

        pub fn run(&self){
            let t = (1,2,3,"sachin");
            println!("{:?} {:?}",t.0,t.1);
            println!("Listening {}",self.address);
            let listener = TcpListener::bind(self.address.as_str()).unwrap();
            loop {
                match listener.accept() {
                    Ok((mut stream,_))=> {
                        let mut buffer = [0;1024];
                        match stream.read(&mut buffer){
                            Ok(_)=>{
                                println!("Received a request:{}",String::from_utf8_lossy(&buffer));
                                match Request::try_from(&buffer[..]){
                                    Ok(request)=>{},
                                    Err(e)=>{println!("Failed ot parse the request {}",e)}
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
