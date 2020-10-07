use lazy_static::lazy_static;
use nix::sys::signal::{self, sigaction, SigAction, Signal};
use std::convert::TryFrom;
use std::io;
use std::sync::atomic::{AtomicUsize, Ordering};
lazy_static! {
  static ref SIGNALED: AtomicUsize = AtomicUsize::new(0);
}
extern "C" fn handle_signal(signal: libc::c_int) {
  match Signal::try_from(signal) {
    Ok(signal) => {
      SIGNALED.store(signal as usize, Ordering::Relaxed);
    }
    Err(_) => {
      exit_signal_safe(-1);
    }
  }
}
pub fn exit_signal_safe(status: i32) {
  unsafe {
    libc::_exit(status);
  }
}

pub fn take() -> Option<Signal> {
  let n = SIGNALED.swap(!0, Ordering::Relaxed);
  if n == !0 {
    None
  } else {
    match Signal::try_from(n as libc::c_int).ok() {
      Some(signal) => Some(signal),
      _ => None,
    }
  }
}
pub fn prepare() -> io::Result<()> {
  let sig_action = signal::SigAction::new(
    signal::SigHandler::Handler(handle_signal),
    signal::SaFlags::empty(),
    signal::SigSet::empty(),
  );

  let _ = unsafe { sigaction(Signal::SIGINT, &sig_action).unwrap() };
  let _ = unsafe { sigaction(Signal::SIGTSTP, &sig_action).unwrap() };
  let _ = unsafe { sigaction(Signal::SIGCONT, &sig_action).unwrap() };
  let _ = unsafe { sigaction(Signal::SIGQUIT, &sig_action).unwrap() };

  Ok(())
}
