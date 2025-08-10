# Process API Interlude: Fork System Call in Rust

This project demonstrates the fundamental process creation mechanism in Unix-like systems through the `fork()` system call, implemented in Rust as a learning exercise for operating systems concepts.

## Overview

The `fork()` system call is one of the most important system calls in Unix-like operating systems. It creates a new process by making an exact copy of the calling process. This project shows three different approaches to implement fork-like behavior in Rust.

## What is fork()?

### The Concept
- **fork()** creates a new process (child) that is an identical copy of the current process (parent)
- Both processes continue execution from the point where fork() was called
- The only difference: fork() returns different values to parent and child
  - **Parent process**: receives child's Process ID (PID) 
  - **Child process**: receives 0
  - **Error case**: returns -1

### Why fork() Matters
1. **Process Creation**: Foundation of how new processes are created in Unix
2. **Concurrency**: Enables parallel execution of programs
3. **System Programming**: Essential for shells, servers, and system utilities
4. **Operating Systems**: Core mechanism for multiprogramming

## Code Implementations

### 1. Direct fork() Implementation (`main.rs`)

```rust
use std::process;

fn main() {
    println!("hello (pid: {})", process::id());

    match unsafe { libc::fork() } {
        -1 => {
            // fork failed
            eprintln!("fork failed");
            process::exit(1);
        }
        0 => {
            // child (new process)
            println!("child (pid:{})", process::id());
        }
        child_pid => {
            // parent goes down this path (main)
            println!("parent of {} (pid:{})", child_pid, process::id());
        }
    }
}
```

**How it works:**
1. Prints initial message with current PID
2. Calls `libc::fork()` to create child process
3. Uses pattern matching on return value:
   - `-1`: Fork failed (error handling)
   - `0`: Code running in child process
   - `positive number`: Code running in parent (number is child's PID)

**Key Features:**
- **Direct system call**: Uses actual Unix fork() via libc
- **Unsafe code**: Required because fork() can cause undefined behavior in some contexts
- **True process duplication**: Creates identical memory image

### 2. Command Spawn with Args (`child.rs`)

```rust
use std::process::{Command, self};

fn child_main() {
    println!("Child (pid:{})", process::id());
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 && args[1] == "--child" {
        child_main();
        return;
    }

    println!("hello (pid:{})", process::id());

    match Command::new(&args[0]).arg("--child").spawn() {
        Ok(mut child) => {
            println!("parent of {} (pid:{})", child.id(), process::id());
            let _ = child.wait();
        }
        Err(e) => {
            eprintln!("spawn failed: {}", e);
            process::exit(1);
        }
    }
}
```

**How it works:**
1. Checks command line arguments to determine if running as child
2. If `--child` argument present, runs child-specific code
3. Otherwise, spawns new instance of same program with `--child` flag
4. Parent waits for child to complete

**Key Features:**
- **Pure Rust**: No unsafe code or external dependencies
- **Cross-platform**: Works on Windows, macOS, Linux
- **Argument-based distinction**: Uses command line flags to differentiate parent/child

### 3. Current Executable Spawn (`spawn.rs`)

```rust
use std::process::{self, Command};

fn main() {
    println!("hello (pid:{})", process::id());

    match Command::new(std::env::current_exe().unwrap())
        .arg("--child")
        .spawn()
    {
        Ok(mut child) => {
            println!("parent of {} (pid:{})", child.id(), process::id());
            let _ = child.wait();
        }
        Err(e) => {
            eprintln!("Spawn failed: {}", e);
            process::exit(1);
        }
    }
}
```

**How it works:**
1. Gets current executable path using `std::env::current_exe()`
2. Spawns new instance of itself with `--child` argument
3. Parent waits for child process to complete

**Key Features:**
- **Self-spawning**: Automatically determines executable path
- **Simplified**: No argument parsing logic
- **Robust**: Less prone to path-related issues

## Comparison: C vs Rust Approaches

| Aspect | C fork() | Rust libc::fork() | Rust Command::spawn() |
|--------|----------|-------------------|----------------------|
| **Safety** | Unsafe | Unsafe (explicit) | Safe |
| **Portability** | Unix only | Unix only | Cross-platform |
| **Memory sharing** | Shared until write | Shared until write | Separate processes |
| **Error handling** | Manual checking | Pattern matching | Result type |
| **Complexity** | Low | Low | Medium |

## Expected Output

### Direct fork() (main.rs):
```
hello (pid: 12345)
parent of 12346 (pid: 12345)
child (pid: 12346)
```

### Command spawn approaches (child.rs, spawn.rs):
```
hello (pid: 12345)
parent of 12346 (pid: 12345)
Child (pid: 12346)
```

*Note: Actual PIDs will vary, and order of parent/child output may differ due to process scheduling.*

## Building and Running

### Prerequisites
- Rust toolchain (cargo, rustc)
- Unix-like system for `main.rs` (fork() system call)
- Any platform for `child.rs` and `spawn.rs`

### Commands

```bash
# Run direct fork implementation (Unix only)
cargo run --bin main

# Run command spawn with args (cross-platform)
cargo run --bin child

# Run current executable spawn (cross-platform)
cargo run --bin spawn

# Build all binaries
cargo build --release
```

### Individual Compilation
```bash
# Direct fork (requires libc)
rustc --extern libc src/main.rs -o fork_main
./fork_main

# Command spawn
rustc src/child.rs -o fork_child
./fork_child

rustc src/spawn.rs -o fork_spawn
./fork_spawn
```


## Real-World Applications

### Where fork() is Used
1. **Shell Programs**: Creating processes for commands
2. **Web Servers**: Handling multiple client connections
3. **System Daemons**: Background service processes
4. **Process Pools**: Pre-forking worker processes
5. **Testing**: Isolating test execution environments

### Modern Alternatives
- **threads**: Lighter weight concurrency within same process
- **async/await**: Cooperative multitasking for I/O-bound work
- **spawn()**: More explicit process creation
- **containers**: Process isolation at system level

## Security and Safety Considerations

### fork() Dangers
1. **Fork bombs**: Uncontrolled process creation can exhaust system resources
2. **Race conditions**: Parent and child accessing shared resources
3. **Signal handling**: Complex interactions between processes
4. **File descriptor inheritance**: Unintended resource sharing

### Rust Safety Features
1. **Memory safety**: Prevents buffer overflows and use-after-free
2. **Type safety**: Compile-time error detection
3. **Explicit unsafe**: Clear boundaries for dangerous operations
4. **Resource management**: Automatic cleanup through RAII