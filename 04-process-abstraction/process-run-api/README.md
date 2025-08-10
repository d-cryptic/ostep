# Process Scheduling Simulator - Experimental Analysis

This document contains the results and analysis of process scheduling experiments using the OSTEP process simulator.

## Overview

The simulator demonstrates key operating system concepts:
- **Process states**: RUNNING, READY, BLOCKED, DONE
- **CPU scheduling policies**: When to switch between processes
- **I/O completion policies**: How to handle processes that complete I/O operations
- **Resource utilization**: CPU and I/O device efficiency

## Experimental Results

### 1. CPU-Only Processes: `-l 5:100,5:100`

**Question**: What should the CPU utilization be with two CPU-only processes?

**Prediction**: 100% - both processes only use CPU, no I/O operations

**Result**: ✅ **CORRECT**
```
Stats: Total Time 10
Stats: CPU Busy 10 (100.00%)
Stats: IO Busy  0 (0.00%)
```

**Analysis**: 
- Process 0 runs completely (5 instructions), then Process 1 runs (5 instructions)
- No I/O operations = no idle CPU time
- Sequential execution with perfect CPU utilization

---

### 2. CPU + I/O Mix (CPU First): `-l 4:100,1:0`

**Question**: How long does it take with one CPU process (4 instructions) and one I/O process?

**Prediction**: 11 time units - 4 CPU + 1 I/O start + 5 I/O wait + 1 I/O completion

**Result**: ✅ **CORRECT**
```
Stats: Total Time 11
Stats: CPU Busy 6 (54.55%)
Stats: IO Busy  5 (45.45%)
```

**Analysis**:
- Process 0: 4 CPU instructions (time 1-4)
- Process 1: 1 I/O operation (start at time 5, complete at time 11)
- I/O blocks process for 5 time units
- CPU is idle during I/O wait period

---

### 3. Order Matters: `-l 1:0,4:100`

**Question**: Does switching the order (I/O first, then CPU) matter?

**Prediction**: Yes, should be faster due to overlapping I/O and CPU work

**Result**: ✅ **CORRECT**
```
Stats: Total Time 7
Stats: CPU Busy 6 (85.71%)
Stats: IO Busy  5 (71.43%)
```

**Analysis**:
- **Much more efficient!** (7 vs 11 time units)
- Process 0 starts I/O at time 1, switches to Process 1
- Process 1 runs 4 CPU instructions while Process 0's I/O completes
- **Parallelism**: I/O and CPU operations happen simultaneously
- **Key insight**: Order dramatically affects system efficiency

---

### 4. No Context Switching During I/O: `-l 1:0,4:100 -S SWITCH_ON_END`

**Question**: What happens when the system doesn't switch during I/O?

**Result**: **INEFFICIENT**
```
Time: 11 (vs 7 with switching)
CPU mostly idle during I/O
```

**Analysis**:
- Process 0 starts I/O, system waits without switching
- Process 1 only runs after Process 0 completely finishes
- **Wastes CPU cycles** - no parallelism between I/O and CPU work
- Demonstrates why context switching during I/O is crucial

---

### 5. Context Switching on I/O: `-l 1:0,4:100 -S SWITCH_ON_IO`

**Question**: What happens when system switches immediately on I/O?

**Result**: **EFFICIENT**
```
Stats: Total Time 7
Stats: CPU Busy 6 (85.71%)
Stats: IO Busy  5 (71.43%)
```

**Analysis**:
- Same result as experiment 3 (default behavior)
- Immediate context switch allows CPU/I/O parallelism
- **Optimal resource utilization**

---

### 6. I/O Run Later Policy: `-l 3:0,5:100,5:100,5:100 -I IO_RUN_LATER`

**Question**: How does delaying I/O process scheduling affect efficiency?

**Result**: **SUBOPTIMAL**
```
Stats: Total Time 31
Stats: CPU Busy 21 (67.74%)
Stats: IO Busy  15 (48.39%)
```

**Analysis**:
- Process 0 (I/O-heavy) gets deprioritized
- CPU-only processes run first, causing long delays
- **Poor resource utilization**: CPU idle during long I/O sequences
- **Convoy effect**: I/O processes wait behind CPU-bound processes

---

### 7. I/O Run Immediate Policy: `-l 3:0,5:100,5:100,5:100 -I IO_RUN_IMMEDIATE`

**Question**: What happens when I/O processes run immediately after I/O completion?

**Result**: **OPTIMAL**
```
Stats: Total Time 21
Stats: CPU Busy 21 (100.00%)
Stats: IO Busy  15 (71.43%)
```

**Analysis**:
- **Dramatic improvement**: 21 vs 31 time units (32% faster)
- **Perfect CPU utilization** (100%)
- I/O processes get priority, allowing better overlap
- **Key insight**: Prioritizing I/O-bound processes improves overall system throughput

---

### 8. Random Process Behavior: `-s 1 -l 3:50,3:50`

**Question**: How do random processes behave with different policies?

**Result**: 
```
Default (IO_RUN_LATER): 15 time units, 53.33% CPU
With IO_RUN_IMMEDIATE:  15 time units, 53.33% CPU
```

**Analysis**:
- In this specific case, both policies performed identically
- With only 2 processes and specific random seed, limited scheduling opportunities
- Different seeds would show more variation in policy effectiveness

## Key Insights and Principles

### 1. **I/O-First Scheduling Wins**
- Running I/O operations early enables CPU/I/O parallelism
- Order of process execution significantly impacts system performance

### 2. **Context Switching During I/O is Essential**
- `SWITCH_ON_IO` policy prevents CPU waste during I/O wait
- Without switching, systems become severely inefficient

### 3. **Prioritizing I/O-Bound Processes Improves Throughput**
- `IO_RUN_IMMEDIATE` policy reduces overall execution time
- I/O-bound processes should get scheduling priority for better resource utilization

### 4. **Resource Utilization Metrics Matter**
- CPU and I/O utilization percentages reveal scheduling effectiveness
- 100% CPU utilization with active I/O indicates optimal scheduling

### 5. **Real-World Implications**
- Modern operating systems use similar principles
- Interactive applications (I/O-heavy) get priority over batch jobs (CPU-heavy)
- Prevents "convoy effect" where short tasks wait behind long ones

## Scheduling Algorithm Comparison

| Policy | CPU Utilization | Total Time | Best For |
|--------|-----------------|------------|----------|
| SWITCH_ON_END | Low | High | Simple systems |
| SWITCH_ON_IO | High | Low | General purpose |
| IO_RUN_LATER | Medium | High | CPU-bound workloads |
| IO_RUN_IMMEDIATE | High | Low | Mixed workloads |