use std::os::unix::net::{UnixListener, UnixStream};
use std::io::{Read, Write};
use std::thread;
use std::fs;

enum DataSendType{
    Text,
    File,
}
struct Packect<T>{
    data_type: DataSendType,
    data: T,
    timestamp: u64,
}


mod g_packet{
    use super::*;
    //struct for buffer so can serialize buffer into something friendly
    //eg packet type, timestamp, data

    pub struct Data<T> {
        data: T,   
    }


    pub fn get_packet_type(input_buffer: &[u8]) -> DataSendType{
        let mut packet_type = DataSendType::Text;
        if input_buffer.len() > 0{
            if input_buffer[0] == 0{
                packet_type = DataSendType::Text;
            }else if input_buffer[0] == 1{
                packet_type = DataSendType::File;
            }
        }
        packet_type
    }
}

fn handle_client(mut stream: UnixStream) {
    let mut buffer = [0; 512];
    let bytes_read = stream.read(&mut buffer).unwrap();

    if(get_packet_type(&buffer[..bytes_read]) == DataSendType::Text){
        let message = String::from_utf8_lossy(&buffer[..bytes_read]);
        println!("Received message: {}", message);

    let message = String::from_utf8_lossy(&buffer[..bytes_read]);
    println!("Received message: {}", message);
    stream.write_all(&buffer[..bytes_read]).unwrap();
    // Stream will be closed when it goes out of scope
}

fn main() {
    let socket_path = "/tmp/rust-ipc.sock";

    // Remove the existing socket file if it exists
    if std::path::Path::new(socket_path).exists() {
        fs::remove_file(socket_path).unwrap();
    }

    let listener = UnixListener::bind(socket_path).unwrap();
    println!("Server listening on {}", socket_path);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_client(stream);
                    // Stream will be closed when the thread finishes
                });
            }
            Err(err) => {
                eprintln!("Connection failed: {}", err);
            }
        }
    }
}