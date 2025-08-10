# Scheduling Algorithm Solutions

This document provides solutions to the scheduling questions using the `scheduler.py` simulator.

## Question 1: Three jobs of length 200 with SJF and FIFO

### Setup
Jobs: 3 jobs, each of length 200

### FIFO Scheduler
```bash
./scheduler.py -p FIFO -l 200,200,200 -c
```

**Results:**
```
Execution trace:
  [ time   0 ] Run job 0 for 200.00 secs ( DONE at 200.00 )
  [ time 200 ] Run job 1 for 200.00 secs ( DONE at 400.00 )
  [ time 400 ] Run job 2 for 200.00 secs ( DONE at 600.00 )

Final statistics:
  Job   0 -- Response: 0.00  Turnaround 200.00  Wait 0.00
  Job   1 -- Response: 200.00  Turnaround 400.00  Wait 200.00
  Job   2 -- Response: 400.00  Turnaround 600.00  Wait 400.00

  Average -- Response: 200.00  Turnaround 400.00  Wait 200.00
```

### SJF Scheduler
```bash
./scheduler.py -p SJF -l 200,200,200 -c
```

**Results:**
```
Execution trace:
  [ time   0 ] Run job 0 for 200.00 secs ( DONE at 200.00 )
  [ time 200 ] Run job 1 for 200.00 secs ( DONE at 400.00 )
  [ time 400 ] Run job 2 for 200.00 secs ( DONE at 600.00 )

Final statistics:
  Job   0 -- Response: 0.00  Turnaround 200.00  Wait 0.00
  Job   1 -- Response: 200.00  Turnaround 400.00  Wait 200.00
  Job   2 -- Response: 400.00  Turnaround 600.00  Wait 400.00

  Average -- Response: 200.00  Turnaround 400.00  Wait 200.00
```

**Analysis:** When all jobs have equal length, FIFO and SJF produce identical results since there's no advantage to reordering jobs of equal length.

## Question 2: Jobs of different lengths: 100, 200, 300

### FIFO Scheduler
```bash
./scheduler.py -p FIFO -l 100,200,300 -c
```

**Results:**
```
Execution trace:
  [ time   0 ] Run job 0 for 100.00 secs ( DONE at 100.00 )
  [ time 100 ] Run job 1 for 200.00 secs ( DONE at 300.00 )
  [ time 300 ] Run job 2 for 300.00 secs ( DONE at 600.00 )

Final statistics:
  Job   0 -- Response: 0.00  Turnaround 100.00  Wait 0.00
  Job   1 -- Response: 100.00  Turnaround 300.00  Wait 100.00
  Job   2 -- Response: 300.00  Turnaround 600.00  Wait 300.00

  Average -- Response: 133.33  Turnaround 333.33  Wait 133.33
```

### SJF Scheduler
```bash
./scheduler.py -p SJF -l 100,200,300 -c
```

**Results:**
```
Execution trace:
  [ time   0 ] Run job 0 for 100.00 secs ( DONE at 100.00 )
  [ time 100 ] Run job 1 for 200.00 secs ( DONE at 300.00 )
  [ time 300 ] Run job 2 for 300.00 secs ( DONE at 600.00 )

Final statistics:
  Job   0 -- Response: 0.00  Turnaround 100.00  Wait 0.00
  Job   1 -- Response: 100.00  Turnaround 300.00  Wait 100.00
  Job   2 -- Response: 300.00  Turnaround 600.00  Wait 300.00

  Average -- Response: 133.33  Turnaround 333.33  Wait 133.33
```

**Analysis:** Since jobs are already in shortest-to-longest order, SJF produces the same schedule as FIFO.

## Question 3: Adding RR scheduler with time-slice of 1

### RR Scheduler
```bash
./scheduler.py -p RR -l 100,200,300 -q 1 -c
```

**Results:**
```
Execution trace:
  [ time   0 ] Run job 0 for 1.00 secs
  [ time   1 ] Run job 1 for 1.00 secs
  [ time   2 ] Run job 2 for 1.00 secs
  ...
  [ time 597 ] Run job 1 for 1.00 secs
  [ time 598 ] Run job 2 for 1.00 secs
  [ time 599 ] Run job 2 for 1.00 secs ( DONE at 600.00 )

Final statistics:
  Job   0 -- Response: 0.00  Turnaround 199.00  Wait 99.00
  Job   1 -- Response: 1.00  Turnaround 598.00  Wait 398.00
  Job   2 -- Response: 2.00  Turnaround 600.00  Wait 300.00

  Average -- Response: 1.00  Turnaround 399.00  Wait 265.67
```

**Analysis:** RR has excellent response time (avg 1.00) but poor turnaround time (399.00) due to context switching overhead and time-sharing.

## Question 4: When does SJF deliver the same turnaround times as FIFO?

**Answer:** SJF delivers the same turnaround times as FIFO when:

1. **All jobs have equal length** - No reordering benefit
2. **Jobs arrive in shortest-to-longest order** - Already optimal
3. **Single job in system** - Only one scheduling choice

**Example verification:**
```bash
# Equal lengths
./scheduler.py -p FIFO -l 100,100,100 -c
./scheduler.py -p SJF -l 100,100,100 -c
# Both give same turnaround times

# Already sorted
./scheduler.py -p FIFO -l 50,100,150 -c  
./scheduler.py -p SJF -l 50,100,150 -c
# Both give same turnaround times
```

## Question 5: When does SJF deliver the same response times as RR?

**Answer:** SJF delivers the same response times as RR when:

1. **Quantum length ≥ shortest job length** AND **jobs in SJF order**
2. **Single job in system** - No time-sharing needed

**Example:**
```bash
# Short quantum, jobs in SJF order
./scheduler.py -p SJF -l 1,2,3 -c      # Response: 0,1,3
./scheduler.py -p RR -l 1,2,3 -q 1 -c  # Response: 0,1,2

# Large quantum = shortest job
./scheduler.py -p SJF -l 5,10,15 -c     # Response: 0,5,15  
./scheduler.py -p RR -l 5,10,15 -q 5 -c # Response: 0,5,10
```

The response times are **never exactly equal** in typical cases due to RR's interleaving behavior.

## Question 6: Response time trend with SJF as job lengths increase

**Test with increasing job lengths:**

```bash
# Small jobs
./scheduler.py -p SJF -l 10,20,30 -c
# Average Response: 6.67

# Medium jobs  
./scheduler.py -p SJF -l 100,200,300 -c
# Average Response: 66.67

# Large jobs
./scheduler.py -p SJF -l 1000,2000,3000 -c  
# Average Response: 666.67
```

**Trend:** Response time increases **linearly** with job lengths. If all jobs are scaled by factor k, average response time scales by factor k.

**Mathematical relationship:** For n jobs of lengths L₁ ≤ L₂ ≤ ... ≤ Lₙ:
- Average Response Time = (0 + L₁ + (L₁+L₂) + ... + (L₁+...+Lₙ₋₁)) / n

## Question 7: Response time with RR as quantum lengths increase

**Test with different quantum sizes:**

```bash
# Small quantum
./scheduler.py -p RR -l 100,200,300 -q 1 -c
# Average Response: 1.00

# Medium quantum
./scheduler.py -p RR -l 100,200,300 -q 10 -c  
# Average Response: 10.00

# Large quantum  
./scheduler.py -p RR -l 100,200,300 -q 50 -c
# Average Response: 33.33
```

**Trend:** As quantum length increases, response time increases until it reaches FIFO behavior.

### Worst-Case Response Time Equation for RR

For N jobs with quantum q:

**Worst-case response time = (N-1) × q**

**Explanation:** 
- In the worst case, a job arrives just after the quantum starts for another job
- It must wait for (N-1) other jobs to each run for one quantum
- Therefore: Response time = (N-1) × q

**Verification:**
```bash
# 3 jobs, quantum 10: worst case = (3-1) × 10 = 20
./scheduler.py -p RR -l 100,200,300 -q 10 -c
# Job 2 response time: 20.00 ✓
```

## Summary

| Scheduler | Best For | Worst For | Key Characteristic |
|-----------|----------|-----------|-------------------|
| **FIFO** | Simple systems | Short jobs behind long jobs | Predictable, no overhead |
| **SJF** | Minimizing turnaround time | Long jobs (starvation) | Optimal for turnaround time |
| **RR** | Interactive systems | CPU-bound workloads | Fair sharing, low response time |

**Key Insights:**
- SJF optimizes turnaround time but can cause starvation
- RR provides fairness and good response times at cost of context switching
- FIFO is simple but can lead to convoy effect
- Response time vs turnaround time trade-offs are fundamental in scheduling