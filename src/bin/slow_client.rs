

use std::{error::Error, net::{IpAddr, Ipv4Addr}, time::Duration};

use tokio::{io::AsyncReadExt, net::*};
use turmoil_quadriatic::{GB_SEND, PORT};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut stream = TcpStream::connect(format!("localhost:{PORT}")).await?;
    // read only a single byte per second
    loop {
        stream.read_u8().await?;
        println!("read a byte");
        tokio::time::sleep(Duration::from_secs(1)).await;
    }


    Ok(())
}
