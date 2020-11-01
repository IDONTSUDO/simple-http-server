use std::io::Read;
use std::net::TcpListener;

pub struct Server {
    addr: String,
}
impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }
    pub fn run(self) {
        let listners = TcpListener::bind(&self.addr).unwrap();
        loop{
            match  listners.accept() {
                Ok(( mut stream, _)) =>{
                    let mut buffer =  [0;1024];

                    match  stream.read(&mut buffer) {
                        Ok(_)=>{
                            println!("Receved a req:{}",String::from_utf8_lossy(&buffer))
                        }
                        Err(e) => println!("FAILED")
                    }
                }
                Err(e) => println!("Failer to estabilish a connect {}",e),
            }
         

        }
        
    }
}
