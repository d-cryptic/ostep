# System Call and Context Switch Measurement

This directory contains tools for measuring the performance costs of fundamental operating system operations: system calls and context switches.

## Overview

Based on the homework assignment in `question.md`, this implementation follows the lmbench methodology to accurately measure OS overhead. The measurements help understand the performance characteristics of process management and system call interface.

## Contents

### measurement_code/
Rust implementation that measures:
1. **Timer Precision** - Determines system timer accuracy
2. **System Call Cost** - Measures system call overhead using null operations  
3. **Context Switch Cost** - Measures process switching overhead using inter-process communication

## Quick Start

```bash
cd measurement_code
cargo run --release
```

## Measurement Details

### Timer Precision Test
```rust
fn measure_timer_precision() -> u64
```
- Makes back-to-back `Instant::now()` calls
- Finds minimum non-zero time difference
- Determines timer resolution (typically 1-100 nanoseconds)
- Used to calculate required iterations for accurate measurements

### System Call Cost Measurement
```rust  
fn measure_syscall_cost(timer_precision: u64) -> u64
```
- **Target**: Measure pure system call overhead
- **Method**: Repeated 0-byte reads from `/dev/zero`
- **Why /dev/zero**: Eliminates I/O overhead, measures only kernel transition
- **Iterations**: Calculated based on timer precision (minimum 100,000)
- **Warm-up**: 1,000 calls to stabilize cache and branch prediction

**Key Implementation Points:**
- Uses empty buffer `[0u8; 0]` for true zero-byte reads
- Divides total time by iterations for per-call cost
- Accounts for timer precision in iteration calculation

### Context Switch Cost Measurement  
```rust
fn measure_context_switch_cost() -> u64
```
- **Target**: Measure process switching overhead
- **Method**: Ping-pong communication between parent/child via pipes
- **Mechanism**: 
  1. Parent writes to pipe1, child reads (context switch)
  2. Child writes to pipe2, parent reads (context switch)
  3. Each iteration = 2 context switches

**Process Flow:**
```
Parent Process          Child Process
     |                       |
  write(pipe1) ----------> read(pipe1)
     |                       |  
  read(pipe2) <----------- write(pipe2)
     |                       |
   (repeat 100,000 times)
```

**Why This Works:**
- When parent writes and waits to read, it blocks
- OS switches to child process (context switch #1)
- When child writes and tries to read again, it blocks  
- OS switches back to parent (context switch #2)
- Forced context switching due to blocking I/O

## Technical Implementation

### Cross-Platform Compatibility
- **Original Design**: Linux with `sched_setaffinity()` for CPU binding
- **Current Version**: macOS compatible (CPU affinity removed)
- **Trade-off**: Slightly less precise on multi-core systems

### Removed Linux-Specific Code:
```rust
// Not available on macOS
fn set_cpu_affinity(cpu: usize) {
    let mut cpu_set: libc::cpu_set_t = std::mem::zeroed();  // ❌
    libc::CPU_SET(cpu, &mut cpu_set);                       // ❌  
    libc::sched_setaffinity(0, size_of_val(&cpu_set), &cpu_set); // ❌
}
```

### Pipe Communication Details
```rust
// Create bidirectional communication
let mut pipe1 = [0; 2];  // Parent -> Child
let mut pipe2 = [0; 2];  // Child -> Parent

// Each process closes unused ends
// Parent: close(read1), close(write2)  
// Child:  close(write1), close(read2)
```

## Sample Output

```
System Call and Context Switch Cost Measurement

Timer precision: 42 nanoseconds
System call cost: 156 nanoseconds  
Context switch cost: 2847 nanoseconds
```

## Interpretation

### Typical Results:
- **Timer precision**: 1-100ns (system dependent)
- **System call cost**: 50-500ns (architecture/OS dependent)
- **Context switch cost**: 1-10μs (depends on process state size)

### Factors Affecting Results:
- **CPU Architecture**: x86 vs ARM, cache sizes, pipeline depth
- **Operating System**: Kernel design, optimization level
- **System Load**: Background processes, thermal throttling
- **Memory Pressure**: Available RAM, swap usage

## Accuracy Considerations

### Sources of Measurement Error:
1. **Timer Resolution**: Limited by system clock precision
2. **Cache Effects**: Cold vs warm instruction/data cache
3. **Branch Prediction**: CPU learning access patterns during warm-up
4. **Scheduler Variance**: Without CPU affinity, processes may migrate
5. **System Interference**: Other processes competing for resources

### Mitigation Strategies:
1. **Large Iteration Counts**: Reduces statistical variance
2. **Warm-up Phases**: Stabilizes cache and prediction state  
3. **Multiple Runs**: Average results across multiple executions
4. **System Isolation**: Run on idle systems when possible