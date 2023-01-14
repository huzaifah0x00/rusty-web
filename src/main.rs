use std::{
    io::{Read, Write},
    net::TcpStream,
};

fn main() {
    println!("Hello, world!");

    let mut stream = TcpStream::connect("127.0.0.1:8000").expect("Something went wrong");

    stream.write(b"GET /\n\n\n").ok();

    let mut response = String::from("");
    for byte in stream.bytes() {
        let char = byte.unwrap();
        response.push(char::from(char));
    }

    println!("{}", response);
}
