use std::net::{ TcpListener, TcpStream };
use std::io::Read;
use rs_http::HttpMessage;
use rs_http::parse_http;

const MESSAGE_SIZE: usize = 8usize;

fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    let mut received: Vec<u8> = vec![];
    let mut bytes = [0u8; MESSAGE_SIZE];

    loop {
        let bytes_read = stream.read(&mut bytes)?;

        received.extend_from_slice(&bytes[0..bytes_read]);

        if bytes_read < MESSAGE_SIZE {
            break;
        }
    }

    println!("{:?}", received);

    let http_message: HttpMessage = parse_http(&received).unwrap();

    let message: String = String::from_utf8(received.clone()).unwrap();

    println!("{:?}", message);
    println!("{:?}", http_message);

    Ok(())
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    println!("Listening on 127.0.0.1:8080");

    // accept connections and process them serially
    for stream in listener.incoming() {
        handle_client(stream?)?;
    }

    Ok(())
}
