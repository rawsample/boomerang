/*
 * 
 * int brk(void *addr);
 * void *sbrk(intptr_t increment);
 * void *mmap(void addr[.length], size_t length, int prot, int flags, int fd, off_t offset);
 * void *mremap(void old_address[.old_size], size_t old_size, size_t new_size, int flags, ... /* void *new_address */);
 * int munmap(void addr[.length], size_t length);
 * int mprotect(void addr[.len], size_t len, int prot);
 * int madvise(void addr[.length], size_t length, int advice);
 */
use serde::{ Serialize, Deserialize };

use crate::{
    syscall::{ RawSyscall },
    syscall::args::{ Direction, Integer, Fd, Size, Offset, Protection, Flag, Address },
    tracer::decoder::{ Decode },
    operation::{ Operation },
};



// int brk(void *addr);
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Brk{
    pub addr: Address,
}
impl Brk {
    pub fn new(raw: RawSyscall) -> Self {
        let addr = Address::new(raw.args[0], Direction::In);
        Self { addr }
    }
}
impl Decode for Brk {
    fn decode(&mut self, pid: i32, operation: &Box<Operation>) {
        self.addr.decode(pid, operation);
    }
}


// void *sbrk(intptr_t increment);
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Sbrk{
    pub increment: Integer,
}
impl Sbrk {
    pub fn new(raw: RawSyscall) -> Self {
        let increment = Integer::new(raw.args[0]);
        Self { increment }
    }
}
impl Decode for Sbrk {
    fn decode(&mut self, pid: i32, operation: &Box<Operation>) {
        self.increment.decode(pid, operation);
    }
}


// void *mmap(void addr[.length], size_t length, int prot, int flags, int fd, off_t offset);
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Mmap{
    pub addr: Address,
    pub length: Size,
    pub prot: Protection,
    pub flags: Flag,
    pub fd: Fd,
    pub offset: Offset,
}
impl Mmap {
    pub fn new(raw: RawSyscall) -> Self {
        let addr = Address::new(raw.args[0], Direction::In);
        let length = Size::new(raw.args[1]);
        let prot = Protection::new(raw.args[2]);
        let flags = Flag::new(raw.args[3]);
        let fd = Fd::new(raw.args[4]);
        let offset = Offset::new(raw.args[5]);
        Self { addr, length, prot, flags, fd, offset }
    }
}
impl Decode for Mmap {
    fn decode(&mut self, pid: i32, operation: &Box<Operation>) {
        self.addr.decode(pid, operation);
        self.length.decode(pid, operation);
        self.prot.decode(pid, operation);
        self.flags.decode(pid, operation);
        self.fd.decode(pid, operation);
        self.offset.decode(pid, operation);
    }
}


// void *mremap(void old_address[.old_size], size_t old_size, size_t new_size, int flags, ... /* void *new_address */);
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Mremap{
    pub old_address: Address,
    pub old_size: Size,
    pub new_size: Size,
    pub flags: Flag,
    pub new_address: Address,
}
impl Mremap {
    pub fn new(raw: RawSyscall) -> Self {
        let old_address = Address::new(raw.args[0], Direction::In);
        let old_size = Size::new(raw.args[1]);
        let new_size = Size::new(raw.args[2]);
        let flags = Flag::new(raw.args[3]);
        let new_address = Address::new(raw.args[4], Direction::In);
        Self { old_address, old_size, new_size, flags, new_address }
    }
}
impl Decode for Mremap {
    fn decode(&mut self, pid: i32, operation: &Box<Operation>) {
        self.old_address.decode(pid, operation);
        self.old_size.decode(pid, operation);
        self.new_size.decode(pid, operation);
        self.flags.decode(pid, operation);
        self.new_address.decode(pid, operation);
    }
}



// int munmap(void addr[.length], size_t length);
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Munmap{
    pub addr: Address,
    pub length: Size,
}
impl Munmap {
    pub fn new(raw: RawSyscall) -> Self {
        let addr = Address::new(raw.args[0], Direction::In);
        let length = Size::new(raw.args[1]);
        Self { addr, length }
    }
}
impl Decode for Munmap {
    fn decode(&mut self, pid: i32, operation: &Box<Operation>) {
        self.addr.decode(pid, operation);
        self.length.decode(pid, operation);
    }
}


// int mprotect(void addr[.len], size_t len, int prot);
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Mprotect{
    pub addr: Address,
    pub len: Size,
    pub prot: Protection,
}
impl Mprotect {
    pub fn new(raw: RawSyscall) -> Self {
        let addr = Address::new(raw.args[0], Direction::In);
        let len = Size::new(raw.args[1]);
        let prot = Protection::new(raw.args[2]);
        Self { addr, len, prot }
    }
}
impl Decode for Mprotect {
    fn decode(&mut self, pid: i32, operation: &Box<Operation>) {
        self.addr.decode(pid, operation);
        self.len.decode(pid, operation);
        self.prot.decode(pid, operation);
    }
}


// int madvise(void addr[.length], size_t length, int advice);
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Madvise{
    pub addr: Address,
    pub length: Size,
    pub advice: Integer,
}
impl Madvise {
    pub fn new(raw: RawSyscall) -> Self {
        let addr = Address::new(raw.args[0], Direction::In);
        let length = Size::new(raw.args[1]);
        let advice = Integer::new(raw.args[2]);
        Self { addr, length, advice }
    }
}
impl Decode for Madvise {
    fn decode(&mut self, pid: i32, operation: &Box<Operation>) {
        self.addr.decode(pid, operation);
        self.length.decode(pid, operation);
        self.advice.decode(pid, operation);
    }
}
