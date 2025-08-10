# Stride Scheduling Implementation

## What

This is a Rust implementation of stride scheduling, a deterministic proportional-share CPU scheduling algorithm. Unlike lottery scheduling, stride scheduling provides perfect proportional fairness by tracking each job's "pass" value and always selecting the job with the minimum pass for execution.

## Why

Stride scheduling was developed to address limitations of lottery scheduling:

- **Deterministic Fairness**: Eliminates randomness for predictable resource allocation
- **Perfect Proportionality**: Guarantees exact proportional sharing over time
- **Low Variance**: Minimizes short-term unfairness compared to random selection
- **Efficient**: O(log n) scheduling decisions using priority queues

### Advantages over Lottery Scheduling
- **No luck-based variance**: Jobs get exactly their fair share
- **Better worst-case guarantees**: Bounded unfairness
- **Predictable behavior**: Scheduling decisions are deterministic

## Core Algorithm

### The Four-Step Process (`main.rs:68-91`)

```rust
fn schedule(&mut self) -> Option<u32> {
    // curr = remove_min(queue); // pick client with min pass
    let mut curr = self.queue.pop()?;
    
    // schedule(curr); // run for quantum  
    // (Job runs for quantum time)
    
    // curr->pass += curr->stride; // update pass using stride
    curr.pass += curr.stride;
    
    // insert(queue, curr); // return curr to queue
    self.queue.push(curr.clone());
    
    Some(curr.id)
}
```

### Key Concepts

#### Pass Value (`main.rs:8`)
- **Global Progress**: Tracks how much "virtual time" each job has consumed
- **Selection Criterion**: Job with minimum pass value runs next
- **Fairness Metric**: Difference in pass values indicates scheduling debt

#### Stride Value (`main.rs:13-18`)
```rust
fn new(id: u32, tickets: u32, large_number: u32) -> Self {
    let stride = if tickets > 0 { 
        large_number / tickets 
    } else { 
        large_number 
    };
    // ...
}
```
- **Inverse Priority**: `stride = large_number / tickets`
- **Progress Rate**: How much pass increases per quantum
- **Resource Share**: Lower stride = higher priority = more CPU time

## Data Structures

### Job Structure (`main.rs:4-26`)
```rust
struct Job {
    id: u32,           // Unique job identifier
    tickets: u32,      // Ticket allocation (priority)
    pass: u32,         // Current pass value (virtual time)
    stride: u32,       // Pass increment per quantum
}
```

### Scheduler Structure (`main.rs:43-55`)
```rust
struct StrideScheduler {
    queue: BinaryHeap<Job>,  // Min-heap ordered by pass value
    large_number: u32,       // Constant for stride calculation (10,000)
    quantum: u32,            // Time slice duration
}
```

## Example Execution

### Setup
```rust
scheduler.add_job(1, 100); // Job 1: 100 tickets, stride = 100
scheduler.add_job(2, 50);  // Job 2: 50 tickets, stride = 200  
scheduler.add_job(3, 25);  // Job 3: 25 tickets, stride = 400
```

### Execution Trace
```
--- Round 1 ---
Scheduled Job 1 (pass=0, stride=100)
  Job 1: pass=0 → 100

--- Round 2 ---  
Scheduled Job 2 (pass=0, stride=200)
  Job 2: pass=0 → 200

--- Round 3 ---
Scheduled Job 3 (pass=0, stride=400)  
  Job 3: pass=0 → 400

--- Round 4 ---
Scheduled Job 1 (pass=100, stride=100)  # Minimum pass = 100
  Job 1: pass=100 → 200

--- Round 5 ---
Scheduled Job 2 (pass=200, stride=200)  # Minimum pass = 200
  Job 2: pass=200 → 400
```

### Pass Value Evolution
| Round | Job 1 (stride=100) | Job 2 (stride=200) | Job 3 (stride=400) | Selected |
|-------|-------------------|-------------------|-------------------|----------|
| 1     | 0 → 100          | 0                 | 0                 | Job 1    |
| 2     | 100              | 0 → 200           | 0                 | Job 2    |
| 3     | 100              | 200               | 0 → 400           | Job 3    |
| 4     | 100 → 200        | 200               | 400               | Job 1    |
| 5     | 200              | 200 → 400         | 400               | Job 2    |

## Proportional Fairness Analysis

### Resource Allocation
With tickets [100, 50, 25]:
- **Job 1**: 100/175 = 57.1% CPU time
- **Job 2**: 50/175 = 28.6% CPU time  
- **Job 3**: 25/175 = 14.3% CPU time

### Stride Relationship
```
stride₁ : stride₂ : stride₃ = 100 : 200 : 400 = 1 : 2 : 4
```
**Execution frequency is inversely proportional to stride values**

## Implementation Details

### Priority Queue (`main.rs:29-41`)
```rust
impl Ord for Job {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Min-heap: smallest pass value has highest priority
        other.pass.cmp(&self.pass)
    }
}
```

Uses `BinaryHeap` with reverse ordering to implement min-heap based on pass values.

### Large Number Selection (`main.rs:53`)
```rust
large_number: 10000, // Large constant for stride calculation
```
- **Precision**: Larger values provide finer granularity
- **Overflow**: Must fit in data type (u32 max = 4,294,967,295)
- **Typical Range**: 10,000 to 1,000,000 depending on ticket distribution

## Testing and Verification

### Unit Tests (`main.rs:129-172`)

#### Stride Calculation Test
```rust
#[test]
fn test_stride_calculation() {
    let job1 = Job::new(1, 100, 10000);
    assert_eq!(job1.stride, 100); // 10000 / 100
}
```

#### Proportional Fairness Test
```rust
#[test] 
fn test_proportional_scheduling() {
    // Verify job1 runs twice as often as job2 over 200 rounds
    let ratio = job1_count as f64 / job2_count as f64;
    assert!((ratio - 2.0).abs() < 0.1);
}
```

## Performance Characteristics

### Time Complexity
- **Scheduling Decision**: O(log n) - heap operations
- **Job Addition**: O(log n) - heap insertion
- **Queue Display**: O(n log n) - sorting for visualization

### Space Complexity
- **Memory Usage**: O(n) - one job structure per process
- **Queue Storage**: Binary heap with n elements

### Fairness Guarantees
- **Maximum Unfairness**: At most one stride unit between any two jobs
- **Convergence**: Pass values automatically balance over time
- **Starvation**: Impossible - every job with tickets will run

## Building and Running

```bash
cargo build       # Compile the project
cargo run         # Execute demonstration
cargo test        # Run unit tests
```

### Expected Output
```
Adding Job 1: tickets=100, stride=100
Adding Job 2: tickets=50, stride=200
Adding Job 3: tickets=25, stride=400

Running stride scheduling for 10 rounds:

--- Round 1 ---
Scheduled Job 1 (pass=0, stride=100)
  Updated Job 1 pass to 100
Job 1 runs for quantum
Current queue state:
  Job 2: pass=0, stride=200
  Job 3: pass=0, stride=400
  Job 1: pass=100, stride=100
```

## Comparison with Other Schedulers

| Scheduler | Fairness | Predictability | Complexity | Overhead |
|-----------|----------|----------------|------------|----------|
| **FIFO** | Poor | High | O(1) | Minimal |
| **Round Robin** | Good | Medium | O(1) | Low |
| **Lottery** | Good (avg) | Low | O(n) | Medium |
| **Stride** | Perfect | High | O(log n) | Medium |

## Real-World Applications

### Operating Systems
- **Linux CFS**: Uses similar virtual time concepts
- **Xen Hypervisor**: Credit scheduler based on stride scheduling
- **Research Systems**: Many experimental schedulers use stride principles

### Resource Management
- **Network QoS**: Bandwidth allocation with stride scheduling
- **Cloud Computing**: CPU allocation in containerized environments
- **Game Engines**: Fair time allocation between subsystems