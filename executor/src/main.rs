/*
 *
 */
use std::{
    collections::{ HashMap },
    os::unix::process::{ CommandExt },
    process::{ exit, Child, Command },
    thread::{ Builder, JoinHandle },
    sync::{ Arc, Barrier },
    net::{ Ipv4Addr },
};

use nix::{
    sys::{
        ptrace,
        wait::{ waitpid, WaitStatus},
        signal::Signal,
    },
    unistd::{ Pid },
};

use sysforward::{
    arch::TargetArch,
    memory::{ read_process_memory_maps, print_memory_regions },
    protocol::control::{ Configuration, ControlThread },
    executor_engine::Executor,
};


/* Static variable to change */
static IP_ADDRESS: &str = "127.0.0.1";
static CONTROL_PORT: u16 = 31000;
static TRACER_PORT: u16 = 31001;
static EXECUTOR_PORT: u16 = 31002;



/*
 *
 */
struct ExecDebugger {
    control_thread: ControlThread,
    handler_map: HashMap<Pid, Option<JoinHandle<()>>>,
    thread_map: HashMap<Pid, ExecThread>,
}

impl ExecDebugger {

    pub fn new() -> Self
    {
        // TODO: configure with ptrace?
        Self {
            control_thread: ControlThread::new(Configuration::Executor),
            handler_map: HashMap::new(),
            thread_map: HashMap::new(),
        }

    }

    pub fn run(&mut self)
    {
        let ip = Ipv4Addr::new(127, 0, 0, 1);
        let port: u16 = 31000;

        self.control_thread.listen(ip, port);
    }
}



/*
 *
 */
#[derive(Clone, Debug)]
struct ExecThread {
    boot_barrier: Arc<Barrier>,
}

impl ExecThread {

    fn boot_thread(
        &self, 
        tracee: Child,
        address_ipv4: &str,
        tracer_port: u16,
        executor_port: u16,
    )
    {
        println!("[EXECUTOR] Start listening on {}:{}", IP_ADDRESS, EXECUTOR_PORT);
        let mut executor = Executor::new(TargetArch::X86_64, address_ipv4, executor_port, tracer_port);

        let mem = read_process_memory_maps(tracee.id());
        print_memory_regions(&mem);

        self.run_thread(executor);
    }

    fn run_thread(&self, mut executor: Executor)
    {
        executor.run();
    }

}



fn main()
{
    /* TODO: add more argument to configure the executor:
     *  - etc.
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: ./executor <>");
        return;
    }
    */

    let mut dbg = ExecDebugger::new();

    // Listen for incomming connection and order from a trace dgb
    println!("[EXECUTOR] Start debugger...");
    dbg.run();

    println!("[EXECUTOR] Stop debugger.");
}