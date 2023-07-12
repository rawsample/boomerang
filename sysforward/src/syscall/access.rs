/*
 *
 */
use serde::{ Serialize, Deserialize };
use crate::{
    syscall::{ RawSyscall },
    syscall::args::{ Direction, Integer, Fd, Flag, NullBuffer },
    //syscall::args::{ Integer, Fd, Size, Flag, Buffer, NullBuffer, Struct },
    tracer::decoder::{ DecodeArg, DecodeEntry, DecodeExit },
    operation::{ Operation },
};



// int access(const char *pathname, int mode)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Access {
    pub pathname: NullBuffer,
    pub mode: Integer,
}
impl Access {
    pub fn new(raw: RawSyscall) -> Self {
        let pathname = NullBuffer::new(raw.args[0], Direction::In);
        let mode = Integer::new(raw.args[1]);
        Self { pathname, mode }
    }
}
impl DecodeEntry for Access {
    fn decode_entry(&mut self, pid: i32, operation: &Box<Operation>) {
        self.pathname.decode(pid, operation);
        self.mode.decode(pid, operation);
    }
}


// int faccessat(int dirfd, const char *pathname, int mode, int flags)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Faccessat {
    pub dirfd: Fd,
    pub pathname: NullBuffer,
    pub mode: Integer,
    pub flags: Flag,
}
impl Faccessat {
    pub fn new(raw: RawSyscall) -> Self {
        let dirfd = Fd::new(raw.args[0]);
        let pathname = NullBuffer::new(raw.args[1], Direction::In);
        let mode = Integer::new(raw.args[2]);
        let flags = Flag::new(raw.args[3]);
        Self { dirfd, pathname, mode, flags }
    }
}
impl DecodeEntry for Faccessat {
    fn decode_entry(&mut self, pid: i32, operation: &Box<Operation>) {
        self.dirfd.decode(pid, operation);
        self.pathname.decode(pid, operation);
        self.mode.decode(pid, operation);
        self.flags.decode(pid, operation);
    }
}


// int syscall(SYS_faccessat2, int dirfd, const char *pathname, int mode, int flags)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Faccessat2 {
    pub dirfd: Fd,
    pub pathname: NullBuffer,
    pub mode: Integer,
    pub flags: Flag,
}
impl Faccessat2 {
    pub fn new(raw: RawSyscall) -> Self {
        let dirfd = Fd::new(raw.args[0]);
        let pathname = NullBuffer::new(raw.args[1], Direction::In);
        let mode = Integer::new(raw.args[2]);
        let flags = Flag::new(raw.args[3]);
        Self { dirfd, pathname, mode, flags }
    }
}
impl DecodeEntry for Faccessat2 {
    fn decode_entry(&mut self, pid: i32, operation: &Box<Operation>) {
        self.dirfd.decode(pid, operation);
        self.pathname.decode(pid, operation);
        self.mode.decode(pid, operation);
        self.flags.decode(pid, operation);
    }
}
