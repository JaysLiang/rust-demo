use std::io;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

fn handle_client(mut stream: TcpStream) {
    // tcp读出来得字节流直接拷贝到输出流中
    io::copy(&mut stream.try_clone().unwrap(), &mut stream);
}

fn main() {
    // tcp listen监听
    let listener = TcpListener::bind("0.0.0.0:10240").unwrap();
    // 等待客户端发起连接请求
    for stream in listener.incoming() {
        println!("tcp connection is built");
        // stream 为tcp connection 连接
        match stream {
            // 异常处理
            Err(e) => println!("failed: {}", e),
            // 正常处理
            Ok(stream) => {
                thread::spawn(move || handle_client(stream));
            }
        }
    }
}