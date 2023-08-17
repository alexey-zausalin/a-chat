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
    while let Some(_stream) = incoming.next().await {
        todo!();
    }

    Ok(())
}
