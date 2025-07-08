use std::fs::File;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

/// Handles an individual client connection by reading the file data
/// sent by the client and writing it to "received_file.txt".
fn handle_client(mut stream: TcpStream) {
    // Create (or overwrite) the file to store the received data.
    let mut file = match File::create("tsmkv.mkv") {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Server: Failed to create file: {}", e);
            return;
        }
    };

    let mut buffer = [0u8; 512];
    println!("Server: Receiving file from {}", stream.peer_addr().unwrap());
    
    // Read data from the stream until EOF (client closes connection).
    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                // End of file.
                println!("Server: Finished receiving file.");
                break;
            }
            Ok(n) => {
                if let Err(e) = file.write_all(&buffer[..n]) {
                    eprintln!("Server: Failed to write to file: {}", e);
                    break;
                }
            }
            Err(e) => {
                eprintln!("Server: Error reading from stream: {}", e);
                break;
            }
        }
    }
}

fn main() {
    // Bind the TCP listener to an address.
    let listener = TcpListener::bind("192.168.68.109:9999")
        .expect("Server: Could not bind to 127.0.0.1:7878");
    println!("Server: Listening on 127.0.0.1:7878...");

    // Accept and handle incoming connections.
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Server: New client connected.");
                thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => {
                eprintln!("Server: Connection failed: {}", e);
            }
        }
    }
}