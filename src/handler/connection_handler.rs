use std::io::{Read, Write};
use std::net::TcpStream;
use serde_json::{Value, Error};
use crate::handler::command_handler::command_handler;

pub(crate) fn handle_client(mut stream: TcpStream) {
    // Create a buffer to hold incoming data
    let mut buffer = [0; 1024];

    loop {
        let bytes_read = stream.read(&mut buffer).expect("Failed to read from client");

        if bytes_read == 0 {
            println!("Client disconnected");
            break;
        }

        let read = &buffer[..bytes_read];
        handle_packet(read, stream.try_clone().unwrap());
    }

}

fn parse_command(command: &str) -> Result<Vec<String>, Error> {
    let mut parts = vec![];
    let mut current_part = String::new();
    let mut inside_json = false;
    let mut json_depth = 0;
    let mut inside_string_arg = false;
    let mut inside_nested_string = false;

    for c in command.chars() {
        if c == '.' && !inside_json && !inside_string_arg {
            if !current_part.is_empty() {
                parts.push(current_part.clone());
                current_part.clear();
            }
        } else if c == '{' {
            inside_json = true;
            json_depth += 1;
            current_part.push(c);
        } else if c == '}' {
            json_depth -= 1;
            if json_depth == 0 {
                inside_json = false;
                current_part.push(c);
                if !current_part.is_empty() {
                    let _parsed_json: Value = serde_json::from_str(&current_part)?;
                    current_part.clear();
                }
            } else {
                current_part.push(c);
            }
        } else if c == '"' {
            if inside_nested_string {
                inside_nested_string = false;
            } else if inside_string_arg {
                inside_nested_string = true;
            }
            inside_string_arg = !inside_string_arg;
            current_part.push(c);
        } else {
            current_part.push(c);
        }
    }

    if !current_part.is_empty() {
        parts.push(current_part);
    }
    Ok(parts)
}


/*
    0x00 - Command
    0x01 - Ping
    0x02 - Stay Alive
    0x03 - Get Status
    0xFF - Exit
 */
fn handle_packet(buffer: &[u8], mut stream: TcpStream) {
    println!("Received: {:?}", buffer);
    println!("Received: {:?}", buffer[0]);
    if buffer[0] == 0xFF {
        println!("Client exited");
        stream.shutdown(std::net::Shutdown::Both).unwrap();
        return;
    }
    if buffer[0] == 0x03 {
        println!("Get Status");
        let mut status = String::from("Status: ");
        status.push_str("Online");
        stream.write(status.as_bytes()).unwrap();
        stream.flush().unwrap();
        return;
    }
    if buffer[0] == 0x02 {
        println!("Received stay alive from {}", stream.peer_addr().unwrap());
        return;
    }

    let message = String::from_utf8_lossy(buffer);

    parse_command(&*message).and_then(|command_args| {
        command_handler(command_args);
        Ok(())
    }).unwrap_or_else(|e| {
        eprintln!("Error parsing command: {}", e);
    });
}