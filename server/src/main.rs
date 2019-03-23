extern crate ws;
use ws::{listen, Handler, Message, Request, Response, Result, Sender};
use std::fs::File;
use std::io::Read;
use std::collections::HashMap;


struct Server {
    sender: Sender,
    files_cache: HashMap<String, Vec<u8>>
}

impl Server {
    fn new(sender: Sender) -> Server {
        let files_cache = HashMap::new();
        Server { sender, files_cache }
    }

    fn get_file(&mut self, filename: String) -> Vec<u8> {
        match self.files_cache.get(&filename) {
            Some(file) => file.clone(),
            None => {
                match File::open(format!("../dist{}", filename)) {
                    Ok(mut file) => {
                        let mut buffer = Vec::new();
                        file.read_to_end(&mut buffer).expect("Reading file");
                        self.files_cache.insert(filename, buffer.clone());
                        buffer
                    },
                    Err(_) => {
                        format!("404 - Resource {} not found!", filename).as_bytes().to_vec()
                    }
                }
            }
        }
    }
}

impl Handler for Server {
    fn on_request(&mut self, req: &Request) -> Result<(Response)> {
        match req.resource() {
            "/ws" => Response::from_request(req),
            "/" => Ok(Response::new(200 , "OK", self.get_file("/index.html".to_string()))),
            path => Ok(Response::new(200 , "OK", self.get_file(path.to_string())))
        }
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        // Broadcast to all connections
        self.sender.broadcast(msg)
    }
}

fn main() {
    let addr = "127.0.0.1:8000";
    println!("Listening on http://{}", addr);
    listen(addr, |sender| Server::new(sender)).unwrap();
}
