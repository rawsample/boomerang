/*
 * TODO: use eBPF to filter syscall and output the decision.
 */
use crate::tracer_engine::Syscall;



#[derive(Clone, Copy, Debug)]
pub enum Decision {
    Continue = 0,
    FwdEntry,
    FwdExit,
    InspectExit,
    LogLocal,
    NoExec,
    Kill,
}

pub struct Filter {
    pub name: String,
    pub decision: Decision,
}

impl Filter {
    pub fn new(name: String) -> Filter {
        Filter {
            name: name,
            decision: Decision::Continue,
        }
    }

    pub fn filter(&self, _syscall: &Syscall) -> Decision {
        self.decision
    }
}
