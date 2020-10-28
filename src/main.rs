use async_std::net::TcpStream;
use async_std::prelude::*;
use bytes::{BufMut, BytesMut};
use std::error::Error;

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let stream = TcpStream::connect("127.0.0.1:6379").await?;
    let mut buf = [0u8; 1024];
    let mut resp = BytesMut::with_capacity(1024);

    let (mut reader, mut writer) = (&stream, &stream);
    // 向服务器发送 PING
    writer.write(b"*1\r\n$4\r\nPING\r\n").await?;
    let n = reader.read(&mut buf).await?;
    resp.put(&buf[0..n]);
    // 返回结果应该是 PONG
    println!("{:?}", resp);
    Ok(())
}
