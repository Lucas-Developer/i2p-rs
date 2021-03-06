use std::io;
use std::net::{SocketAddr, ToSocketAddrs};

pub use self::addr::{I2pSocketAddr, ToI2pSocketAddrs};
pub use self::i2p::I2pAddr;
pub use self::streaming::{I2pStream, I2pListener};
pub use self::datagram::I2pDatagramSocket;

mod addr;
mod datagram;
mod i2p;
mod streaming;
#[cfg(test)]
mod test;

fn each_addr<A: ToSocketAddrs, B: ToI2pSocketAddrs, F, T>(sam_addr: A, addr: B, mut f: F) -> io::Result<T>
where
    F: FnMut(&SocketAddr, &I2pSocketAddr) -> io::Result<T>,
{
    let mut last_err = None;
    for addr in addr.to_socket_addrs()? {
        for sam_addr in sam_addr.to_socket_addrs()? {
            match f(&sam_addr, &addr) {
                Ok(l) => return Ok(l),
                Err(e) => last_err = Some(e),
            }
        }
    }
    Err(
        last_err.unwrap_or_else(
            || {
                io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "could not resolve to any addresses",
                )
            },
        ),
    )
}
