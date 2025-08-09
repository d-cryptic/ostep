# xv6 Process Structure Implementation

## What

This Rust implementation recreates the core process management data structures from MIT's xv6 operating system. It provides a type-safe abstraction of how operating systems track and manage processes, including their state, memory, and execution context.

## Why

Understanding process abstractions is fundamental to systems programming and operating systems design. This implementation serves as an educational tool to:

- Demonstrate how operating systems internally represent processes
- Show the lifecycle and state transitions of processes
- Illustrate memory management concepts in kernel space
- Provide hands-on experience with low-level system structures

## Purpose

- **Educational**: Learn process management concepts through practical implementation
- **Reference**: Understand xv6's process structure in a memory-safe language
- **Foundation**: Build upon these concepts for more complex OS features

## Code Structure

### Core Components

#### Context Structure (`main.rs:5-14`)
```rust
struct Context {
    eip: u32, // Instruction pointer
    esp: u32, // Stack pointer
    ebx: u32, // General purpose registers
    ecx: u32,
    edx: u32,
    esi: u32,
    edi: u32,
    ebp: u32, // Base pointer
}
```
Represents the CPU register state that must be saved/restored during process context switching.

#### Process States (`main.rs:32-40`)
```rust
enum ProcState {
    Unused,    // Process slot is free
    Embryo,    // Process is being created
    Sleeping,  // Process is blocked waiting for I/O
    Runnable,  // Process is ready to run
    Running,   // Process is currently executing
    Zombie,    // Process has exited but parent hasn't read exit status
}
```
Defines the possible states in a process's lifecycle.

#### Process Structure (`main.rs:58-72`)
The main `Proc` struct contains all information the kernel needs to track about each process:

- **Memory Management**: `mem` (process memory start), `sz` (memory size), `kstack` (kernel stack)
- **Process Control**: `state`, `pid`, `parent`, `killed`
- **I/O and Files**: `ofile` (open file table), `cwd` (current working directory)
- **Execution Context**: `context` (saved registers), `tf` (trap frame for system calls/interrupts)
- **Synchronization**: `chan` (sleep channel for blocking operations)

#### Process Table (`main.rs:124-167`)
Safe wrapper providing process management operations:

- `allocate_process()`: Finds unused process slot or creates new process
- `find_process(pid)`: Locates process by PID
- Process recycling to efficiently reuse process slots

### Key Methods

#### Process Control (`main.rs:94-122`)
- `new(pid)`: Creates process in `Embryo` state
- `is_running()` / `is_runnable()`: State checking utilities
- `kill()`: Marks process for termination
- `set_state()`: Updates process state

### Testing

The implementation includes comprehensive tests (`main.rs:170-196`) covering:
- Process creation and initialization
- State transitions and validation
- Process table operations

### Example Usage

```rust
let mut ptable = ProcessTable::new(5);

// Allocate a new process
if let Some(proc) = ptable.allocate_process() {
    println!("Created process with PID: {}", proc.pid);
    proc.set_state(ProcState::Runnable);
    
    // Process is now ready to be scheduled
    if proc.is_runnable() {
        // Scheduler would pick this up
    }
}
```

## Building and Running

```bash
cargo build    # Compile the project
cargo test     # Run tests
cargo run      # Execute the demo
```