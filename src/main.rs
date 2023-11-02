// fn main() {
//     println!("Hello, world!");
// }

use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) {
    // ...
    let mut buffer = [0; 512];//[0;512] is a repeat expression
    stream.read(&mut buffer).unwrap();//将stream内容读取到buffer中
    println!("Request:{}",String::from_utf8_lossy(&buffer[..]));//调用from_utf8_lossy方法将buffer转为string并打印
    let content = fs::read_to_string("main.html").unwrap();
    let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", content);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:8080")?;//创建一个server
    // let listener = TcpListener::bind("127.0.0.1:80").unwrap();

    // accept connections and process them serially
    for stream in listener.incoming() {
        handle_client(stream?);
        // handle_client(stream.unwrap());
    }
    Ok(())
}