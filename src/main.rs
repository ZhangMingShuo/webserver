// fn main() {
//     println!("Hello, world!");
// }

use std::io::Read;
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) {
    // ...
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();//将stream内容读取到buffer中
    println!("Request:{}",String::from_utf8_lossy(&buffer[..]))//调用from_utf8_lossy方法将buffer转为string并打印
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:80")?;//创建一个server
    // let listener = TcpListener::bind("127.0.0.1:80").unwrap();

    // accept connections and process them serially
    for stream in listener.incoming() {
        handle_client(stream?);
        // handle_client(stream.unwrap());
    }
    Ok(())
}