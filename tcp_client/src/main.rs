use std::fs::File;
use std::io::{Read, Write};
use std::net::TcpStream;

fn main() {
    // Open the file that will be sent. Adjust the filename as needed.
    let mut file = File::open(r"C:\Users\ThomasHo\Desktop\thomas-rust-file-manager-tut-and-training\trian\tcp_client\rust-程序设计语言-31-变量与可变性数据类型.mkv")
        .expect("Client: Could not open hi.txt");

    // Connect to the server.
    let mut stream = TcpStream::connect("192.168.68.110:9999")
        .expect("Client: Could not connect to server at 127.0.0.1:7878");
    println!("Client: Connected to server. Sending file...");

    let mut buffer = [0u8; 1024];
    // Read the file in chunks and send each chunk to the server.
    loop {
        let n = file.read(&mut buffer)
            .expect("Client: Failed to read from file");
        if n == 0 {
            // End of file reached.
            break;
        }
        stream.write_all(&buffer[..n])
            .expect("Client: Failed to send data to server");
    }

    println!("Client: File sent successfully.");
}