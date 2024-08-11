// https://doc.rust-lang.org/src/std/net/tcp.rs.html
use std::io;
use std::net::TcpListener;
use std::net::{SocketAddr, ToSocketAddrs};

pub struct TcpListenerMyself(TcpListener);

impl TcpListenerMyself {
  pub fn bind<A: ToSocketAddrs>(addr: A) -> io::Result<TcpListenerMyself> {
    TcpListener::bind(addr).map(TcpListenerMyself)
  }

  pub fn local_addr(&self) -> io::Result<SocketAddr> {
    self.0.local_addr()
  }

  pub fn incoming(&self) -> impl Iterator<Item = io::Result<std::net::TcpStream>> + '_ {
    self.0.incoming()
  }
}
