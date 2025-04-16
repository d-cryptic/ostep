package proc

type ProcState int

const (
	UNUSED   ProcState = iota // slot in process table is free
	EMBRYO                    // process is being created
	SLEEPING                  // process is blocked, waiting on some event (chan)
	RUNNABLE                  // ready to run but not running
	RUNNING                   // currently executing
	ZOMBIE                    // Terminated, waiting for parent to wait()
)

type Context struct {
	EIP uint32 // Instruction Pointer - where the process was executing
	ESP uint32 // STack pointer - points to top of the stack

	// rest holds temporary data
	EBX uint32
	ECX uint32
	ESI uint32
	EDI uint32
	EBP uint32
}

type TrapFrame struct {
	// Simulate saved user space registers on syscall/trap
	// Add fields like EAX, CS, DS, etc. if needed
}

type File struct {
	// file structure
	Name string
}

type Inode struct {
	// inode representation
	Path string
}

type Proc struct {
	Mem     []byte      // process memory
	Size    uint32      // size of memory
	Kstack  []byte      // kernel stack
	State   ProcState   // Process State
	PID     int         // Process ID
	Parent  *Proc       // Parent Process
	Chan    interface{} // Sleeping channel - blocking syscall
	Killed  bool        // if true, process should be terminated
	OFile   [16]*File   // Open Files
	CWD     *Inode      // current working directory
	Context *Context    // CPU context for scheduling - where process resumes from on context switch
	TF      *TrapFrame  // Trap Frame - snapshot of user registers during syscalls or trap
}

// fork(): Clone the current process
// 1. Copy user memory
// 2. duplicate trap frame
// 3. copy file descriptors
// 4. assign new pid
// 5. add to process table
// 6. set state to RUNNABLE
func Fork(parent *Proc) *Proc {
	child := &Proc{
		Mem:     append([]byte(nil), parent.Mem...), // copy memory
		Size:    parent.Size,
		Kstack:  make([]byte, len(parent.Kstack)),
		State:   EMBRYO,
		Parent:  parent,
		PID:     nextPID(), // placeholder function
		CWD:     parent.CWD,
		OFile:   parent.OFile,
		Context: &Context{},   // copied as needed
		TF:      &TrapFrame{}, // copy if needed
	}

	child.State = RUNNABLE
	return child
}

// exec(): replace memory with a new program
// 1. free current memory
// 2. load new binary into memory
// 3. reset trapframe/context
// 4. set new instruction pointer to entry point
func Exec(p *Proc, binary []byte) {
	p.Mem = binary //load new program
	p.Size = uint32(len(binary))
	p.TF = &TrapFrame{}          // reset trapframe
	p.Context = &Context{EIP: 0} // set entry point
}

// context switching (in terms of xv6)
// saving old process's registers into its context
// loading new process's context into cpu registers
// below is just a placeholder function
func Switch(old *Proc, new *Proc) {
  // save old context
  saveContext(old.Context)

  // Restore new context
  restoreContext(new.Context)

  // simulate trapFrame jump
  runProcess(new)
}
