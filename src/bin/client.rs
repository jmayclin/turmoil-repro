

use std::{error::Error, net::{IpAddr, Ipv4Addr}};

use tokio::{io::AsyncReadExt, net::*};
use turmoil_quadriatic::{GB_SEND, PORT};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut stream = TcpStream::connect(format!("localhost:{PORT}")).await?;
    let mut bytes = vec![0; 1_000_000];
    for gb in 0..GB_SEND {
        println!("reading gb {gb}");
        for i in 0..1_000 {
            stream.read_exact(&mut bytes).await?;
        }
    }

    Ok(())
}
