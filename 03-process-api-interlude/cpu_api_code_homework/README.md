# Process API Code Homework - Rust Solutions

This repository contains Rust implementations of process API homework exercises that demonstrate fundamental Unix system calls including `fork()`, `exec()`, `wait()`, pipes, and file descriptor manipulation.

## Overview

These exercises explore core operating system concepts through hands-on programming, showing how processes are created, managed, and communicate with each other. Each solution demonstrates different aspects of process control and inter-process communication.

## Project Structure

```
cpu_api_code_homework/
├── src/
│   ├── question.md     # Original homework questions
│   ├── main.rs         # Default entry point  
│   ├── sol1.rs         # Variable sharing between processes
│   ├── sol2.rs         # File descriptor sharing
│   ├── sol3.rs         # Process synchronization without wait()
│   ├── sol4.rs         # exec() family variants
│   ├── sol5.rs         # wait() behavior and return values
│   ├── sol6.rs         # waitpid() usage and advantages
│   ├── sol7.rs         # stdout closure effects
│   └── sol8.rs         # Inter-process pipes
├── Cargo.toml          # Dependencies (libc, thread)
└── shared_file.txt     # Output file from solution 2
```

## Questions and Solutions

### Question 1: Variable Sharing in fork() (`sol1.rs`)

**Question**: Write a program that calls fork(). Before calling fork(), have the main process access a variable (e.g., x) and set its value to something (e.g., 100). What value is the variable in the child process? What happens when both processes change the value?

**Key Concepts**:
- Process memory isolation
- Copy-on-write semantics
- Shared vs separate memory spaces

**Solution Analysis**:
```rust
let x = Arc::new(AtomicI32::new(100));
```

**Expected Behavior**:
- Child inherits parent's memory image with x = 100
- Changes in child don't affect parent (and vice versa)
- Each process has its own memory space after fork()
- Demonstrates fundamental process isolation

**Run**: `cargo run --bin sol1`

### Question 2: File Descriptor Sharing (`sol2.rs`)

**Question**: Write a program that opens a file and then calls fork(). Can both processes access the file descriptor? What happens when they write concurrently?

**Key Concepts**:
- File descriptor inheritance
- Concurrent file access
- Race conditions in I/O

**Solution Analysis**:
- File descriptors are inherited by child processes
- Both processes can write to the same file
- Output may be interleaved due to concurrent access
- Demonstrates need for synchronization in I/O

**Run**: `cargo run --bin sol2`
**Output**: Check `shared_file.txt` for interleaved content

### Question 3: Process Ordering Without wait() (`sol3.rs`)

**Question**: The child should print "hello", parent should print "goodbye". Ensure child prints first without using wait().

**Key Concepts**:
- Process scheduling
- Race conditions
- Synchronization techniques

**Solution Analysis**:
```rust
// Child executes immediately
println!("hello");

// Parent adds artificial delay
thread::sleep(Duration::from_millis(10));
println!("goodbye");
```

**Limitations**:
- Not guaranteed to work on all systems
- Depends on process scheduler behavior
- Demonstrates why proper synchronization is needed

**Run**: `cargo run --bin sol3`

### Question 4: exec() Family Variants (`sol4.rs`)

**Question**: Use fork() and different exec() variants to run /bin/ls. Try execl(), execle(), execlp(), execv(), execvp(), execvpe().

**Key Concepts**:
- Process replacement with exec()
- Different argument passing methods
- PATH searching vs absolute paths

**Solution Analysis**:
- **execl()**: Arguments as individual parameters
- **execv()**: Arguments as array/vector
- **execlp()**: Searches PATH for program
- **execvp()**: Vector arguments + PATH search
- **execle()**: Includes environment variables

**Why So Many Variants?**:
1. **Argument format**: List vs vector
2. **PATH searching**: Automatic vs manual
3. **Environment**: Inherit vs specify
4. **Convenience**: Different use cases require different interfaces

**Run**: `cargo run --bin sol4`

### Question 5: wait() Behavior (`sol5.rs`)

**Question**: Use wait() to wait for child process. What does wait() return? What happens if you use wait() in the child?

**Key Concepts**:
- Parent-child synchronization
- Process exit status handling
- Error conditions

**Solution Analysis**:
- **wait() returns**: PID of waited child, or -1 on error
- **Status information**: Exit code, signal information
- **Child calling wait()**: Fails with -1 (no children to wait for)
- **Status macros**: WIFEXITED(), WEXITSTATUS(), etc.

**Run**: `cargo run --bin sol5`

### Question 6: waitpid() Usage (`sol6.rs`)

**Question**: Use waitpid() instead of wait(). When would waitpid() be useful?

**Key Concepts**:
- Selective child waiting
- Non-blocking wait operations
- Multiple child management

**Solution Analysis**:
```rust
// Wait for specific child
libc::waitpid(child2, &mut status, 0);

// Non-blocking wait
libc::waitpid(-1, &mut status, libc::WNOHANG);
```

**waitpid() Advantages**:
1. **Selective waiting**: Wait for specific child by PID
2. **Non-blocking**: WNOHANG flag for immediate return
3. **Multiple children**: Better control over which child to wait for
4. **Signal handling**: More robust in signal-heavy environments

**Run**: `cargo run --bin sol6`

### Question 7: stdout Closure Effects (`sol7.rs`)

**Question**: Create child process, close stdout, then call printf(). What happens?

**Key Concepts**:
- File descriptor manipulation
- Standard stream redirection
- Output buffering behavior

**Solution Analysis**:
- Closing stdout makes printf() output invisible
- stderr remains available for error output
- Demonstrates file descriptor independence
- Shows why proper I/O handling is important

**Expected Output**:
- Messages before close(): visible
- Messages after close(): invisible
- stderr messages: still visible

**Run**: `cargo run --bin sol7`

### Question 8: Inter-Process Pipes (`sol8.rs`)

**Question**: Create two children, connect stdout of one to stdin of the other using pipe().

**Key Concepts**:
- Inter-process communication
- Pipe creation and management
- File descriptor redirection
- Process coordination

**Solution Analysis**:
```rust
// Create pipe
libc::pipe(pipe_fds.as_mut_ptr());

// Child 1: Writer
libc::dup2(write_fd, libc::STDOUT_FILENO);

// Child 2: Reader  
libc::dup2(read_fd, libc::STDIN_FILENO);
```

**Process Flow**:
1. Parent creates pipe (read_fd, write_fd)
2. Child 1 redirects stdout to write_fd
3. Child 2 redirects stdin to read_fd
4. Data flows from Child 1 → pipe → Child 2

**Run**: `cargo run --bin sol8`

## Building and Running

### Prerequisites
- Rust toolchain (cargo, rustc)
- Unix-like system (macOS, Linux) for system calls
- Standard Unix utilities (/bin/ls, /bin/cat)

### Individual Solutions
```bash
# Run specific solution
cargo run --bin sol1  # Question 1
cargo run --bin sol2  # Question 2
cargo run --bin sol3  # Question 3
cargo run --bin sol4  # Question 4
cargo run --bin sol5  # Question 5
cargo run --bin sol6  # Question 6
cargo run --bin sol7  # Question 7
cargo run --bin sol8  # Question 8
```

### Cargo.toml Configuration
Add binary entries for each solution:
```toml
[[bin]]
name = "sol1"
path = "src/sol1.rs"

[[bin]]
name = "sol2"
path = "src/sol2.rs"

# ... etc for sol3-sol8
```

### Build All Solutions
```bash
cargo build --release
```

## Key Learning Points

### Process Management
1. **Process Isolation**: fork() creates separate memory spaces
2. **Resource Inheritance**: File descriptors inherited but memory is not
3. **Process Coordination**: Multiple techniques for synchronization
4. **Clean Termination**: Proper cleanup and status handling

### System Calls
1. **fork()**: Process duplication with copy-on-write
2. **exec()**: Process replacement with different variants
3. **wait()/waitpid()**: Parent-child synchronization
4. **pipe()**: Inter-process communication channel

### File Descriptors
1. **Inheritance**: Child processes inherit open file descriptors
2. **Manipulation**: dup2() for redirection, close() for cleanup
3. **Standard streams**: stdin(0), stdout(1), stderr(2)
4. **Concurrent access**: Race conditions in shared file access

### Inter-Process Communication
1. **Pipes**: Unidirectional communication channels
2. **Redirection**: Connecting process output to input
3. **Buffering**: Understanding output buffering behavior
4. **Synchronization**: Coordinating multiple processes

## Real-World Applications

### Shell Implementation
- Process creation for command execution
- Pipe implementation for command chaining
- I/O redirection for file operations
- Job control and process management

### Server Development
- Fork-based server architecture
- Process pooling for request handling
- Inter-process communication
- Resource sharing and isolation

### System Utilities
- Process monitoring tools
- File processing pipelines
- System administration scripts
- Development and debugging tools

## Common Issues and Solutions

### Compilation Errors
```bash
# Missing libc dependency
cargo add libc

# Edition compatibility
edition = "2021"  # In Cargo.toml
```

### Runtime Issues
```bash
# Permission errors
chmod +x target/release/sol*

# Path issues
export PATH=/bin:/usr/bin:$PATH
```

### Platform Compatibility
- Code designed for Unix-like systems
- Windows requires different system calls
- Some features may behave differently on different Unix variants

## Advanced Topics

### Signal Handling
- SIGCHLD for asynchronous child termination
- Signal masks and handlers
- Interrupted system calls

### Process Groups
- Session and process group management
- Terminal control and job control
- Daemon process creation

### Modern Alternatives
- Threads for lightweight concurrency
- Async/await for I/O-bound operations
- Message queues for IPC
- Shared memory for high-performance communication

## Conclusion

These exercises demonstrate fundamental Unix process management concepts that form the foundation of:
- Operating system design
- Shell implementation
- Server architecture
- System programming

Understanding these concepts is essential for:
- System administrators
- Backend developers
- DevOps engineers
- Operating system developers
- Anyone working with Unix-like systems

The Rust implementations provide memory safety while maintaining direct access to system calls, offering a modern approach to learning classical system programming concepts.