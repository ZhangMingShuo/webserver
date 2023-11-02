# 用rust开发http服务端,用浏览器测试

## 参考TcpListener文档
https://doc.rust-lang.org/std/net/struct.TcpListener.html
```rs
use std::net::{TcpListener, TcpStream};

fn handle_client(stream: TcpStream) {
    // ...
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:80")?;

    // accept connections and process them serially
    for stream in listener.incoming() {
        handle_client(stream?);
    }
    Ok(())
}

```

## 修改handle_client函数实现读取stream并输出stream到控制台
```rs
fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();//将stream内容读取到buffer中
    println!("Request:{}",String::from_utf8_lossy(&buffer[..]))//调用from_utf8_lossy方法将buffer转为string并打印
}
```