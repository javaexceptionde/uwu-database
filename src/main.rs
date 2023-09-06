mod handler;
mod command;
mod database;

use std::net::{TcpListener};
use std::thread;
use crate::handler::connection_handler;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8080").expect("Failed to bind to address");

    println!("Listening on 0.0.0.0:8080");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // Spawn a new thread to handle the client connection
                println!("New client connection: {}", stream.peer_addr().unwrap());
                thread::spawn(|| {
                    connection_handler::handle_client(stream)
                });
            }
            Err(e) => {
                eprintln!("Error accepting client connection: {}", e);
            }
        }
    }
}
