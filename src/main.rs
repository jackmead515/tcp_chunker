use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::thread;

use futures::executor::block_on;

mod protocol;

async fn handle(client: &mut TcpStream) -> Result<(), protocol::errors::Errors> {
    let headers = protocol::headers::headers(client)?;


    return Ok(());
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3434").expect("CRAP!");

    for stream in listener.incoming() {
        match stream {
            Ok(client) => {
                let mut client = client;
                thread::spawn(move || block_on(handle(&mut client)));
            }
            Err(err) => {
                println!("{}", err);
            }
        }
    }
}
