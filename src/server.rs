use std::{io::Read, net::TcpListener};
use crate::http::Request;
pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Server { addr }
    }

    pub fn run(self) {
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
                            println!("-------text:--------\n\n {}", String::from_utf8_lossy(&buffer));
                            
                            match Request::try_from(&buffer[..]) {
                                Ok(request)=>{
                                    println!("{:?}",request);
                                },
                                Err(_)=>println!("error creating Request from buffer")
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
