use core::mem;
use libc::c_int;
use std::io;
use std::net::SocketAddr;

use crate::sys::cvt;
use crate::sys::net::Socket;

pub struct TcpListener {
  inner: Socket,
}

pub fn setsockopt<T>(sock: &Socket, level: c_int, option_name: c_int, option_value: T) -> io::Result<()> {
  unsafe {
    cvt(c::setsockopt(
      sock.as_raw(),
      level,
      option_name,
      core::ptr::addr_of!(option_value) as *const _,
      mem::size_of::<T>() as c::socklen_t,
    ))?;
    Ok(())
  }
}

impl TcpListener {
  pub fn bind(addr: io::Result<&SocketAddr>) -> io::Result<TcpListener> {
    let addr = addr?;

    init();

    let sock = Socket::new(addr, c::SOCK_STREAM)?;

    setsockopt(&sock, c::SOL_SOCKET, c::SO_REUSEADDR, 1 as c_int)?;

    // Bind our new socket
    let (addr, len) = addr.into_inner();
    cvt(unsafe { c::bind(sock.as_raw(), addr.as_ptr(), len as _) })?;

    let backlog = 128;

    // Start listening
    cvt(unsafe { c::listen(sock.as_raw(), backlog) })?;
    Ok(TcpListener { inner: sock })
  }
}
