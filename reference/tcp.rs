// https://doc.rust-lang.org/src/std/net/tcp.rs.html
impl TcpListener {
  #[stable(feature = "rust1", since = "1.0.0")]
  pub fn bind<A: ToSocketAddrs>(addr: A) -> io::Result<TcpListener> {
    super::each_addr(addr, net_imp::TcpListener::bind).map(TcpListener)
  }

  #[stable(feature = "rust1", since = "1.0.0")]
  pub fn local_addr(&self) -> io::Result<SocketAddr> {
    self.0.socket_addr()
  }

  #[stable(feature = "rust1", since = "1.0.0")]
  pub fn incoming(&self) -> Incoming<'_> {
    Incoming { listener: self }
  }
}