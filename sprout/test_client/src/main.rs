use std::os::unix::net::UnixStream;
use std::io::{Read, Write};

fn main() {
    let mut stream = UnixStream::connect("/tmp/rust-ipc.sock").unwrap();
    stream.write_all(b"Hello from client").unwrap();

    let mut response = String::new();
    stream.read_to_string(&mut response).unwrap();
    println!("Response: {}", response);
}