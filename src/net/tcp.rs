// https://doc.rust-lang.org/src/std/net/tcp.rs.html
use std::io;
use std::net::{SocketAddr, ToSocketAddrs};

use crate::sys_common::net::TcpListener;

pub struct TcpListenerMyself(TcpListener);

pub struct Incoming<'a> {
  listener: &'a TcpListener,
}

impl TcpListenerMyself {
  pub fn bind<A: ToSocketAddrs>(addr: A) -> io::Result<TcpListenerMyself> {
    super::each_addr(addr, TcpListener::bind).map(TcpListenerMyself)
  }

  pub fn local_addr(&self) -> io::Result<SocketAddr> {
    self.0.socket_addr()
  }

  // pub fn incoming(&self) -> impl Iterator<Item = io::Result<std::net::TcpStream>> + '_ {
  //   self.0.incoming()
  // }

  pub fn incoming(&self) -> Incoming<'_> {
    Incoming { listener: self }
  }
}

pub trait IntoInner<Inner> {
  fn into_inner(self) -> Inner;
}

impl IntoInner<TcpListener> for TcpListener {
  fn into_inner(self) -> TcpListener {
    self.0
  }
}
