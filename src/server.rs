use crate::http::{
    request::ParseError,
    Request, Response, StatusCode,
};
use std::{
    io::Read,
    net::TcpListener,
};

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;

    fn handle_bad_request(&mut self, e: &ParseError) -> Response{
        println!("Failed to parse request: {}", e);
        Response::new(StatusCode::BadResquest, None)
    }
}
pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Server { addr }
    }

    pub fn run(self, mut handler: impl Handler) {
        println!("listening in address {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();
        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    println!("success");
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("readed stream to buffer succesfully");
                            println!(
                                "-------text:--------\n\n {}",
                                String::from_utf8_lossy(&buffer)
                            );

                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => handler.handle_request(&request),
                                Err(e) => handler.handle_bad_request(&e),
                            };

                            if let Err(e) = response.send(&mut stream) {
                                println!("failed to send response: {}", e)
                            }
                        }
                        _ => {
                            println!("error while reading stream")
                        }
                    }
                }

                Err(_) => {
                    println!("error stablishing tcp connection");
                }
            }
        }
    }
}
