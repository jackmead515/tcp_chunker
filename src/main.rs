use std::net::TcpListener;
use std::thread;
use std::sync::Arc;

use futures::executor::block_on;

mod protocol;

fn main() {
    let cache = Arc::new(protocol::AppCache::new());
    let listener = TcpListener::bind("127.0.0.1:3434").expect("CRAP!");

    for stream in listener.incoming() {
        match stream {
            Ok(client) => {
                let mut pipeline = protocol::Pipeline::new(client, cache.clone());
                thread::spawn(move || block_on(pipeline.run()));
            }
            Err(err) => {
                println!("{}", err);
            }
        }
    }
}
