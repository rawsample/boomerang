/*
 * The tracer engine implements the instrumentation for tracing a system call.
 */
/*
 * The tracer engine takes care of handling syscalls.
 */
use std::{
    collections::HashMap,
    sync::{ Arc },
    io::{ self },
};
use nix::{
    libc::user_regs_struct,
};
use serde_json;
use crate::{
    syscall::{ Syscall },
    arch::{ TargetArch, Architecture },
    operation::Operation,
    tracer::{
        decoder::{ Decoder },
        filtering::{ Decision, Filter },
    },
    protocol::data::Client, memory,
};



pub struct TracerEngine {

    pub pid: i32,
    pub arch: Arc<Architecture>,
    //pub regs: Vec<u64>,
    pub regs: user_regs_struct,     // only for x86_64

    syscall: Syscall,
    remote_syscall: Syscall,
    insyscall: bool,
    filter: Filter,

    operator: Box<Operation>,
    decoder: Arc<Decoder>,
    protocol: Client,

    saved_syscall: Vec<Syscall>,
}

impl TracerEngine {

    pub fn new(
        pid: i32,
        target_arch: TargetArch,
        ipv4_address: &str,
        tracer_port: u16,
        executor_port: u16,
        operator: Box<Operation>,
    ) -> Self 
    {
        let arch = Arc::new(Architecture::new(target_arch));
        let decoder = Arc::new(Decoder::new(arch.clone()));

        Self {
            pid: pid,
            arch: arch,
            //regs: vec![0; 33],
            regs: user_regs_struct {
                r15: 0,
                r14: 0,
                r13: 0,
                r12: 0,
                rbp: 0,
                rbx: 0,
                r11: 0,
                r10: 0,
                r9: 0,
                r8: 0,
                rax: 0,
                rcx: 0,
                rdx: 0,
                rsi: 0,
                rdi: 0,
                orig_rax: 0,
                rip: 0,
                cs: 0,
                eflags: 0,
                rsp: 0,
                ss: 0,
                fs_base: 0,
                gs_base: 0,
                ds: 0,
                es: 0,
                fs: 0,
                gs: 0,
            },
            syscall: Syscall::new(),
            remote_syscall: Syscall::new(),
            insyscall: false,   // Hypothesis: we do the tracing from the start!
            filter: Filter::new(String::from("filtername")),
            operator: operator,
            decoder: decoder,
            protocol: Client::new(ipv4_address, tracer_port, executor_port),
            saved_syscall: Vec::new(),
        }
    }


    /*
     * When the tracking of the syscall entry/exit is left to the library,
     * we only synchronize the registers.
     */
    pub fn sync_registers(&mut self, regs: user_regs_struct) {
        self.regs = regs.clone();
    }

    pub fn trace(&mut self) -> Result<(), io::Error>
    {
        match self.insyscall {
            false    => {
                self.sync_entry();
                self.trace_entry();
            },

            true   => {
                self.sync_exit();
                self.trace_exit();
            },
        }
        Ok(())
    }

    fn sync_entry(&mut self) {
        self.syscall = Syscall::new();

        // Only for x86_64
        self.set_syscall_entry(self.regs.orig_rax,
                               self.regs.rdi,
                               self.regs.rsi,
                               self.regs.rdx,
                               self.regs.r10,
                               self.regs.r8,
                               self.regs.r9,
                               0,
        );
    }

    fn sync_exit(&mut self) {
        // Only for x86_64
        self.set_syscall_exit(self.regs.orig_rax, self.regs.rdx);
    }

    /*
     * The other way is to directly call the right method.
     */
    pub fn set_syscall_entry(&mut self, scno: u64, arg1: u64, 
                             arg2: u64, arg3: u64, arg4: u64,
                             arg5: u64, arg6: u64, arg7: u64) {
        // TODO: what about seccomp (see strace & PTRACE_GET_SYSCALL_INFO)
        self.syscall.raw.no = scno;
        self.syscall.raw.args[0] = arg1;
        self.syscall.raw.args[1] = arg2;
        self.syscall.raw.args[2] = arg3;
        self.syscall.raw.args[3] = arg4;
        self.syscall.raw.args[4] = arg5;
        self.syscall.raw.args[5] = arg6;
        self.syscall.raw.args[6] = arg7;
    }

    pub fn set_syscall_exit(&mut self, retval: u64, errno: u64) {
        self.syscall.raw.retval = retval;
        self.syscall.raw.errno = errno;
    }

    pub fn shutdown(&mut self) -> Result<(), io::Error>
    {
        // Calculate & print syscall statistics
        // syscall number | how many? | is_decoded? | name
        match self.calculate_stats() {
            Ok(syscall_stats) => {
                self.print_stats(syscall_stats);
            },
            Err(err) => {
                println!("Oops, something happens when calculating syscall statistics: {}", err);
            }
        }
        Ok(())
    }

    /* Tracing */

    fn trace_entry(&mut self) {
        //self._log_raw_entry();

        // TODO: Add an option to decode only certain syscalls to increase speed.
        self.decoder.decode_entry(&mut self.syscall, self.pid, &self.operator);

        self.filter_entry();
        self.log_entry();

        self.carry_out_entry_decision();

        self.insyscall = true;
    }

    fn trace_exit(&mut self) {
        //self._log_raw_exit();

        self.decoder.decode_exit();

        self.filter_exit();
        self.log_exit();

        self.carry_out_exit_decision();

        self.insyscall = false;
    }


    fn _log_raw_entry(&self) {
        println!("[{}] [ENTRY] no: {:#x} args: {:x?}", 
                 self.pid, self.syscall.raw.no as usize, self.syscall.raw.args)
    }

    fn _log_raw_exit(&self) {
        println!("[{}] [EXIT] retval: {:#x}", 
                 self.pid, self.syscall.raw.retval as usize)
    }

    fn log_entry(&self) {
        let json = serde_json::to_string(&self.syscall).unwrap();
        println!("[{}] {}", self.pid, json)
    }

    fn log_exit(&mut self) {
        let json = serde_json::to_string(&self.syscall).unwrap();
        println!("[{}] {}", self.pid, json);
        println!("");

        self.saved_syscall.push(self.syscall.clone());
    }

    /* Filtering */

    fn filter_entry(&mut self) -> Option<Decision> {
        self.syscall.decision = Some(self.filter.filter(&self.syscall));
        self.syscall.decision
    }

    fn filter_exit(&mut self) -> Option<Decision> {
        self.syscall.decision = Some(self.filter.filter(&self.syscall));
        self.syscall.decision
    }

    fn carry_out_entry_decision(&mut self)
    {
        //TODO: finish implementing the decisions
        match self.syscall.decision {
            Some(Decision::Continue) => (),
            Some(Decision::Forward) => {
                self.forward_entry().unwrap();
            },
            _ => panic!("Decision not implemented")
        }
    }

    fn carry_out_exit_decision(&mut self)
    {
        // TODO: finish implementing the decisions
        match self.syscall.decision {
            Some(Decision::Continue) => (),
            Some(Decision::Forward) => {
                self.forward_exit().unwrap();
            },
            _ => panic!("Decision not implemented")
        }

    }

    /* Forward decision */

    fn forward_entry(&mut self) -> Result<(), io::Error>
    {
        self.remote_syscall = self.protocol.send_syscall_entry(&self.syscall)?;
        println!("[{}] remote syscall retval: {:#x}", self.pid, self.remote_syscall.raw.retval as usize);
        Ok(())
    }

    fn forward_exit(&mut self) -> Result<(), io::Error>
    {
        // TODO
        //self.write_syscall_ret(self.remote_syscall.raw.retval, self.remote_syscall.raw.errno)?;
        Ok(())
    }

    /* Statistics */

    fn calculate_stats(&self) -> Result<HashMap<(u64, String), i32>, io::Error>
    {
        let mut syscall_stats: HashMap<(u64, String), i32> = HashMap::new();

        for syscall in &self.saved_syscall {
            let key = (syscall.raw.no.clone(), syscall.name.clone());
            let count = syscall_stats.entry(key).or_insert(0);
            *count += 1;
        }
        Ok(syscall_stats)
    }

    fn print_stats(&self, syscall_stats: HashMap<(u64, String), i32>)
    {
        //println!("{:?}", syscall_stats);

        println!("+-----+------------------+--------+");
        println!("| No  |       Name       | Number |");
        println!("+-----+------------------+--------+");

        for ((no, name), number) in syscall_stats {
            let name_str = if name.is_empty() { "<empty>" } else { &name };
            println!(
                "| {:<3} | {:<16} | {:<6} |",
                no, name_str, number
            );
        }
        println!("+-----+------------------+--------+");
    }



    /*
     * API to read/write from the environment by the interceptor "backend".
    pub fn read_registers(&self) -> Option<user_regs_struct> {
        self.operator.read_registers(self.pid)
    }

    //pub fn write_registers(&self, regs: user_regs_struct) -> Result<(), io::Error> {
    pub fn write_registers(&self, regs: user_regs_struct) -> bool {
        self.operator.write_registers(self.pid, regs)
    }

    pub fn read_memory(&self, addr: u64, size: u64) -> Vec<u8> {
        self.operator.read_memory(self.pid, addr, size)
    }

    pub fn write_memory(&self, addr: u64, mem: Vec<u8>) -> u64 {
        self.operator.write_memory(self.pid, addr, mem)
    }
     */

    /*
    pub fn read_syscall_args(&self) -> Vec<u64> {
        self.interceptor.read_syscall_args(self.pid)
    }

    pub fn write_syscall_args(&self, args: Vec<u64>) -> Result<(), io::Error> {
        self.interceptor.write_syscall_args(self.pid, args)
    }

    pub fn read_syscall_ret(&self) -> (u64, u64) {
        self.interceptor.read_syscall_ret(self.pid)
    }

    pub fn write_syscall_ret(&self, retval: u64, errno: u64) -> Result<(), io::Error> {
        self.interceptor.write_syscall_ret(self.pid, retval, errno)
    }
    */
}