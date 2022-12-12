use std::io::{Read, Write};
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("accepted new connection");

                loop {
                    let mut buf = [0; 512];
                    let bytes_read = stream.read(&mut buf).unwrap();
                    if bytes_read == 0 {
                        println!("client closed the connection");
                        break;
                    }
                    
                    stream.write("+PONG\r\n".as_bytes()).unwrap();
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
