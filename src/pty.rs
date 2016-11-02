use libc;

use {Errno, Result};
use std::os::unix::io::RawFd;

use sys::termios::Termios;

pub use libc::pid_t as SessionId;
pub use libc::winsize as Winsize;

pub struct OpenptyResult {
    pub master: RawFd,
    pub slave: RawFd,
}

/// Create a new pseudoterminal, returning the slave and master file descriptors
/// in `OpenptyResult`
/// (see [openpty](http://man7.org/linux/man-pages/man3/openpty.3.html)). 
///
/// If `winsize` is not `None`, the window size of the slave will be set to
/// the values in `winsize`. If `termios` is not `None`, the pseudoterminal's
/// terminal settings of the slave will be set to the values in `termios`.
#[inline]
pub fn openpty(winsize: Option<Winsize>, termios: Option<Termios>) -> Result<OpenptyResult> {
    let mut slave: libc::c_int = -1;
    let mut master: libc::c_int = -1;
    let c_termios = match &termios {
        &Some(ref termios) => termios as *const Termios,
        &None => 0 as *const Termios,
    };
    let c_winsize = match &winsize {
        &Some(ref ws) => ws as *const Winsize,
        &None => 0 as *const Winsize,
    };
    let ret = unsafe {
        libc::openpty(
            &mut master as *mut libc::c_int,
            &mut slave as *mut libc::c_int,
            0 as *mut libc::c_char,
            c_termios as *const libc::termios,
            c_winsize)
    };

    let _ = try!(Errno::result(ret));

    Ok(OpenptyResult {
        master: master,
        slave: slave,
    })
}
