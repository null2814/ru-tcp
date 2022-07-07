use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    // 开启服务并监听7878端口
    let listener = TcpListener::bind("0.0.0.0:7878")?;

    // 循环连接
    for stream in listener.incoming() {
        // 处理连接
        handle_connection(stream?)
    }

    Ok(())
}

fn handle_connection(mut stream: TcpStream) {
    // 初始化用于从流中读取消息的缓冲区
    let mut buf = [0; 1024];

    // 读取流中消息到缓冲区
    stream.read(&mut buf).unwrap();

    // 将缓冲区消息转为字符串
    let request = String::from_utf8_lossy(&buf[..]).to_string();

    // 制作消息回复的内容
    let response = format!("get message: {}", request);

    // 将回复消息写入流缓冲区
    stream.write(response.as_bytes()).unwrap();
    // 将流写入缓冲区数据刷入响应
    stream.flush().unwrap();
}
