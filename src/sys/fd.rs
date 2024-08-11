use std::cmp;
use std::io;
use std::os::fd::{AsRawFd, FromRawFd, RawFd};
use std::os::unix::io::OwnedFd;

use crate::sys::cvt;

// macosではないので、定数
const READ_LIMIT: usize = libc::ssize_t::MAX as usize;

pub struct FileDesc(OwnedFd);

impl FileDesc {
  pub fn read(&self, buf: &mut [u8]) -> io::Result<usize> {
    let ret = cvt(unsafe {
      libc::read(
        self.as_raw_fd(),
        buf.as_mut_ptr() as *mut libc::c_void,
        cmp::min(buf.len(), READ_LIMIT),
      )
    })?;
    Ok(ret as usize)
  }
}

impl AsRawFd for FileDesc {
  #[inline]
  fn as_raw_fd(&self) -> RawFd {
    self.0.as_raw_fd()
  }
}

impl FromRawFd for FileDesc {
  unsafe fn from_raw_fd(raw_fd: RawFd) -> Self {
    Self(FromRawFd::from_raw_fd(raw_fd))
  }
}
