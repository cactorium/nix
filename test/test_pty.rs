use nix::pty::openpty;
use nix::unistd::{read, write, close};

#[test]
fn test_openpty() {
    // TODO: figure out the right termios settings to pass in here
    let pty = openpty(None, None).unwrap();
    assert!(pty.master > 0); // must be valid file descriptors
    assert!(pty.slave > 0);

    // writing to one should be readable on the other one
    let string = "foofoofoo\n";
    let mut buf = [0u8; 16];
    write(pty.master, string.as_bytes()).unwrap();
    let len = read(pty.slave, &mut buf).unwrap();

    assert_eq!(len, string.len());
    assert_eq!(&buf[0..len], string.as_bytes());

    // read the echo as well
    let echoed_string = "foofoofoo\r\n";
    let len = read(pty.master, &mut buf).unwrap();
    assert_eq!(len, echoed_string.len());
    assert_eq!(&buf[0..len], echoed_string.as_bytes());

    let string2 = "barbarbarbar\n";
    let echoed_string2 = "barbarbarbar\r\n";
    write(pty.slave, string2.as_bytes()).unwrap();
    let len = read(pty.master, &mut buf).unwrap();

    assert_eq!(len, echoed_string2.len());
    assert_eq!(&buf[0..len], echoed_string2.as_bytes());

    close(pty.master).unwrap();
    close(pty.slave).unwrap();
}
