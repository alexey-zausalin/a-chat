use async_std::{io::BufReader, net::TcpStream};
use async_std::{
    net::{TcpListener, ToSocketAddrs},
    prelude::*,
    task,
};

fn main() -> Result<()> {
    let fut = accept_loop("127.0.0.1:8000");
    task::block_on(fut)
}

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

async fn accept_loop(addr: impl ToSocketAddrs) -> Result<()> {
    let listener = TcpListener::bind(addr).await?;
    let mut incoming = listener.incoming();
    while let Some(stream) = incoming.next().await {
        let stream = stream?;
        println!("Accepting from {}", stream.peer_addr()?);
        let _handle = task::spawn(connection_loop(stream));
    }

    Ok(())
}

async fn connection_loop(stream: TcpStream) -> Result<()> {
    let reader = BufReader::new(&stream);
    let mut lines = reader.lines();

    let name = match lines.next().await {
        Some(line) => line?,
        None => Err("peer disconnected immediately")?,
    };
    println!("name: {}", name);

    while let Some(line) = lines.next().await {
        let line = line?;
        let (dest, msg) = match line.find(':') {
            Some(idx) => (&line[..idx], line[idx + 1..].trim()),
            None => continue,
        };

        let _dest: Vec<String> = dest
            .split(',')
            .map(|name| name.trim().to_string())
            .collect();
        let _msg: String = msg.to_string();
    }

    Ok(())
}
