use nix::sys::inotify::*;

use tempfile::NamedTempFile;

#[cfg(target_os = "linux")]
#[test]
fn test_inotify() {
    let ifd = InotifyFd::new(InotifyCreateFlags::empty()).unwrap();
    {
        let mut tmp = NamedTempFile::new().unwrap();

        ifd.add(tmp.path(), IN_DELETE).unwrap();
    }
}
