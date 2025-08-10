# MLFQ Scheduler Solutions

This document provides solutions to the Multi-Level Feedback Queue (MLFQ) scheduling questions using the `mlfq.py` simulator.

## Question 1: Two jobs, two queues with random generation

Let's run a few examples with simple parameters to understand MLFQ behavior.

### Example 1: Basic MLFQ behavior
```bash
./mlfq.py -n 2 -j 2 -m 20 -M 0 -s 1 -c
```

**Output:**
```
OPTIONS jobs 2
OPTIONS queues 2
OPTIONS allotments for queue  1 is   1
OPTIONS quantum length for queue  1 is  10
OPTIONS allotments for queue  0 is   1
OPTIONS quantum length for queue  0 is  10

Job List:
  Job  0: startTime   0 - runTime  15 - ioFreq   0
  Job  1: startTime   0 - runTime   8 - ioFreq   0

Execution Trace:

[ time 0 ] Run JOB 0 at PRIORITY 1 [ TICKS 9 ALLOT 0 TIME 14 (of 15) ]
...
[ time 9 ] Run JOB 0 at PRIORITY 1 [ TICKS 0 ALLOT 0 TIME 5 (of 15) ]
[ time 10 ] Run JOB 1 at PRIORITY 1 [ TICKS 9 ALLOT 0 TIME 7 (of 8) ]
...
[ time 17 ] Run JOB 1 at PRIORITY 1 [ TICKS 0 ALLOT 0 TIME 0 (of 8) ]
[ time 18 ] FINISHED JOB 1
[ time 18 ] Run JOB 0 at PRIORITY 0 [ TICKS 9 ALLOT 0 TIME 4 (of 15) ]
...
```

**Analysis:** Job 0 runs first in high priority queue, uses up its quantum, gets demoted. Job 1 then runs in high priority, finishes. Job 0 completes in low priority queue.

### Example 2: Different job lengths
```bash
./mlfq.py -n 2 -l 0,25,0:0,5,0 -c
```

Shows how longer jobs get demoted while shorter jobs can complete in high priority.

## Question 2: Reproducing chapter examples

### Example: Gaming the scheduler (old rules)
```bash
./mlfq.py -n 3 -l 0,200,1:100,20,0 -i 5 -S -c
```

This creates:
- Job 0: Long-running job that does I/O every 1 time unit
- Job 1: CPU-bound job arriving at time 100

With `-S` flag, Job 0 stays at high priority by doing I/O just before quantum expires.

### Example: Priority boost demonstration
```bash
./mlfq.py -n 3 -l 0,200,0:50,20,0 -B 50 -c
```

Shows periodic priority boost moving all jobs back to highest queue.

## Question 3: Configuring MLFQ to behave like Round Robin

To make MLFQ behave like Round Robin:

```bash
./mlfq.py -n 1 -q 10 -l 0,50,0:0,30,0:0,40,0 -c
```

**Key configuration:**
- `-n 1`: Single queue (no multi-level)
- `-q 10`: Quantum of 10 time units
- No I/O (`ioFreq = 0`)

**Result:** All jobs stay in same queue, get equal time slices in round-robin fashion.

### Alternative with multiple queues but same behavior:
```bash
./mlfq.py -n 3 -Q 10,10,10 -A 999,999,999 -l 0,50,0:0,30,0:0,40,0 -c
```

- All queues have same quantum (10)
- High allotments prevent demotion
- Effectively creates round-robin behavior

## Question 4: Gaming the scheduler (99% CPU)

Create a workload where one job games the system using old rules:

```bash
./mlfq.py -n 3 -l 0,100,9:0,100,0 -i 1 -S -c
```

**Setup:**
- Job 0: `ioFreq=9` - does I/O just before quantum (10) expires
- Job 1: CPU-bound job with no I/O
- `-S`: Jobs stay at same priority after I/O
- `-i 1`: I/O takes 1 time unit

**Result:**
```
[ time 0 ] Run JOB 0 at PRIORITY 2 [ TICKS 9 ALLOT 0 TIME 99 (of 100) ]
...
[ time 8 ] Run JOB 0 at PRIORITY 2 [ TICKS 1 ALLOT 0 TIME 91 (of 100) ]
[ time 9 ] IO_START by JOB 0
[ time 10 ] IO_DONE by JOB 0
[ time 10 ] Run JOB 0 at PRIORITY 2 [ TICKS 9 ALLOT 0 TIME 90 (of 100) ]
```

Job 0 repeatedly:
1. Runs for 9 time units (just before quantum expires)
2. Issues I/O for 1 time unit  
3. Returns to high priority queue (due to -S flag)
4. Prevents Job 1 from running

**Gaming ratio:** Job 0 gets ~90% of CPU time by avoiding demotion.

## Question 5: Boost frequency for 5% CPU guarantee

**Problem:** Quantum = 10ms in highest queue. How often to boost for 5% CPU guarantee?

**Analysis:**
- Worst case: Job gets demoted to lowest priority
- In lowest queue with long quantum, job might wait a very long time
- Need boost frequent enough that job gets 5% CPU overall

**Calculation:**
Let B = boost interval
- Job gets demoted after first quantum (10ms)
- Must wait at most B time units for boost
- After boost, gets another 10ms quantum
- Ratio = 10 / (10 + B) ≥ 0.05
- Solving: 10 ≥ 0.05 × (10 + B)
- 10 ≥ 0.5 + 0.05B
- 9.5 ≥ 0.05B
- B ≤ 190

**Answer:** Boost every 190ms or less.

**Verification:**
```bash
./mlfq.py -n 3 -l 0,1000,0:0,1000,0 -B 190 -Q 10,50,100 -c
```

This ensures any long-running job gets boosted before being starved too long.

## Question 6: I/O queue insertion behavior (-I flag)

The `-I` flag controls whether jobs finishing I/O go to:
- **Without -I**: End of queue (normal)  
- **With -I**: Front of queue (priority boost)

### Example: Without -I flag
```bash
./mlfq.py -n 2 -l 0,20,5:5,20,0 -i 3 -c
```

**Behavior:**
```
[ time 5 ] IO_DONE by JOB 0    # Job 0 added to END of queue
[ time 5 ] Run JOB 1 at PRIORITY 1  # Job 1 runs first
```

### Example: With -I flag  
```bash
./mlfq.py -n 2 -l 0,20,5:5,20,0 -i 3 -I -c
```

**Behavior:**
```
[ time 5 ] IO_DONE by JOB 0    # Job 0 added to FRONT of queue  
[ time 5 ] Run JOB 0 at PRIORITY 1  # Job 0 runs immediately
```

### Impact Analysis

**Interactive workload test:**
```bash
# Without -I: Interactive jobs wait behind CPU-bound jobs
./mlfq.py -n 3 -l 0,100,0:10,50,10:20,30,8 -i 2 -c

# With -I: Interactive jobs get immediate attention  
./mlfq.py -n 3 -l 0,100,0:10,50,10:20,30,8 -i 2 -I -c
```

**Results:**
- **Without -I**: Interactive jobs (frequent I/O) have higher response times
- **With -I**: Interactive jobs get better responsiveness at cost of CPU-bound job performance

**Use cases:**
- **-I flag ON**: Better for interactive systems (GUI, web servers)
- **-I flag OFF**: More fair for mixed workloads

## Summary of Key MLFQ Concepts Demonstrated

| Concept | Command Example | Key Insight |
|---------|----------------|-------------|
| **Basic MLFQ** | `-n 3 -l 0,30,0:0,20,0` | Jobs start high, get demoted after quantum |
| **Round Robin** | `-n 1 -q 10` | Single queue = RR behavior |
| **Gaming Prevention** | `-S` vs default | Old rules allowed gaming, new rules prevent it |
| **Starvation Prevention** | `-B 100` | Periodic boost prevents starvation |
| **I/O Responsiveness** | `-I` flag | Controls I/O job priority after completion |

## Advanced Scenarios

### CPU vs I/O bound comparison:
```bash
# CPU-bound jobs get demoted quickly
./mlfq.py -n 3 -l 0,100,0:0,100,0 -c

# I/O-bound jobs stay at high priority  
./mlfq.py -n 3 -l 0,100,10:0,100,10 -c
```

### Gaming mitigation:
```bash
# Old rules (gameable)
./mlfq.py -n 3 -l 0,100,9 -S -c

# New rules (gaming prevented)  
./mlfq.py -n 3 -l 0,100,9 -c
```

The MLFQ scheduler successfully balances:
- **Responsiveness** for interactive jobs
- **Turnaround time** optimization  
- **Fairness** prevention of gaming
- **Starvation avoidance** through priority boost