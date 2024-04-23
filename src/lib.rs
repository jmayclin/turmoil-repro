
pub const PORT: u16 = 9_999;
pub const GB_SEND: usize = 16;

#[cfg(test)]
mod tests {
    use std::net::{IpAddr, Ipv4Addr};
    use super::*;

    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use turmoil::{
        net::*,
        Builder,
    };

    #[test]
    fn network_partitions_during_connect() -> turmoil::Result {
        let mut sim = Builder::new().build();

        sim.host("server", || async {
            let listener = TcpListener::bind((IpAddr::from(Ipv4Addr::UNSPECIFIED), PORT)).await?;
            loop {
                let (mut stream, addr) = listener.accept().await?;
                // 1 Mb buffer
                let bytes = vec![3; 1_00_000];
                for gb in 0..GB_SEND {
                    let start = std::time::SystemTime::now();
                    for _ in 0..1_000 {
                        // write_all will panic?
                        //stream.write_all(&bytes).await?;
                    }
                    let elapsed = start.elapsed()?;
                    println!("gb {gb} send: {:?}", elapsed);
                }
            }
        });

        sim.client("client", async {
            let mut stream = TcpStream::connect(("server", PORT)).await?;
            let mut bytes = vec![0; 1_00_000];
            for gb in 0..GB_SEND {
                println!("reading gb {gb}");
                for i in 0..1_000 {
                    println!("reading {i}");
                    stream.read_exact(&mut bytes).await?;

                }
            }

            Ok(())
        });

        sim.run()
    }
}
