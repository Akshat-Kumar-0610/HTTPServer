use std::net::TcpListener;
use std::io::Read;

pub struct Server {
    address: String,   
}
impl Server {
    pub fn new(address: String) -> Self {
        Self {
            address: address
        }
    }
    pub fn run(self) {
        println!("{}", self.address);
        let listner = TcpListener::bind(&self.address).unwrap();
        
        loop {    
            match listner.accept() {
                Ok((mut stream, address)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) =>{
                           println!("Recieved a connection : {}", String::from_utf8_lossy(&buffer)) 
                        },
                        Err(e) =>{
                            println!("Error While reading connection: {}", e)
                        }
                    }
                },
                Err(E) => println!("{}",E)
            } 
        }
    }
}