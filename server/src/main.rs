extern crate ws;
use ws::{listen, Handler, Message, Request, Response, Result, Sender};
use std::fs::File;
use std::io::Read;


struct Server {
    sender: Sender,
    index: Vec<u8>
}

impl Server {
    fn new(sender: Sender) -> Server {
        let mut file = File::open("./src/index.html").expect("Opening file index.html");

        let mut index = Vec::new();
        file.read_to_end(&mut index).expect("Reading file");

        Server { sender, index }
    }
}

impl Handler for Server {
    fn on_request(&mut self, req: &Request) -> Result<(Response)> {
        match req.resource() {
            "/ws" => Response::from_request(req),
            "/" => Ok(Response::new(200, "OK", self.index.clone())),
            path => Ok(Response::new(404, "Not Found", format!("404 - Resource {} not found!", path).as_bytes().to_vec()))
        }
    }

    // Handle messages received in the websocket (in this case, only on /ws)
    fn on_message(&mut self, msg: Message) -> Result<()> {
        // Broadcast to all connections
        self.sender.broadcast(msg)
    }
}

fn main() {
    let addr = "127.0.0.1:8000";
    listen(addr, |sender| Server::new(sender)).unwrap();
    println!("Listening on http://{}", addr);
}
