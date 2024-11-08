/*
 */
use serde::{ Serialize, Deserialize };

use decoding_macro::DecodeExit;
use crate::{
    syscall::RawSyscall,
    syscall::args::{ Direction, Integer, Fd, Flag, Address, NullBuffer, Struct },
    syscall::decoder::{ DecodeArg, DecodeEntry, DecodeExit },
    targets::operation::Operation,
};




// int name_to_handle_at(int dirfd, const char *pathname, struct file_handle *handle, int *mount_id, int flags)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
#[derive(DecodeExit)]
pub struct NameToHandleAt {
    pub dirfd: Fd,
    pub pathname: NullBuffer,
    pub handle: Struct,
    pub mount_id: Address,
    pub flags: Flag,
    pub retval: Option<Integer>,
}
impl NameToHandleAt {
    pub fn new(raw: RawSyscall) -> Self {
        let dirfd = Fd::new(raw.args[0]);
        let pathname = NullBuffer::new(raw.args[1], Direction::In);
        let handle = Struct::new(raw.args[2], Direction::InOut);
        let mount_id = Address::new(raw.args[3], Direction::InOut);
        let flags = Flag::new(raw.args[4]);
        let retval = None;
        Self { dirfd, pathname, handle, mount_id, flags, retval }
    }
}
impl DecodeEntry for NameToHandleAt {
    fn decode_entry(&mut self, pid: i32, operation: &Box<Operation>) {
        self.dirfd.decode(pid, operation).unwrap();
        self.pathname.decode(pid, operation).unwrap();
        self.handle.decode(pid, operation).unwrap();
        self.mount_id.decode(pid, operation).unwrap();
        self.flags.decode(pid, operation).unwrap();
    }
}

// int open_by_handle_at(int mount_fd, struct file_handle *handle, int flags)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
#[derive(DecodeExit)]
pub struct OpenByHandleAt {
    pub mount_fd: Fd,
    pub handle: Struct,
    pub flags: Flag,
    pub retval: Option<Integer>,
}
impl OpenByHandleAt {
    pub fn new(raw: RawSyscall) -> Self {
        let mount_fd = Fd::new(raw.args[0]);
        let handle = Struct::new(raw.args[1], Direction::InOut);
        let flags = Flag::new(raw.args[2]);
        let retval = None;
        Self { mount_fd, handle, flags, retval }
    }
}
impl DecodeEntry for OpenByHandleAt {
    fn decode_entry(&mut self, pid: i32, operation: &Box<Operation>) {
        self.mount_fd.decode(pid, operation).unwrap();
        self.handle.decode(pid, operation).unwrap();
        self.flags.decode(pid, operation).unwrap();
    }
}