/*
 *
 */
use std::collections::HashMap;

//use crate::arch::x86_64;

mod x86_64;



#[derive(Clone, Copy)]
pub enum TargetArch {
    Arm,
    Aarch64,
    Mipso32,
    Mipsn32,
    Mipsn64,
    Powerpc,
    Riscv,
    X86,
    X86_64,
}


pub struct Architecture {
    name: TargetArch,
    //register_table: Register,
    syscall_table: SyscallTable,
}

impl Architecture {
    pub fn new(name: TargetArch) -> Self {
        Self {
            name: name,
            //register_table: Register::new(&name),
            syscall_table: SyscallTable::new(&name),
        }
    }
}


/*
pub(crate) struct Register {
    map: HashMap<&'static str, u8>,
}

impl Register {
    pub fn new(arch: &TargetArch) -> Self {
        Self {
            map: match arch {
                TargetArch::X86_64  => x86_64::create_register_table(),
                //TargetArch::X86     => x86::create_register_table(),
                //TargetArch::Arm     => arm::create_register_table(),
                //TargetArch::Aarch64 => aarch64::create_register_table(),
                //TargetArch::Mipso32 => mipso32::create_register_table(),
                //TargetArch::Mipsn32 => mipsn32::create_register_table(),
                //TargetArch::Mipsn64 => mipsn64::create_register_table(),
                //TargetArch::Powerpc => powerpc::create_register_table(),
                //TargetArch::Riscv32 => riscv32::create_register_table(),
                //TargetArch::Riscv64 => riscv64::create_register_table(),
                _ => panic!("Architecture not implemented"),
            },
        }
    }

    pub fn get_no(&self, name: &str) -> Option<u8> {
        self.map.get(name).copied()
    }

    pub fn get_name(&self, no: &u64) -> Option<str> {
        self.map.iter().find_map(|(key, &val)| if val == no { Some(no) } else { None })
    }
}
*/


pub(crate) struct SyscallTable {
    map: HashMap<&'static str, u64>,
}

impl SyscallTable {
    pub fn new(arch: &TargetArch) -> Self {
        Self {
            map: match arch {
                TargetArch::X86_64  => x86_64::create_syscall_table(),
                /*
                TargetArch::X86     => x86::create_syscall_table(),
                TargetArch::Arm     => arm::create_syscall_table(),
                TargetArch::Aarch64 => aarch64::create_syscall_table(),
                TargetArch::Mipso32 => mipso32::create_syscall_table(),
                TargetArch::Mipsn32 => mipsn32::create_syscall_table(),
                TargetArch::Mipsn64 => mipsn64::create_syscall_table(),
                TargetArch::Powerpc => powerpc::create_syscall_table(),
                TargetArch::Riscv32 => riscv32::create_syscall_table(),
                TargetArch::Riscv64 => riscv64::create_syscall_table(),
                */
                _ => panic!("Architecture not implemented"),
            },
        }
    }

    pub fn get_syscall_no(&self, name: &str) -> Option<u64> {
        self.map.get(name).copied()
    }

    /*
    pub fn get_syscall_name(&self, no: &u64) -> Option<str> {
        self.map.iter().find_map(|(key, &val)| if val == no { Some(no) } else { None })
    }
    */
}