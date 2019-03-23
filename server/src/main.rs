extern crate ws;
use std::collections::HashMap;
use std::fs::{read_to_string, File};
use std::io::Read;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use ws::{listen, Handler, Message, Request, Response, Result, Sender};

extern crate libc;
use std::ffi::CString;

struct Server {
    files_cache: HashMap<String, Vec<u8>>,
}

fn send(ws_sender: Sender, receiver: Arc<Mutex<Receiver<String>>>) {
    println!("Spawning thread...");
    thread::spawn(move || loop {
        let msg = receiver.lock().unwrap().recv().unwrap();
        println!("Pushing to websocket: {}", msg);
        ws_sender
            .broadcast(Message::Text(msg))
            .expect("Pushing to websocket");
    });
}

fn absorb(pipe_name: String, sender: mpsc::Sender<String>) {
    let c_pipe_name = CString::new(pipe_name.clone()).unwrap();
    unsafe {
        libc::mkfifo(c_pipe_name.as_ptr(), 0o644);
    }

    thread::spawn(move || loop {
        let content =
            read_to_string(pipe_name.clone()).expect("Something went wrong reading the file");
        sender.send(content).expect("Sending file content");
    });
}

impl Server {
    fn new(ws_sender: Sender, receiver: Arc<Mutex<Receiver<String>>>) -> Server {
        let files_cache = HashMap::new();
        println!("Calling send...");
        send(ws_sender, receiver);
        Server { files_cache }
    }

    /// Serves static files of directory ./dist
    fn get_file(&mut self, filename: String) -> Result<(Response)> {
        match self.files_cache.get(&filename) {
            Some(file) => Ok(Response::new(200, "OK", file.clone())),
            None => match File::open(format!("./dist{}", filename)) {
                Ok(mut file) => {
                    let mut buffer = Vec::new();
                    file.read_to_end(&mut buffer).expect("Reading file");
                    self.files_cache.insert(filename, buffer.clone());
                    Ok(Response::new(200, "OK", buffer))
                }
                Err(_) => {
                    let buffer = format!("404 - Resource {} not found!", filename)
                        .as_bytes()
                        .to_vec();
                    Ok(Response::new(404, "404 Not Found", buffer))
                }
            },
        }
    }
}

impl Handler for Server {
    fn on_request(&mut self, req: &Request) -> Result<(Response)> {
        match req.resource() {
            "/ws" => Response::from_request(req),
            "/" => self.get_file("/index.html".to_string()),
            path => self.get_file(path.to_string()),
        }
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        Ok(())
    }
}

fn main() {
    let (sender, receiver) = mpsc::channel();
    absorb("./comm/node".to_string(), sender.clone());
    let receiver = Arc::new(Mutex::new(receiver));
    let addr = "127.0.0.1:8000";
    println!("Listening on http://{}", addr);
    listen(addr, |sender| Server::new(sender, Arc::clone(&receiver))).expect("Listing on ethernet");
}
