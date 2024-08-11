use libc::c_int;
use std::io;
use std::net::SocketAddr;
use std::os::fd::FromRawFd;

use crate::sys::cvt;
use crate::sys::fd::FileDesc;

pub struct Socket(FileDesc);

impl Socket {
  pub fn new(addr: &SocketAddr, ty: c_int) -> io::Result<Socket> {
    let fam = match *addr {
      SocketAddr::V4(..) => libc::AF_INET,
      SocketAddr::V6(..) => libc::AF_INET6,
    };
    Socket::new_raw(fam, ty)
  }

  pub fn new_raw(fam: c_int, ty: c_int) -> io::Result<Socket> {
    unsafe {
      // 自分の環境がLinuxであるため、SOCK_CLOEXECを指定している
      let fd = cvt(libc::socket(fam, ty | libc::SOCK_CLOEXEC, 0))?;
      Ok(Socket(FileDesc::from_raw_fd(fd)))
    }
  }
}
