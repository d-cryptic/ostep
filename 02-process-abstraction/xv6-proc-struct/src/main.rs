// The registers xv6 will save and restore
// to stop and subsequently restart a process
#[derive(Debug, Clone, Copy)]
#[repr(C)]
struct Context {
    eip: u32, //instruction pointer
    esp: u32, //stack pointer
    ebx: u32, // general purpose registers
    ecx: u32,
    edx: u32,
    esi: u32,
    edi: u32,
    ebp: u32, //base pointer
}

impl Default for Context {
    fn default() -> Self {
        Context {
            eip: 0,
            esp: 0,
            ebx: 0,
            ecx: 0,
            edx: 0,
            esi: 0,
            edi: 0,
            ebp: 0,
        }
    }
}

// The different states a process can be in
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ProcState {
    Unused,
    Embryo,
    Sleeping,
    Runnable,
    Running,
    Zombie,
}

impl Default for ProcState {
    fn default() -> Self {
        ProcState::Unused
    }
}

// Forward declarations for types that would be defined elsewhere
type FileHandle = usize; // placeholder for file handle
type InodeHandle = usize; // placeholder for inode handle
type TrapFrame = [u8; 256]; // placeholder for trap frame

const NOFILE: usize = 16; //maximum open files per process

// the information xv6 tracks about each process
// including its register context and state

#[derive(Debug)]
struct Proc {
    mem: Option<*mut u8>,    //start of process memory (raw pointer for kernel space)
    sz: u32,                 // size of process memory
    kstack: Option<*mut u8>, //Bottom of kernel stack for this process
    state: ProcState,        //process state
    pid: i32,                //process ID
    parent: Option<Box<Proc>>, // Parent Process (using Box for ownership)
    chan: Option<*const ()>, // If Some, sleeping on cha
    killed: bool,            // if true, has been killed
    ofile: [Option<FileHandle>; NOFILE], //open files
    cwd: Option<InodeHandle>, //current directory
    context: Context,        // switch here to run process
    tf: Option<Box<TrapFrame>>, //trap frame for current interrupt
}

impl Default for Proc {
    fn default() -> Self {
        Proc {
            mem: None,
            sz: 0,
            kstack: None,
            state: ProcState::Unused,
            pid: 0,
            parent: None,
            chan: None,
            killed: false,
            ofile: [None; NOFILE],
            cwd: None,
            context: Context::default(),
            tf: None,
        }
    }
}

impl Proc {
    // constructor for new process
    fn new(pid: i32) -> Self {
        Proc {
            pid,
            state: ProcState::Embryo,
            ..Default::default()
        }
    }

    // check if process is running
    fn is_running(&self) -> bool {
        self.state == ProcState::Running
    }

    // check if process can be scheduled
    fn is_runnable(&self) -> bool {
        self.state == ProcState::Runnable
    }

    // Mark process as killed
    fn kill(&mut self) {
        self.killed = true;
    }

    // Set process state
    fn set_state(&mut self, new_state: ProcState) {
        self.state = new_state;
    }
}

// safe wrapper for process table operations
struct ProcessTable {
    processes: Vec<Proc>,
    next_pid: i32,
}

impl ProcessTable {
    fn new(max_processes: usize) -> Self {
        ProcessTable {
            processes: Vec::with_capacity(max_processes),
            next_pid: 1,
        }
    }

    fn allocate_process(&mut self) -> Option<&mut Proc> {
        // First, look for an unused process slot by finding its index
        let unused_index = self.processes
            .iter()
            .position(|p| p.state == ProcState::Unused);
        
        if let Some(index) = unused_index {
            // Reuse existing unused slot
            let proc = &mut self.processes[index];
            *proc = Proc::new(self.next_pid);
            self.next_pid += 1;
            return Some(proc);
        }
        
        // No unused slot found, try to add new process if capacity allows
        if self.processes.len() < self.processes.capacity() {
            let proc = Proc::new(self.next_pid);
            self.next_pid += 1;
            self.processes.push(proc);
            // Return the last element (the one we just pushed)
            self.processes.last_mut()
        } else {
            None // Process table is full and no unused slots
        }
    }

    fn find_process(&mut self, pid: i32) -> Option<&mut Proc> {
        self.processes.iter_mut().find(|p| p.pid == pid)
    }
}

// Example usage and testing
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_creation() {
        let mut ptable = ProcessTable::new(10);

        let proc = ptable.allocate_process().unwrap();
        assert_eq!(proc.pid, 1);
        assert_eq!(proc.state, ProcState::Embryo);
        assert!(!proc.killed);
    }

    #[test]
    fn test_process_states() {
        let mut proc = Proc::new(1);

        assert!(!proc.is_running());
        assert!(!proc.is_runnable());

        proc.set_state(ProcState::Runnable);
        assert!(proc.is_runnable());

        proc.set_state(ProcState::Running);
        assert!(proc.is_running());
    }
}

fn main() {
    let mut ptable = ProcessTable::new(5);

    // create processes
    if let Some(proc1) = ptable.allocate_process() {
        println!("Created process with PID: {}", proc1.pid);
        proc1.set_state(ProcState::Runnable);
        println!("Process state: {:?}", proc1.state);
    }

    if let Some(proc2) = ptable.allocate_process() {
        println!("Created process with PID: {}", proc2.pid);
        proc2.kill();
        println!("Process killed: {}", proc2.killed);
    }
}
