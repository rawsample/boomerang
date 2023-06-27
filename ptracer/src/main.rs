/*
 * Example to use libsysforward with ptrace.
 * Works with an executor instance.
 */
use std::{
    collections::{ HashMap },
    thread::{ Builder, JoinHandle },
    os::unix::process::{ CommandExt },
    process::{ exit, Child, Command },
    sync::{ Arc, Barrier },
    io::{self, prelude::*, BufReader, BufWriter },
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
    protocol::control::{ Configuration, ControlChannel },
    tracer_engine::{ TracerCallback, TracerEngine },
};



/* Static variable to change */
static IP_ADDRESS: &str = "127.0.0.1";
//static CONTROL_PORT: u16 = 31000;
static TRACER_PORT: u16 = 32000;
static EXECUTOR_PORT: u16 = 32001;



/*
 * The debugger is the high-level structure which manage the tracing threads and connection with the executor.
 */
struct TraceDebugger {
    control_channel: ControlChannel,
}


impl TraceDebugger {

    pub fn new() -> Self
    {
        // TODO: configure with ptrace?

        /* HERE !!!
         * The idea is to be able to call from the control channel some functions from the debugger,
         * such as spawn_process, kill_process, start_tracing, read_mem, write_regs, set_breakpoint, etc.
         * 
         * For that, we need a callback mechanisms to "register" or refer to the right function within the 
         * control_channel object.
         * 
         * We could use:
         *      1. function pointers
         *      2. callback with closure
         *      3. Trait
         *      4. Rc<RefCell<>>
         * 
         */
        Self {
            control_channel: ControlChannel::new(Configuration::Tracer, Some(Box::new(TraceDebuggerCallback::new())), None)
        }
    }

    pub fn run(&mut self)
    {
        let ip = Ipv4Addr::new(127, 0, 0, 1);
        let port: u16 = 31000;

        //self.control_channel.connect(ip, port).unwrap();

        self.control_channel.listen(ip, port);
    }
}

impl Default for TraceDebugger {
    fn default() -> Self {
        Self::new()
    }
}



struct TraceDebuggerCallback {
    handler_map: HashMap<Pid, Option<JoinHandle<()>>>,
    thread_map: HashMap<Pid, TracingThread>,
}

impl TraceDebuggerCallback {
    pub fn new() -> Self {
        Self {
            handler_map: HashMap::new(),
            thread_map: HashMap::new(),
        }
    }

}

impl TracerCallback for TraceDebuggerCallback {

    fn spawn_process(&mut self, program: &str, prog_args: &[&str]) -> Result<Pid, io::Error>
    {
        //println!("* Spawn process: {} {:?} *", program, prog_args);
        //Ok(Pid::from_raw(10))

        /* TODO Verify the connection with executor
        if ! self.check_connected() {
            return Err(io::Error::new(io::ErrorKind::Other, "Not connected with executor"));
        }
        */

        // Spawn the child
        println!("[TRACER] Spawn {} {:?}", program, prog_args);
        let child: Child = unsafe {
            let mut command = Command::new(program);
            command.args(prog_args);
            command.pre_exec(|| {
                ptrace::traceme().unwrap();
                Ok(())
            });

            command.spawn().expect("Failed to spawn child process")
        };

        let pid = Pid::from_raw(child.id() as i32);

        // Wait for first syscall
        match waitpid(pid, None) {
            Ok(WaitStatus::Stopped(_, Signal::SIGTRAP)) => { /* ??? */ },
            _ => panic!("WaitStatus not handled"),
        };

        // Create the tracing thread
        let boot_barrier = Arc::new(Barrier::new(2));

        let tracing_thread = TracingThread { boot_barrier };
        let copy_tracing_thread = tracing_thread.clone();
        self.thread_map.insert(pid, tracing_thread);

        let builder = Builder::new().name(child.id().to_string());
        let handler = builder.spawn(move ||
            copy_tracing_thread.boot_thread(child)
        ).unwrap();
        self.handler_map.insert(pid, Some(handler));

        // Notify the executor
        //self.notify_new_process();

        Ok(pid)
    }

    fn kill_process(&mut self, pid: Pid) -> Result<(), io::Error>
    {
        println!("* Kill process {:?} *", pid);
        
        match self.handler_map.get_mut(&pid) {
            Some(handler) => {
                match handler.take() {
                    Some(thread) => {
                        println!("killing...");
                        ptrace::kill(pid).unwrap();
                        println!("joining...");
                        // BUG: The start_tracing should be executed before otherwise the barrier deadlocks the join()
                        thread.join().unwrap();
                        println!("finished!");
                    },
                    None => {
                        // Error
                    }
                }
            },
            None => {
                // Error
            }
        }

        Ok(())
    }

    fn start_tracing(&mut self, pid: Pid) -> Result<(), io::Error>
    {
        println!("* Trace process {:?} *", pid);

        match self.thread_map.get_mut(&pid) {
            Some(thread) => {
                thread.boot_barrier.wait();
            },

            None => {
                //Error
            }
        }

        Ok(())
    }

    fn stop_tracing(&mut self, pid: Pid) -> Result<(), io::Error>
    {
        println!("* Stop process {:?} *", pid);
        Ok(())
    }





    /*
    fn check_connected(&self) -> bool
    {
        if self.reader.is_none() || self.writer.is_none() {
            return false;
        } else {
            return true;
        }
    }

    pub fn spawn(&mut self, program: &str, prog_args: &[String]) -> Result<Pid, io::Error>
    {
        // Verify the connection with executor
        if ! self.check_connected() {
            return Err(io::Error::new(io::ErrorKind::Other, "Not connected with executor"));
        }

        // Spawn the child
        println!("[TRACER] Spawn {} {:?}", program, prog_args);
        let child: Child = unsafe {
            let mut command = Command::new(program);
            command.args(prog_args);
            command.pre_exec(|| {
                ptrace::traceme().unwrap();
                Ok(())
            });

            command.spawn().expect("Failed to spawn child process")
        };

        let pid = Pid::from_raw(child.id() as i32);

        // Wait for first syscall
        match waitpid(pid, None) {
            Ok(WaitStatus::Stopped(_, Signal::SIGTRAP)) => { /* ??? */ },
            _ => panic!("WaitStatus not handled"),
        };

        // Create the tracing thread
        let boot_barrier = Arc::new(Barrier::new(2));

        let tracing_thread = TracingThread { boot_barrier };
        let copy_tracing_thread = tracing_thread.clone();
        self.thread_map.insert(pid, tracing_thread);

        let builder = Builder::new().name(child.id().to_string());
        let handler = builder.spawn(move ||
            copy_tracing_thread.boot_thread(child)
        ).unwrap();
        self.handler_map.insert(pid, Some(handler));

        // Notify the executor
        self.notify_new_process();

        Ok(pid)
    }

    fn notify_new_process(&self)
    {
        // TODO: the port depends on how many thread has been spawn
        let payload = control::NewProcessRequestPayload { 
            address_ipv4: IP_ADDRESS,
            tracer_port: TRACER_PORT,
            executor_port: EXECUTOR_PORT,
        };

        let payload = serde_json::to_string(&payload).unwrap();

        let message = control::Message {
            command: control::Command::NewProcess,
            payload: payload,
        };

        let mut data = serde_json::to_string(&message).unwrap();
        data.push_str("\n");
        println!("[TRACER] Send message: {}", data);

        let _ret = self.writer.expect("No TcpStream Writer found").write(data.as_bytes());
        self.writer.expect("No TcpStream Writer found").flush();
    }

    
    pub fn start_tracing_thread(&self, pid: Pid)
    {
        let thread = self.thread_map.get(&pid).unwrap();
        thread.boot_barrier.wait();
    }

    pub fn join_tracing_thread(&mut self, pid: Pid)
    {
        if let Some(handler) = self.handler_map.get_mut(&pid) {
            if let Some(thread) = handler.take() {
                thread.join().unwrap();
            }
        }
    }
    */

}

/*
 * Represent a thread tracing the execution of a child thread.
 */
#[derive(Clone, Debug)]
struct TracingThread {
    boot_barrier: Arc<Barrier>,
    // Use condvar instead maybe ?
 }


impl TracingThread {

    /*
     * Small code given to a newly spawn tracing thread for waiting to start tracing.
     */
    fn boot_thread(&self, tracee: Child)
    {
        // Setup the tracer
        let tracer = TracerEngine::new(tracee.id() as i32, TargetArch::X86_64, IP_ADDRESS, TRACER_PORT, EXECUTOR_PORT);
        let tracee_pid = tracee.id();
        
        // Wait for the main thread to start tracing
        self.boot_barrier.wait();
        self.run_thread(tracee, tracer);
        self.shutdown_thread(tracee_pid);
    }

    fn shutdown_thread(&self, pid: u32)
    {
        println!("[TRACER] Thread tracing process {} shutdown", pid);
    }

    fn run_thread(&self, tracee: Child, mut tracer: TracerEngine)
    {
        let pid = Pid::from_raw(tracee.id() as i32);
        /*
         * The main tracing
         */
        loop {
            match self.wait_for_syscall(pid) {
                false => break,
                true => (),
            }
            
            self.sync_registers(pid, &mut tracer);
            
            tracer.trace();
        }
    }

    fn sync_registers(&self, pid: Pid, tracer: &mut TracerEngine) 
    {
        let regs: nix::libc::user_regs_struct = ptrace::getregs(pid).unwrap();
        tracer.sync_registers(regs);
    }

    fn wait_for_syscall(&self, pid: Pid) -> bool
    {
        // Continue execution
        match ptrace::syscall(pid, None) {
            Ok(()) => { },
            /*
            Err(ref err) if err.kind() == nix::errno::Errno::ESRCH => {
                println!("ESRCH: No such process: {:?}", err);
                return false;

            }
            */
            Err(err) => {
                println!("Fail to restart tracee: {:?}", err);
                return false;
            }
        }

        match waitpid(pid, None) {
            Err(err) => {
                panic!("Oops something happens when waiting: {}", err);
            },

            Ok(status) => {
                match status {
                    WaitStatus::Stopped(pid, signo) => {
                        match signo {
                            Signal::SIGTRAP => {
                                return true;
                            },
                            Signal::SIGSEGV => {
                                let regs = ptrace::getregs(pid).unwrap();
                                println!("Tracee {} segfault at {:#x}", pid, regs.rip);
                                return false;
                            },
                            // TODO: add support for other signals
                            _ => {
                                println!("Tracee {} received signal {} which is not handled", pid, signo);
                                return false;
                            },
                        }
                    },
                    WaitStatus::Exited(pid, exit_status) => {
                        println!("The tracee {} exits with status {}", pid, exit_status);
                        return false;
                    },
                    // TODO: add support for other WaitStatus
                    _ => {
                        panic!("WaitStatus not handled");
                    },
                }
            },
        }
    }
}



fn main()
{
    /* TODO: add more argument to configure the tracer:
     *  - program to trace with its arguments
     *  - option on which port to listen
     *  - options for what to trace
     *  - architecture?
     *  - etc.
     * 
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: ./ptracer <program> <arguments>");
        return;
    }

    let program = &args[1];
    let prog_args = &args[2..];
     */

    let mut dbg = TraceDebugger::new();


    // Start tracing system calls
    println!("[TRACER] Start debugger...");
    dbg.run();

    println!("[TRACER] Stop debugger.");

}
