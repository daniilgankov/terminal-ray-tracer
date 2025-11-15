use std::{
    io::{Read, stdin},
    mem::MaybeUninit,
    os::fd::AsRawFd,
    sync::mpsc::{Receiver, channel},
    thread::spawn,
};

use libc::{TCSANOW, cfmakeraw, tcgetattr, tcsetattr, termios};

pub(crate) struct Input {
    stdin_fd: i32,
    old_termios: termios,
    receiver: Receiver<char>,
}

impl Input {
    pub(crate) fn new() -> Self {
        let mut stdin = stdin();
        let stdin_fd = stdin.as_raw_fd();
        let mut old_termios: termios = unsafe { MaybeUninit::zeroed().assume_init() };
        unsafe { tcgetattr(stdin_fd, &mut old_termios as *mut _) };
        let mut new_termios = old_termios;
        unsafe { cfmakeraw(&mut new_termios as *mut _) };
        unsafe { tcsetattr(stdin_fd, TCSANOW, &new_termios as *const _) };
        let (sender, receiver) = channel();
        spawn(move || {
            loop {
                let mut buf = [0];
                stdin.read_exact(&mut buf).unwrap();
                let byte = buf[0];
                // Read only the first 128 bits of ASCII table and skip the later Unicode as
                // described in https://en.wikipedia.org/wiki/UTF-8#Description
                if byte >> 7 == 0 {
                    let bytes = [byte, 0, 0, 0];
                    let char = char::from_u32(u32::from_le_bytes(bytes)).unwrap();
                    sender.send(char).unwrap();
                } else if byte >> 5 == 0b110 {
                    stdin.read_exact(&mut [0]).unwrap();
                } else if byte >> 4 == 0b1110 {
                    stdin.read_exact(&mut [0; 2]).unwrap();
                } else if byte >> 3 == 0b11110 {
                    stdin.read_exact(&mut [0; 3]).unwrap();
                } else {
                    panic!("unknown Unicode sequence")
                }
            }
        });
        Self {
            stdin_fd,
            old_termios,
            receiver,
        }
    }

    pub(crate) fn pop(&mut self) -> Option<char> {
        self.receiver.try_recv().ok()
    }
}

impl Drop for Input {
    fn drop(&mut self) {
        unsafe { tcsetattr(self.stdin_fd, TCSANOW, &self.old_termios as *const _) };
    }
}
