# Operating Systems: Three Easy Pieces - Introduction Programs

This repository contains both C/C++ and Rust implementations of the introductory programs from the OSTEP (Operating Systems: Three Easy Pieces) textbook. These programs demonstrate fundamental operating system concepts including process execution, memory management, concurrency, and I/O operations.

## Programs Overview

### 1. CPU Program (`cpu.cpp` / `cpu.rs`)
**Purpose**: Demonstrates basic process behavior and CPU scheduling concepts.

**What it does**:
- Takes a string as command-line argument
- Runs an infinite loop printing the string every second
- Uses busy-waiting to consume CPU cycles predictably

**Why it exists**:
- Shows how processes consume CPU time
- Useful for observing CPU scheduling with system monitoring tools (`top`, `htop`)
- Demonstrates process lifecycle and resource consumption patterns

**C++ Usage**:
```bash
make cpu
./cpu "Hello OS"
```

**Rust Usage**:
```bash
rustc cpu.rs -o cpu
./cpu "Hello Rust"
```

### 2. Memory Program (`mem.cpp` / `mem.rs`) 
**Purpose**: Demonstrates memory allocation and process memory management.

**What it does**:
- Allocates memory on the heap for an integer
- Takes initial value as command-line argument
- Continuously increments and displays the value with process ID
- Shows memory address and value changes over time

**Why it exists**:
- Illustrates heap memory allocation (`malloc` in C++, `Box` in Rust)
- Shows how processes have separate memory spaces
- Useful for understanding virtual memory concepts
- Demonstrates process identification with PIDs

**C++ Usage**:
```bash
make mem
./mem 42
```

**Rust Usage**:
```bash
rustc mem.rs -o mem  
./mem 42
```

### 3. Threads Program (`threads.c` / `threads.rs`)
**Purpose**: Demonstrates concurrency issues and race conditions.

**What it does**:
- Creates two threads that increment a shared counter
- Each thread increments the counter N times (N = command-line argument)
- Expected result: 2×N, Actual result: < 2×N (due to race conditions)

**Why it exists**:
- Shows the fundamental problem of concurrent access to shared data
- Demonstrates race conditions and lost updates
- Illustrates why synchronization primitives are necessary
- Educational tool for understanding thread safety

**C++ Usage**:
```bash
make threads
./threads 100000
# Run multiple times to see different results
```

**Rust Usage**:
```bash
rustc threads.rs -o threads
./threads 100000
# Shows race conditions using unsafe code
```

### 4. I/O Program (`io.c` / `io.rs`)
**Purpose**: Demonstrates basic file I/O operations.

**What it does**:
- Creates/opens a file (`/tmp/file`)
- Writes "hello world\n" to the file
- Syncs data to disk and closes the file

**Why it exists**:
- Shows basic file system operations
- Demonstrates I/O system calls
- Illustrates file creation, writing, and synchronization
- Foundation for understanding file system concepts

**C++ Usage**:
```bash
make io
./io
cat /tmp/file
```

**Rust Usage**:
```bash
rustc io.rs -o io
./io
cat /tmp/file
```

## Libraries and System Calls Used

### C/C++ Libraries
- **`<sys/time.h>`** - High-precision timing with `gettimeofday()`
- **`<stdio.h>`** - Standard I/O operations (`printf`, `fprintf`)
- **`<stdlib.h>`** - Memory allocation (`malloc`), process control (`exit`)
- **`<unistd.h>`** - POSIX API (`getpid`, `write`, `close`)
- **`<pthread.h>`** - POSIX threads for concurrent programming
- **`<fcntl.h>`** - File control operations (`open` flags)

### Rust Libraries
- **`std::time`** - System time and duration measurements
- **`std::thread`** - Native threading support
- **`std::sync`** - Synchronization primitives (`Arc`, `Mutex`, `AtomicUsize`)
- **`std::fs`** - File system operations
- **`std::io`** - I/O traits and error handling
- **`std::process`** - Process-related functionality

## C++ to Rust Translation Differences

### Memory Safety
- **C++**: Manual memory management with `malloc`/`free`, potential null pointers
- **Rust**: Automatic memory management with `Box`, ownership system prevents leaks

### Error Handling
- **C++**: Return codes with `assert()` for error checking
- **Rust**: `Result<T, E>` types with `unwrap_or_else()` or `expect()` for error handling

### Threading
- **C++**: POSIX threads (`pthread_create`, `pthread_join`)
- **Rust**: Native `std::thread` with move semantics and type safety

### Concurrency Safety
- **C++**: Volatile variables, manual synchronization, undefined behavior on races
- **Rust**: Type system prevents data races, `unsafe` blocks required for race conditions

### String Handling
- **C++**: C-style strings with potential buffer overflows
- **Rust**: UTF-8 strings with bounds checking and memory safety

## Prerequisites

- **C++**: GCC compiler, POSIX-compliant system, pthread library
- **Rust**: Rust toolchain (rustc), standard library