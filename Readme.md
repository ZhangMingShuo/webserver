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