use libc::{c_int, c_ushort, ioctl, TIOCGWINSZ};
use std::io;
use std::mem::zeroed;

pub fn get_winsize(fd: c_int) -> io::Result<Winsize> {
  let mut winsz: Winsize = unsafe { zeroed() };

  let res = unsafe { ioctl(fd, TIOCGWINSZ, &mut winsz) };

  if res == -1 {
    Err(io::Error::last_os_error())
  } else {
    Ok(winsz)
  }
}

#[repr(C)]
#[derive(Debug)]
pub struct Winsize {
  pub ws_row: c_ushort,
  pub ws_col: c_ushort,
  pub ws_xpixel: c_ushort,
  pub ws_ypixel: c_ushort,
}
