# Process API Interlude: Exec System Call in Rust

This project demonstrates the `exec()` family of system calls in Unix-like systems, implemented in Rust as an educational tool for understanding process execution and program replacement.

## Overview

The `exec()` system call is fundamental to how programs are executed in Unix-like operating systems. Unlike `fork()` which creates a new process, `exec()` **replaces** the current process image with a new program while keeping the same Process ID (PID).

## What is exec()?

### The Concept
- **exec()** replaces the current process with a new program
- The original process memory image is **completely replaced**
- Same PID, but entirely different program code and data
- No return to the original program (unless exec fails)
- Foundation of how shells execute commands

### Why exec() Matters
1. **Program Execution**: How operating systems run programs
2. **Shell Implementation**: Core mechanism shells use to run commands
3. **Process Transformation**: Efficient way to change what a process does
4. **System Programming**: Essential for building system tools and utilities

## The fork() + exec() Pattern

Most process creation in Unix follows this pattern:
1. **fork()**: Create a copy of current process
2. **exec()**: Replace child process with desired program
3. **wait()**: Parent waits for child to complete

This separation allows for:
- Setting up process environment before execution
- I/O redirection and file descriptor manipulation
- Signal handling configuration
- Process group and session management

## Code Implementations

### 1. Direct fork() + execvp() (`main.rs`)

```rust
use std::ffi::CString;
use std::process;
use std::ptr;

fn main() {
    println!("hello (pid:{})", process::id());

    let rc = unsafe { libc::fork() };
    match rc {
        -1 => {
            // fork failed; exit
            eprintln!("fork failed");
            process::exit(1);
        }
        0 => {
            // child (new process)
            println!("child (pid:{})", process::id());

            // prepare arguments for execvp
            let program = CString::new("wc").unwrap();
            let arg1 = CString::new("src/main.rs").unwrap();
            let args = [program.as_ptr(), arg1.as_ptr(), ptr::null()];

            // execute word count program
            unsafe {
                libc::execvp(program.as_ptr(), args.as_ptr());
            }

            // This shouldn't print out (only if exec fails)
            println!("this shouldn't print out");
            process::exit(1); // Exit if exec fails
        }
        child_pid => {
            // parent goes down this path
            let mut status = 0;
            let rc_wait = unsafe { libc::wait(&mut status) };
            println!(
                "parent of {} (rc_wait:{}) (pid:{})",
                child_pid, rc_wait, process::id()
            );
        }
    }
}
```

**How it works:**
1. **Fork**: Creates identical child process
2. **Child process**: Calls `execvp()` to replace itself with `wc` program
3. **exec()**: Completely replaces process memory with `wc` program
4. **Parent**: Waits for child (now running `wc`) to complete
5. **Critical insight**: The `println!("this shouldn't print out")` never executes because `exec()` replaces the entire process

**Key Features:**
- **True Unix behavior**: Uses actual `fork()` and `execvp()` system calls
- **Process replacement**: Child process is completely transformed
- **Same PID**: Child keeps same process ID but runs different program
- **No return**: `exec()` never returns on success

### 2. Cross-platform Command Spawn (`lvl2.rs`)

```rust
use std::env;
use std::process::{self, Command};

fn main() {
    let args: Vec<String> = env::args().collect();

    // Check if we're the child process
    if args.len() > 1 && args[1] == "--child" {
        println!("child (pid:{})", process::id());

        // Execute word count program - this replaces the current process
        let status = Command::new("wc")
            .arg("src/main.rs")
            .status()
            .unwrap_or_else(|e| {
                eprintln!("failed to execute wc: {}", e);
                process::exit(1);
            });

        // Exit with the same code as the executed program
        process::exit(status.code().unwrap_or(1));
    }

    // parent process
    println!("hello (pid:{})", process::id());

    match Command::new(&args[0]).arg("--child").spawn() {
        Ok(mut child) => {
            let child_id = child.id();
            // wait for child to complete
            match child.wait() {
                Ok(_status) => {
                    println!(
                        "parent of {} (rc_wait:{}) (pid:{})",
                        child_id, child_id, process::id()
                    );
                }
                Err(e) => {
                    eprintln!("wait failed: {}", e);
                    process::exit(1);
                }
            }
        }
        Err(e) => {
            eprintln!("spawn failed: {}", e);
            process::exit(1);
        }
    }
}
```

**How it works:**
1. **Self-spawning**: Program spawns new instance of itself with `--child` flag
2. **Child detection**: Uses command line arguments to determine behavior
3. **Program execution**: Child runs `wc` command and exits
4. **Safe alternative**: Avoids unsafe system calls while demonstrating concept

**Key Features:**
- **Cross-platform**: Works on Windows, macOS, Linux
- **Safe Rust**: No unsafe blocks or FFI calls
- **Educational**: Demonstrates exec-like behavior without system calls
- **Argument-based**: Uses flags to control process behavior

### 3. Simplified Process Spawn (`lvl3.rs`)

```rust
use std::process::{self, Command};

fn main() {
    println!("hello (pid:{})", process::id());

    // fork equivalent: spawn a new process that runs wc
    match Command::new("wc").arg("src/main.rs").spawn() {
        Ok(mut child) => {
            let child_id = child.id();
            println!("child process started (pid:{})", child_id);

            // Parent waits for child (equivalent to wait())
            match child.wait() {
                Ok(status) => {
                    println!(
                        "parent of {} (exit_status:{}) (pid:{})",
                        child_id,
                        status.code().unwrap_or(-1),
                        process::id()
                    );
                }
                Err(e) => {
                    eprintln!("wait failed: {}", e);
                    process::exit(1);
                }
            }
        }
        Err(e) => {
            eprintln!("spawn failed: {}", e);
            process::exit(1);
        }
    }
}
```

**How it works:**
1. **Direct spawning**: Creates new process running `wc` directly
2. **No fork**: Bypasses fork step, goes straight to program execution
3. **Simple model**: Demonstrates parent-child relationship without complexity

**Key Features:**
- **Minimal complexity**: Straightforward process creation
- **Modern approach**: Uses high-level Rust APIs
- **Clear demonstration**: Shows parent-child waiting pattern
- **Practical**: How most Rust programs handle subprocess creation

## Comparison: Different Approaches

| Aspect | Direct fork+exec | Command Self-spawn | Direct Command |
|--------|------------------|-------------------|----------------|
| **Authenticity** | True Unix behavior | Simulates behavior | Modern equivalent |
| **Safety** | Unsafe (explicit) | Safe | Safe |
| **Portability** | Unix only | Cross-platform | Cross-platform |
| **Complexity** | High | Medium | Low |
| **Educational Value** | Maximum | High | Medium |
| **Process Model** | Traditional | Hybrid | Simplified |

## Expected Output

### Direct fork+exec (main.rs):
```
hello (pid:12345)
child (pid:12346)
  42  234 1567 src/main.rs
parent of 12346 (rc_wait:12346) (pid:12345)
```

### Command spawn approaches (lvl2.rs, lvl3.rs):
```
hello (pid:12345)
child (pid:12346)
  42  234 1567 src/main.rs
parent of 12346 (rc_wait:12346) (pid:12345)
```

*Note: The `wc` output shows line count, word count, and byte count of the source file.*

## Building and Running

### Prerequisites
- Rust toolchain (cargo, rustc)
- Unix-like system for `main.rs` (direct exec calls)
- `wc` command available in PATH

### Commands

```bash
# Run direct fork+exec implementation (Unix only)
cargo run --bin main

# Run cross-platform command spawn
cargo run --bin lvl2

# Run simplified process spawn
cargo run --bin lvl3

# Build all binaries
cargo build --release
```

### Individual Compilation
```bash
# Direct fork+exec (requires libc)
rustc --extern libc src/main.rs -o exec_main
./exec_main

# Command spawn variants
rustc src/lvl2.rs -o exec_lvl2
./exec_lvl2

rustc src/lvl3.rs -o exec_lvl3
./exec_lvl3
```

### Programming Concepts

1. **Foreign Function Interface (FFI)**
   - Calling C libraries from Rust
   - Memory safety considerations with unsafe code
   - Cross-language interoperability

2. **Process Management**
   - Parent-child process relationships
   - Process synchronization and waiting
   - Exit status handling

3. **Cross-platform Abstraction**
   - Portable vs platform-specific solutions
   - High-level APIs vs system calls
   - Trade-offs in abstraction layers

## The exec() Family

### Common exec() Variants
- **execl()**: Takes argument list
- **execv()**: Takes argument vector
- **execle()**: With environment
- **execve()**: Vector with environment
- **execlp()**: Searches PATH
- **execvp()**: Vector, searches PATH

### Rust Equivalents
- **Direct**: `libc::execvp()` through FFI
- **High-level**: `Command::new().exec()` (Unix-only, experimental)
- **Practical**: `Command::new().status()` for most use cases

## Real-World Applications

### Where exec() is Used
1. **Shells**: bash, zsh, fish execute user commands
2. **Init Systems**: systemd, OpenRC launch system services
3. **Process Spawners**: supervisors, job schedulers
4. **Container Runtimes**: Docker, podman execute container processes
5. **Testing Frameworks**: Isolated test execution

### Modern Alternatives
- **spawn()**: Creates new processes without fork
- **posix_spawn()**: More efficient process creation
- **clone()**: Linux-specific process/thread creation
- **CreateProcess()**: Windows process creation API

## Security and Safety Considerations

### exec() Security Issues
1. **Path traversal**: Malicious program paths
2. **Environment injection**: Unsafe environment variables
3. **Privilege escalation**: Setuid/setgid programs
4. **Resource leaks**: File descriptors, signals

### Rust Safety Features
1. **Memory safety**: Prevents buffer overflows in arguments
2. **Type safety**: Compile-time argument validation
3. **Explicit unsafe**: Clear boundaries for dangerous operations
4. **Resource management**: Automatic cleanup through RAII

## Advanced Topics

### Process Control
```rust
// Setting process groups
unsafe {
    libc::setpgid(0, 0);
}

// Signal handling
unsafe {
    libc::signal(libc::SIGINT, libc::SIG_IGN);
}

// File descriptor manipulation
unsafe {
    libc::dup2(fd, libc::STDIN_FILENO);
}
```

### Environment Management
```rust
// Setting environment variables
std::env::set_var("PATH", "/usr/local/bin:/usr/bin");

// Clearing environment
Command::new("program").env_clear();

// Custom environment
Command::new("program")
    .env("CUSTOM_VAR", "value")
    .spawn();
```

## Debugging and Troubleshooting

### Common Issues
1. **"No such file or directory"**: Program not in PATH
2. **Permission denied**: Executable permissions not set
3. **exec format error**: Wrong architecture or corrupted binary
4. **Argument list too long**: Command line too large

### Debugging Techniques
```bash
# Trace system calls
strace ./exec_main

# Check program existence
which wc

# Verify permissions
ls -l /usr/bin/wc

# Test command manually
wc src/main.rs
```

## Conclusion

The `exec()` system call is fundamental to understanding how Unix-like operating systems execute programs. This project demonstrates the concept through multiple Rust implementations, from low-level system calls to high-level abstractions.

Key takeaways:
1. **exec() replaces processes**, it doesn't create them
2. **fork() + exec()** is the standard Unix process creation pattern
3. **Modern languages** provide safer, cross-platform alternatives
4. **Understanding system calls** is essential for system programming

The progression from unsafe system calls to safe abstractions illustrates both the power of low-level control and the benefits of modern programming language design. This foundation is essential for understanding shells, servers, containers, and any software that manages other programs.