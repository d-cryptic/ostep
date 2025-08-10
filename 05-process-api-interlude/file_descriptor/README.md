# Process API Interlude: File Descriptors and I/O Redirection in Rust

This project demonstrates file descriptor manipulation and I/O redirection in Unix-like systems through the `fork()`, `exec()`, and file system calls, implemented in Rust as an educational tool for understanding process I/O control and system programming.

## Overview

File descriptors are fundamental abstractions in Unix-like systems that represent open files, pipes, sockets, and other I/O sources. This project shows how processes can manipulate file descriptors to redirect input/output, enabling powerful I/O control patterns essential for shells, pipelines, and system utilities.

## What are File Descriptors?

### The Concept
- **File descriptors** are small non-negative integers that identify open files
- **Standard descriptors**:
  - `0` (STDIN_FILENO): Standard input
  - `1` (STDOUT_FILENO): Standard output  
  - `2` (STDERR_FILENO): Standard error
- **Process-specific**: Each process has its own file descriptor table
- **Inheritance**: Child processes inherit parent's file descriptors
- **Abstraction**: Uniform interface for files, pipes, sockets, devices

### I/O Redirection Mechanism
1. **Close** the target file descriptor (e.g., stdout)
2. **Open** a new file - gets the lowest available descriptor number
3. **Result**: New file takes over the closed descriptor's number
4. **Effect**: Program output goes to the new file instead of terminal

### Why This Matters
1. **Shell Functionality**: Foundation of shell I/O redirection (`>`, `<`, `|`)
2. **Process Communication**: Basis for pipes and inter-process communication
3. **System Programming**: Essential for servers, utilities, and system tools
4. **Resource Management**: Understanding file descriptor limits and cleanup
5. **Security**: Controlling process access to files and resources

## File Descriptor Inheritance

### Process Creation and File Descriptors
```
Parent Process:
├── FD 0: stdin (terminal)
├── FD 1: stdout (terminal)  
├── FD 2: stderr (terminal)
└── FD 3: file.txt

After fork():
Child Process (inherits all):
├── FD 0: stdin (terminal)
├── FD 1: stdout (terminal)
├── FD 2: stderr (terminal) 
└── FD 3: file.txt
```

### I/O Redirection Process
```
Child Process:
1. close(STDOUT_FILENO)     // Close stdout (FD 1)
2. open("output.txt", ...)  // Open file, gets FD 1
3. exec("wc", ...)          // wc's output goes to output.txt
```

## Code Implementations

### 1. Direct File Descriptor Manipulation (`main.rs`)

```rust
use std::ffi::c_uint;
use std::ffi::CString;
use std::process;
use std::ptr;

fn main() {
    let rc = unsafe { libc::fork() };

    match rc {
        -1 => {
            // fork failed; exit
            eprintln!("fork failed");
            process::exit(1);
        }
        0 => {
            // child: redirect standard output to a file
            unsafe {
                libc::close(libc::STDOUT_FILENO);
                let fd = libc::open(
                    CString::new("src/tmp.txt").unwrap().as_ptr(),
                    libc::O_CREAT | libc::O_WRONLY | libc::O_TRUNC,
                    libc::S_IRWXU as c_uint,
                );
                assert!(fd >= 0);
            }

            // now exec "wc"
            let program = CString::new("wc").unwrap();
            let arg1 = CString::new("src/main.rs").unwrap();
            let args = [program.as_ptr(), arg1.as_ptr(), ptr::null()];

            unsafe {
                libc::execvp(program.as_ptr(), args.as_ptr());
            }
        }
        _child_pid => {
            // parent goes down this path (original process)
            let wc = unsafe { libc::wait(ptr::null_mut()) };
            assert!(wc >= 0);
        }
    }
}
```

**How it works:**
1. **Fork**: Create child process that inherits all file descriptors
2. **Close stdout**: Child closes standard output (file descriptor 1)
3. **Open file**: Opens `tmp.txt` - gets file descriptor 1 (lowest available)
4. **Execute**: `wc` command inherits redirected stdout pointing to file
5. **Result**: `wc` output goes to `tmp.txt` instead of terminal
6. **Parent**: Waits for child to complete

**Key Features:**
- **True Unix behavior**: Uses actual file descriptor manipulation
- **Low-level control**: Direct system call interface
- **Educational value**: Shows exact mechanism shells use for redirection
- **Authentic redirection**: Demonstrates real I/O redirection implementation

### 2. High-Level File Redirection (`lvl2.rs`)

```rust
use std::fs::File;
use std::process::{Command, Stdio};

fn main() {
    // create output file and redirect stdout to it
    let output_file = File::create("src/tmp.txt").unwrap_or_else(|e| {
        eprintln!("failed to create output file: {}", e);
        std::process::exit(1);
    });

    // execute wc with stdout redirected to file
    let status = Command::new("wc")
        .arg("src/main.rs")
        .stdout(Stdio::from(output_file))
        .status()
        .unwrap_or_else(|e| {
            eprintln!("failed to execute wc: {}", e);
            std::process::exit(1);
        });

    assert!(status.success());
    println!("Word count output written to src/tmp.txt");
}
```

**How it works:**
1. **File creation**: Creates output file using safe Rust APIs
2. **Stdio redirection**: Uses `Stdio::from()` to redirect command output
3. **Command execution**: Runs `wc` with stdout redirected to file
4. **Status checking**: Verifies command completed successfully

**Key Features:**
- **Safe Rust**: No unsafe blocks or manual file descriptor manipulation
- **High-level abstractions**: Uses `Command` and `Stdio` APIs
- **Error handling**: Comprehensive error checking with proper messages
- **Cross-platform**: Works on multiple operating systems

### 3. Fork-Based Safe Redirection (`lvl3.rs`)

```rust
use std::env;
use std::fs::File;
use std::process::{self, Command, Stdio};

fn main() {
    let args: Vec<String> = env::args().collect();

    // Check if we're the child process
    if args.len() > 1 && args[1] == "--child" {
        // Child process: redirect output and exec wc
        let output_file = File::create("src/tmp.txt").unwrap_or_else(|e| {
            eprintln!("failed to create output file: {}", e);
            process::exit(1);
        });

        let status = Command::new("wc")
            .arg("p4.c")
            .stdout(Stdio::from(output_file))
            .status()
            .unwrap_or_else(|e| {
                eprintln!("failed to execute wc: {}", e);
                process::exit(1);
            });

        process::exit(status.code().unwrap_or(1));
    }

    // parent process: spawn child
    match Command::new(&args[0]).arg("--child").spawn() {
        Ok(mut child) => {
            // wait for child to complete
            match child.wait() {
                Ok(status) => {
                    assert!(status.success());
                    println!("Child process completed successfully");
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
1. **Self-spawning**: Program spawns itself with `--child` flag
2. **Child behavior**: Child process handles file redirection and command execution
3. **Safe APIs**: Uses Rust's safe file and process APIs throughout
4. **Parent waiting**: Parent waits for child completion

**Key Features:**
- **Fork simulation**: Demonstrates fork-like behavior safely
- **Argument-based control**: Uses command line flags for process differentiation
- **Safe throughout**: No unsafe code blocks
- **Educational**: Shows fork+exec+redirect pattern with modern APIs

## Comparison: Implementation Approaches

| Aspect | Direct FD manipulation | High-level Command | Fork-based safe |
|--------|----------------------|-------------------|-----------------|
| **File descriptors** | Manual close/open | Automatic | Automatic |
| **Safety** | Unsafe (explicit) | Safe | Safe |
| **Authenticity** | True Unix behavior | Abstracted | Simulated |
| **Complexity** | High | Low | Medium |
| **Error handling** | Manual checking | Built-in | Comprehensive |
| **Portability** | Unix only | Cross-platform | Cross-platform |
| **Educational value** | Maximum | Medium | High |

## Expected Output

### All implementations create `src/tmp.txt` with content like:
```bash
$ cat src/tmp.txt
  46  123  1234 src/main.rs
```

### Console output varies:
- **Direct FD**: Silent execution, output only in file
- **High-level**: "Word count output written to src/tmp.txt"  
- **Fork-based**: "Child process completed successfully"

## Building and Running

### Prerequisites
- Rust toolchain (cargo, rustc)
- Unix-like system for `main.rs` (direct file descriptor manipulation)
- `wc` command available in PATH
- Write permissions in `src/` directory

### Commands

```bash
# Run direct file descriptor manipulation (Unix only)
cargo run --bin main

# Run high-level command redirection
cargo run --bin lvl2

# Run fork-based safe redirection  
cargo run --bin lvl3

# Build all binaries
cargo build --release

# Check output files
cat src/tmp.txt
```

### Individual Compilation
```bash
# Direct FD manipulation (requires libc)
rustc --extern libc src/main.rs -o fd_redirect_direct
./fd_redirect_direct

# High-level approach
rustc src/lvl2.rs -o fd_redirect_highlevel
./fd_redirect_highlevel

# Fork-based safe approach
rustc src/lvl3.rs -o fd_redirect_safe
./fd_redirect_safe
```

## Educational Value

### Operating System Concepts Demonstrated

1. **File Descriptor Management**
   - How processes access files and I/O resources
   - File descriptor inheritance across fork()
   - Lowest-available descriptor assignment

2. **I/O Redirection Mechanics**
   - How shells implement `>`, `<`, `>>` operators
   - Process output control without program modification
   - Transparent I/O redirection for programs

3. **Process Environment Setup**
   - Configuring child process I/O before execution
   - Fork+exec pattern with I/O setup
   - Parent-child coordination for I/O control

4. **System Call Interface**
   - Low-level file operations (`open`, `close`)
   - File permission and mode handling
   - Error handling in system programming

### Programming Concepts

1. **Resource Management**
   - File descriptor lifecycle management
   - Resource cleanup and inheritance
   - Preventing resource leaks

2. **Abstraction Layers**
   - Low-level system calls vs high-level APIs
   - Safety vs control trade-offs
   - Cross-platform portability considerations

3. **Error Handling Patterns**
   - System call error checking
   - Rust Result types and error propagation
   - Graceful failure and recovery

## File Descriptor Operations

### Common System Calls
- **open()**: Create/open file, returns file descriptor
- **close()**: Close file descriptor, free resources
- **dup()/dup2()**: Duplicate file descriptors
- **read()/write()**: I/O operations using file descriptors
- **lseek()**: Change file position

### Rust Equivalents
- **Direct**: `libc::open()`, `libc::close()` through FFI
- **Safe**: `File::create()`, `File::open()` with automatic cleanup
- **Command**: `Stdio::from()` for process I/O redirection

## Real-World Applications

### Where File Descriptor Manipulation is Used
1. **Shell Programs**: Implementing `>`, `<`, `|` operators
2. **Web Servers**: Managing client connections and logging
3. **System Utilities**: Log rotation, file processing tools
4. **Container Runtimes**: Setting up container I/O
5. **Development Tools**: Build systems, test frameworks
6. **Network Services**: Socket management and proxying

### Modern Alternatives and Extensions
- **Pipes**: `pipe()` for inter-process communication
- **Named pipes (FIFOs)**: Persistent inter-process channels
- **Sockets**: Network communication endpoints
- **Event polling**: `epoll`, `kqueue` for scalable I/O
- **Async I/O**: Non-blocking and asynchronous operations

## Advanced File Descriptor Concepts

### dup2() System Call
```rust
// Duplicate file descriptor to specific number
unsafe {
    libc::dup2(source_fd, target_fd);
}
```

### Multiple Redirections
```rust
// Redirect both stdout and stderr
unsafe {
    let fd = libc::open(filename.as_ptr(), flags, mode);
    libc::dup2(fd, libc::STDOUT_FILENO);
    libc::dup2(fd, libc::STDERR_FILENO);
    libc::close(fd);
}
```

### Pipe Creation
```rust
// Create pipe for inter-process communication
let mut pipe_fds = [0i32; 2];
unsafe {
    libc::pipe(pipe_fds.as_mut_ptr());
}
let (read_fd, write_fd) = (pipe_fds[0], pipe_fds[1]);
```

## Security and Safety Considerations

### File Descriptor Security Issues
1. **Resource exhaustion**: Too many open files can crash processes
2. **Information leakage**: Unintended file descriptor inheritance
3. **Race conditions**: File descriptor reuse timing issues
4. **Permission bypasses**: Inherited privileged file descriptors

### Safe Programming Practices
1. **Close-on-exec**: Set FD_CLOEXEC flag to prevent inheritance
2. **Resource limits**: Monitor and limit open file descriptors
3. **Error handling**: Always check system call return values
4. **Cleanup**: Ensure file descriptors are properly closed

### Rust Safety Features
1. **RAII**: Automatic resource cleanup through Drop trait
2. **Type safety**: Compile-time prevention of invalid operations
3. **Memory safety**: Prevents buffer overflows in file operations
4. **Explicit unsafe**: Clear boundaries for dangerous operations

## Debugging and Troubleshooting

### Common Issues
1. **"Too many open files"**: File descriptor limit exceeded
2. **"Permission denied"**: Insufficient file permissions
3. **"No such file or directory"**: Path or program not found
4. **Zombie processes**: Child processes not properly cleaned up

### Debugging Techniques
```bash
# List open file descriptors for a process
lsof -p PID

# Monitor file descriptor usage
ls -la /proc/PID/fd/

# Check system limits
ulimit -n  # Max open files
cat /proc/sys/fs/file-max  # System-wide limit

# Trace system calls
strace -e trace=file ./program
```

### File Descriptor Monitoring
```bash
# Real-time file descriptor monitoring
watch -n 1 'ls /proc/$$/fd | wc -l'

# Detailed file descriptor information
ls -l /proc/self/fd/

# System-wide file descriptor usage
cat /proc/sys/fs/file-nr
```

## Performance Considerations

### File Descriptor Efficiency
- **Reuse**: Close and reuse file descriptors when possible
- **Limits**: Be aware of per-process and system limits
- **Buffering**: Consider file I/O buffering strategies
- **Non-blocking I/O**: Use for scalable applications

### I/O Redirection Overhead
- **System call cost**: Each open/close has overhead
- **Memory usage**: Buffered I/O uses memory
- **Context switching**: Process creation and management costs
- **File system performance**: Disk vs memory-based files


## Conclusion

File descriptors and I/O redirection are fundamental concepts in Unix-like systems that enable:

1. **Flexible I/O Control**: Programs can have their input/output redirected transparently
2. **Shell Functionality**: Foundation of command-line I/O operators (`>`, `<`, `|`)
3. **Process Communication**: Basis for pipes and inter-process communication
4. **System Programming**: Essential for servers, utilities, and system tools

Key insights from the implementations:

- **File descriptor inheritance** enables transparent I/O redirection
- **Close-and-open pattern** is the core mechanism for redirection
- **Modern abstractions** provide safer alternatives to raw system calls
- **Understanding low-level mechanics** helps with debugging and optimization

This knowledge is essential for:
- Shell scripting and command-line tools
- System programming and utility development
- Server and daemon programming  
- Container and orchestration systems
- Network programming and I/O multiplexing

The progression from unsafe system calls to safe Rust abstractions demonstrates both the power of low-level control and the benefits of modern programming language safety features, while maintaining the core understanding of how Unix I/O redirection actually works.