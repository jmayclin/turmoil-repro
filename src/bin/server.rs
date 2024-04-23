use std::{error::Error, net::{IpAddr, Ipv4Addr}};

use tokio::{io::AsyncWriteExt, net::*};
use turmoil_quadriatic::{GB_SEND, PORT};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind((IpAddr::from(Ipv4Addr::UNSPECIFIED), PORT)).await?;
    loop {
        let (mut stream, addr) = listener.accept().await?;
        // 1 Mb buffer
        let bytes = vec![3; 1_000_000];
        for gb in 0..GB_SEND {
            let start = std::time::SystemTime::now();
            for _ in 0..1_000 {
                // write_all will panic?
                stream.write_all(&bytes).await?;
            }
            let elapsed = start.elapsed()?;
            println!("gb {gb} send: {:?}", elapsed);
        }
    }
}
