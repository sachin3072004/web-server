struct Server{
    address:String,
    port: i32
}

impl Server{
    fn new(s:String)->Server{
        Server{address:s,port:p}
    }
}
fn main() {
        let mut s = Server::new(String::from("127.0.0.1:8080"));
}
