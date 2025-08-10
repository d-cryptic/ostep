# OSINT: Operating Systems Implementation and Theory

A comprehensive collection of implementations, solutions, and educational materials based on the "Operating Systems: Three Easy Pieces" (OSTEP) textbook by Remzi and Andrea Arpaci-Dusseau.

## Introduction

This repository contains practical implementations, homework solutions, and deep-dive explorations of fundamental operating systems concepts. Each chapter's content includes:

- **Code implementations** in Rust, C, Python, and assembly
- **Detailed explanations** and documentation  
- **Solution walkthroughs** for textbook problems
- **Performance analysis** and benchmarking tools
- **Educational examples** demonstrating core OS principles

The goal is to bridge theory and practice, providing hands-on experience with operating systems concepts while maintaining academic rigor and real-world applicability.

## Repository Index

### 📖 Process Management
- **[02-process-abstraction/](./02-process-abstraction/)**
  - [`xv6-proc-struct/`](./02-process-abstraction/xv6-proc-struct/) - xv6 process structure implementation in Rust
  
- **[03-process-api-interlude/](./03-process-api-interlude/)**
  - [`cpu_api_code_homework/`](./03-process-api-interlude/cpu_api_code_homework/) - Process API exercises and fork() demonstrations

- **[04-limited-directed-execution/](./04-limited-directed-execution/)**
  - [`xv6-context-switch-code/`](./04-limited-directed-execution/xv6-context-switch-code/) - Assembly implementation of xv6 context switching
  - [`measurement/`](./04-limited-directed-execution/measurement/) - System call and context switch performance measurement tools

### ⚖️ Scheduling Algorithms  
- **[07-scheduling-intro/](./07-scheduling-intro/)**
  - Scheduler simulator solutions (FIFO, SJF, RR)
  - Performance comparison and analysis

- **[08-scheduling-mlfq/](./08-scheduling-mlfq/)**
  - Multi-Level Feedback Queue (MLFQ) scheduler implementation
  - Gaming prevention and priority boost analysis

- **[09-scheduling-proportional-share/](./09-scheduling-proportional-share/)**
  - [`lottery_scheduling_decision/`](./09-scheduling-proportional-share/lottery_scheduling_decision/) - Lottery scheduling implementation in Rust
  - [`stride_scheduling/`](./09-scheduling-proportional-share/stride_scheduling/) - Stride scheduling with deterministic fairness
  - [`lottery_scheduling_assignment/`](./09-scheduling-proportional-share/lottery_scheduling_assignment/) - Homework solutions and analysis

- **[10-scheduling-multi-cpus/](./10-scheduling-multi-cpus/)**
  - Multi-CPU scheduling with cache affinity
  - Load balancing and work stealing strategies
  - Super-linear speedup analysis

## Key Features

### 🛠️ Implementation Quality
- **Production-ready code** with comprehensive error handling
- **Cross-platform compatibility** (Linux, macOS, Windows where applicable)
- **Extensive testing** with unit tests and integration tests
- **Performance benchmarks** for comparing different approaches


### 🔬 Research & Analysis
- **Comparative studies** between different scheduling algorithms
- **Performance measurements** of actual system calls and context switches
- **Cache effects** and multi-CPU considerations
- **Gaming prevention** and fairness guarantees

## Getting Started

### Prerequisites
- **Rust** (latest stable) - for Rust implementations
- **Python 3.x** - for simulator scripts
- **GCC/Clang** - for C/assembly code
- **Git** - for version control

### Quick Start
```bash
# Clone the repository
git clone <repository-url>
cd osint

# Example: Run lottery scheduling demo
cd 09-scheduling-proportional-share/lottery_scheduling_decision
cargo run

# Example: Measure system call overhead
cd 04-limited-directed-execution/measurement/measurement_code
cargo run --release

# Example: Test scheduler performance
cd 07-scheduling-intro
python scheduler.py -p RR -l 50,100,150 -c
```

### Building All Projects
```bash
# Build all Rust projects
find . -name "Cargo.toml" -execdir cargo build \;

# Run all tests
find . -name "Cargo.toml" -execdir cargo test \;
```

## Project Structure

Each directory follows a consistent structure:
```
chapter-name/
├── README.md           # Chapter overview and concepts
├── implementation/     # Core implementations
├── homework/          # Textbook problem solutions  
├── analysis/          # Performance studies
└── examples/          # Educational demonstrations
```


## Summary

This repository transforms the theoretical knowledge from "Operating Systems: Three Easy Pieces" into practical, runnable code. Whether you're a student learning operating systems concepts, an educator looking for teaching materials, or a practitioner wanting to understand OS internals, these implementations provide a solid foundation for exploration and learning.

The code demonstrates not just *how* operating systems work, but *why* they work that way, with performance measurements, comparative analysis, and real-world considerations that bridge the gap between academic theory and practical implementation.

Key highlights include:
- **Complete scheduler implementations** (FIFO, SJF, RR, MLFQ, Lottery, Stride)
- **Performance measurement tools** for system calls and context switches
- **Multi-CPU scheduling** with cache affinity considerations
- **Assembly-level context switching** from xv6 operating system
- **Comprehensive documentation** with examples and analysis

---

# Citation and Links

- **Book** - [Operating Systems: Three Easy Pieces](https://pages.cs.wisc.edu/~remzi/OSTEP/)
- **Official Code** - [GitHub - remzi-arpacidusseau/ostep-code: Code from various chapters in OSTEP (http://www.ostep.org)](https://github.com/remzi-arpacidusseau/ostep-code)
- **Official Project/Assignments Recommendations** - [GitHub - remzi-arpacidusseau/ostep-projects: Projects for an undergraduate OS course](https://github.com/remzi-arpacidusseau/ostep-projects)
- **Official Simulated Assignments** - [GitHub - remzi-arpacidusseau/ostep-homework](https://github.com/remzi-arpacidusseau/ostep-homework)
- **Table of Content** - [toc.pdf](https://pages.cs.wisc.edu/~remzi/OSTEP/toc.pdf)

## Citation

```
Operating Systems: Three Easy Pieces
Remzi H. Arpaci-Dusseau and Andrea C. Arpaci-Dusseau  
Arpaci-Dusseau Books  
November, 2023 (Version 1.10)
```