use unistd;
use libc::{self, c_int};
use std::os::unix::io::{RawFd, AsRawFd};
use {Errno, Result, NixPath};

bitflags!{
    #[repr(C)]
    flags InotifyCreateFlags: c_int {
        const IN_NONBLOCK = libc::O_NONBLOCK,
        const IN_CLOEXEC = libc::O_CLOEXEC
    }
}

bitflags!(
    #[repr(C)]
    flags InotifyEventMask: u32 {
        const IN_ACCESS = 0x001,
        const IN_MODIFY = 0x002,
        const IN_ATTRIB = 0x004,
        const IN_CLOSE_WRITE = 0x008,
        const IN_CLOSE_NOWRITE = 0x010,
        const IN_OPEN = 0x020,
        const IN_MOVED_FROM = 0x040,
        const IN_MOVED_TO = 0x080,
        const IN_CREATE = 0x100,
        const IN_DELETE = 0x200,
        const IN_DELETE_SELF = 0x400,
        const IN_MOVE_SELF = 0x800,

        const IN_UNMOUNT = 0x2000,
        const IN_Q_OVERFLOW = 0x4000,
        const IN_IGNORED = 0x8000,
    }
);

#[derive(Clone, Copy)]
#[repr(C)]
pub struct InotifyEvent {
    wd: i32,
    mask: u32,
    cookie: u32,
    len: u32,
    name: *const char
}

impl InotifyEvent {
    pub fn wd(&self) -> WatchFd {
        WatchFd(self.wd)
    }
    pub fn mask(&self) -> InotifyEventMask {
        InotifyEventMask::from_bits_truncate(self.mask)
    }
    pub fn cookie(&self) -> u32 {
        self.cookie
    }
    pub fn filename(&self) -> &[u8] {
        use std::slice;
        unsafe { slice::from_raw_parts(self.name as *const u8, self.len as usize) }
    }
}

mod ffi {
    use libc::{c_int, c_char};

    extern {
        pub fn inotify_init1(flags: c_int) -> c_int;
        pub fn inotify_add_watch(fd: c_int, pathname: *const c_char, mask: u32) -> c_int;
        pub fn inotify_rm_watch(fd: c_int, wd: c_int) -> c_int;
    }
}

#[inline]
pub fn inotify_init1(flags: InotifyCreateFlags) -> Result<RawFd> {
    let res = unsafe { ffi::inotify_init1(flags.bits()) };
    Errno::result(res).map(|r| r as RawFd)
}

#[inline]
pub fn inotify_add<P: ?Sized + NixPath>(fd: RawFd, pathname: &P, mask: InotifyEventMask) -> Result<c_int> {
    pathname.with_nix_path(|p| {
        let res = unsafe { ffi::inotify_add_watch(fd as c_int, p.as_ptr(), mask.bits()) };
        Errno::result(res)
    })
    // unwrap the inner Result
    .and_then(|r| r)
}

#[inline]
pub fn inotify_rm(fd: RawFd, wd: WatchFd) -> Result<()> {
    let res = unsafe { ffi::inotify_rm_watch(fd as c_int, wd.0 as c_int) };
    Errno::result(res).map(|_| ())
}

pub struct InotifyFd(RawFd);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct WatchFd(c_int);

impl InotifyFd {
    pub fn new(flags: InotifyCreateFlags) -> Result<InotifyFd> {
        let fd = try!(inotify_init1(flags));
        Ok(InotifyFd(fd))
    }

    pub fn add<P: ?Sized + NixPath>(&self, path: &P, mask: InotifyEventMask) -> Result<WatchFd> {
        inotify_add(self.0, path, mask).map(|wd| WatchFd(wd))
    }

    pub fn rm(&self, wd: WatchFd) -> Result<()> {
        inotify_rm(self.0, wd)
    }
}

impl Drop for InotifyFd {
    fn drop(&mut self) {
        let _ = unistd::close(self.0);
    }
}

impl AsRawFd for InotifyFd {
    fn as_raw_fd(&self) -> RawFd {
        self.0
    }
}

