# Chapter 13: Address Space Abstraction - Programs

This directory contains several programs designed to explore and understand how operating systems implement the address space abstraction. These tools help visualize memory layout, allocation patterns, and the differences between virtual and physical memory.

## Programs Overview

### 1. memory_user
**Purpose**: Basic memory allocation and usage demonstration program

**Location**: `./memory_user/`

**Description**: 
A simple Rust program that allocates a specified amount of memory and continuously accesses it. This helps demonstrate:
- How virtual memory allocation appears in process memory maps
- The relationship between allocated memory and physical memory usage
- Memory patterns visible through tools like `pmap` (Linux) or `vmmap` (macOS)

**Usage**:
```bash
cd memory_user
cargo build --release
./target/release/memory_user <memory_in_mb>
```

**Example**:
```bash
./target/release/memory_user 100  # Allocates 100MB
```

---

### 2. memory_user_updated
**Purpose**: Enhanced memory allocation tool with detailed monitoring capabilities

**Location**: `./memory_user_updated/`

**Description**: 
An improved version of memory_user that provides:
- Process ID (PID) display for easy monitoring
- Optional duration parameter for timed execution
- Initial memory touch to ensure all pages are resident
- Progress indicators showing continuous memory access
- Clean termination after specified duration

**Features**:
- Displays PID for use with `pmap -x <pid>` or `vmmap <pid>`
- Touches all allocated memory to ensure full residency
- Continuously modifies memory to maintain dirty pages
- Optional automatic termination after specified seconds

**Usage**:
```bash
cd memory_user_updated
cargo build --release
./target/release/memory_user_updated <memory_in_mb> [duration_in_seconds]
```

**Examples**:
```bash
# Allocate 100MB and run indefinitely
./target/release/memory_user_updated 100

# Allocate 200MB and run for 60 seconds
./target/release/memory_user_updated 200 60
```

**Monitoring Memory**:
```bash
# In another terminal, while the program is running:

# Linux:
pmap -x <PID>        # Basic extended format
pmap -X <PID>        # Detailed format with PSS, Referenced, Anonymous
pmap -d <PID>        # Device format

# macOS:
vmmap <PID>          # Full memory map
vmmap -summary <PID> # Summary only
```

---

### 3. location_of_routine
**Purpose**: Demonstrates the memory layout of different program components

**Location**: `./location_of_routine/`

**Description**: 
A program that reveals where different parts of a program reside in the address space:
- Code (text segment)
- Global variables (data segment)
- Heap allocations
- Stack variables
- Function addresses

This program has three different implementations:
- `main.rs`: Primary implementation showing basic memory regions
- `box.rs`: Demonstrates heap allocation using Box
- `diff_mem_region.rs`: Shows differences between memory regions

**Usage**:
```bash
cd location_of_routine
cargo build --release

# Run the main program
./target/release/location_of_routine

# Or run specific examples
cargo run --bin box
cargo run --bin diff_mem_region
```

**What it demonstrates**:
- Address space layout (code, data, heap, stack)
- ASLR (Address Space Layout Randomization) effects
- Distance between different memory regions
- Virtual address patterns

---

## Key Concepts Demonstrated

### 1. Virtual vs Physical Memory
- Programs see virtual addresses (e.g., 0x7fff...)
- OS maps these to physical RAM transparently
- Multiple processes can have same virtual addresses

### 2. Memory Regions
Traditional model:
- **Text**: Program code (read-only, executable)
- **Data**: Initialized global variables
- **BSS**: Uninitialized global variables
- **Heap**: Dynamic allocations (grows upward)
- **Stack**: Local variables, function calls (grows downward)

Modern reality (revealed by pmap/vmmap):
- Multiple heap zones (MALLOC_TINY, SMALL, LARGE on macOS)
- Shared libraries mapped into address space
- Guard pages for security
- Thread-local storage
- Memory-mapped files
- GPU buffers
- JIT code regions

### 3. Memory Metrics
- **VSZ/VSIZE**: Virtual memory size (address space allocated)
- **RSS**: Resident Set Size (physical memory in use)
- **Dirty**: Modified pages since loaded
- **Shared**: Memory shared with other processes
- **Private**: Memory exclusive to this process

### 4. Observable Patterns

When running these programs with different memory sizes:
```bash
# Small allocation (50MB)
Physical footprint: ~51MB (1MB overhead)

# Medium allocation (100MB)  
Physical footprint: ~101MB (1MB overhead)
Single MALLOC_LARGE region

# Large allocation (200MB)
Physical footprint: ~201MB (1MB overhead)
Split into multiple regions (128MB + 72MB)
```

## Platform Differences

### Linux (pmap)
```bash
pmap -x <PID>  # Shows RSS, Dirty, Mode, Mapping
pmap -X <PID>  # Adds PSS, Referenced, Anonymous, Swap
```

Output shows:
- Single [heap] region that grows
- .so shared libraries
- [stack] region
- [vdso] kernel interface

### macOS (vmmap)
```bash
vmmap <PID>    # Detailed memory map
```

Output shows:
- Multiple MALLOC zones (NANO, TINY, SMALL, LARGE)
- .dylib shared libraries
- Guard pages explicitly
- More detailed metadata

## Educational Exercises

1. **Memory Growth Observation**:
   ```bash
   # Run program with increasing sizes
   for size in 10 50 100 200 500; do
     ./memory_user_updated $size 30 &
     PID=$!
     sleep 2
     echo "=== ${size}MB ==="
     vmmap $PID | grep "Physical footprint"
     kill $PID
   done
   ```

2. **Compare Virtual vs Resident Memory**:
   ```bash
   # Start program
   ./memory_user_updated 1000 &
   PID=$!
   
   # Check immediately (before memory touched)
   vmmap $PID | grep "Physical"
   
   # Check after initial touch
   sleep 5
   vmmap $PID | grep "Physical"
   ```

3. **Observe Memory Regions**:
   ```bash
   ./location_of_routine
   # Note the addresses for code, data, heap, stack
   # Run again and observe ASLR changing addresses
   ```

## Building All Programs

```bash
# Build all programs at once
for dir in memory_user memory_user_updated location_of_routine; do
  echo "Building $dir..."
  (cd $dir && cargo build --release)
done
```

## Requirements

- Rust toolchain (rustc, cargo)
- Operating System: Linux or macOS
- Tools for observation:
  - Linux: `pmap`, `free`, `ps`
  - macOS: `vmmap`, `vm_stat`, `ps`

## Learning Objectives

By using these programs, you will understand:
1. How the OS provides each process with its own address space
2. The difference between virtual and physical memory
3. How memory allocation actually works under the hood
4. Why "using 16GB of virtual memory" doesn't mean using 16GB of RAM
5. How modern systems organize memory for security and efficiency
6. The complexity hidden by the simple malloc/free interface

## Related Files

- `solution.md`: Detailed answers to OSTEP Chapter 13 questions
- `questions.md`: Original questions from the textbook
- `free.man.md`: Documentation about the `free` command

## Further Exploration

Try these experiments:
1. Run multiple instances simultaneously and check total system memory
2. Use `strace` (Linux) or `dtruss` (macOS) to see system calls
3. Write to only part of allocated memory and observe RSS differences
4. Compare memory layouts between different programming languages
5. Observe memory behavior under system memory pressure

## References

- [Operating Systems: Three Easy Pieces - Chapter 13](http://pages.cs.wisc.edu/~remzi/OSTEP/)
- `man pmap` (Linux)
- `man vmmap` (macOS)
- `man 2 mmap` - Memory mapping system call
- `man 3 malloc` - Memory allocation functions