# Lottery Scheduling Decision Implementation

## What

This is a Rust implementation of the lottery scheduling algorithm decision code from Figure 9.1 in the Operating Systems textbook. It demonstrates proportional-share scheduling where processes are allocated "tickets" and the scheduler randomly selects a winning ticket to determine which process runs next.

## Why

Lottery scheduling solves the problem of proportional fairness in CPU scheduling:
- **Fair Share**: Each process gets CPU time proportional to its ticket allocation
- **Randomized**: Avoids deterministic patterns that could be gamed
- **Simple**: Elegant probabilistic approach to resource allocation
- **Flexible**: Easy to adjust process priorities by changing ticket counts

## How It Works

### Core Algorithm (`main.rs:41-68`)
```rust
fn schedule(&self) -> Option<u32> {
    let mut counter = 0;
    let winner = rng.gen_range(0..self.total_tickets);
    
    let mut current = &self.head;
    while let Some(job) = current {
        counter += job.tickets;
        if counter > winner {
            return Some(job.id);
        }
        current = &job.next;
    }
    None
}
```

### Step-by-Step Process
1. **Generate Random Winner**: Pick random number from `[0, total_tickets)`
2. **Walk Job List**: Traverse linked list of jobs
3. **Accumulate Tickets**: Add each job's tickets to running counter
4. **Find Winner**: When counter exceeds random number, that job wins
5. **Schedule Job**: Return winning job ID for execution

### Data Structures

#### Job Structure (`main.rs:3-18`)
```rust
struct Job {
    id: u32,           // Unique job identifier
    tickets: u32,      // Number of lottery tickets
    next: Option<Box<Job>>, // Linked list pointer
}
```

#### Scheduler Structure (`main.rs:20-31`)
```rust
struct LotteryScheduler {
    head: Option<Box<Job>>,  // Head of job linked list
    total_tickets: u32,      // Sum of all tickets
}
```

## Example Execution

### Setup
```rust
scheduler.add_job(1, 100); // Job 1: 100 tickets (57.1% probability)
scheduler.add_job(2, 50);  // Job 2: 50 tickets  (28.6% probability)  
scheduler.add_job(3, 25);  // Job 3: 25 tickets  (14.3% probability)
```

### Sample Output
```
Total tickets: 175
Running lottery scheduling 10 times:
Round 1: Job 1 wins
Round 2: Job 2 wins
Round 3: Job 1 wins
Round 4: Job 1 wins
Round 5: Job 3 wins
Round 6: Job 1 wins
Round 7: Job 1 wins
Round 8: Job 2 wins
Round 9: Job 1 wins
Round 10: Job 3 wins
```

### Probability Analysis
- **Job 1**: 100/175 = 57.1% chance per scheduling decision
- **Job 2**: 50/175 = 28.6% chance per scheduling decision
- **Job 3**: 25/175 = 14.3% chance per scheduling decision

## Key Features

### Proportional Fairness
Jobs receive CPU time proportional to their ticket allocation over time, ensuring fair resource distribution.

### Memory Efficient
Uses a simple linked list structure with minimal overhead per job.

### Randomized Selection
Prevents gaming and provides statistical fairness guarantees.

### Easy Priority Adjustment
Changing ticket counts immediately affects scheduling probability without complex recalculation.

## Algorithm Complexity

- **Time**: O(n) per scheduling decision (linear scan through job list)
- **Space**: O(n) for job storage (one node per job)
- **Randomness**: O(1) random number generation

## Building and Running

```bash
cargo build    # Compile the project
cargo run      # Execute the demo
```

## Design Considerations

### Advantages
✅ **Simple Implementation**: Easy to understand and implement
✅ **Proportional Fairness**: Guarantees long-term fair share
✅ **Dynamic Priority**: Can adjust tickets at runtime
✅ **No Starvation**: Every job with tickets will eventually run

### Limitations  
⚠️ **Linear Search**: O(n) time complexity per decision
⚠️ **Short-term Variance**: Random selection can cause temporary unfairness
⚠️ **Ticket Management**: Requires careful ticket allocation strategy

## Extensions and Variations

### Possible Improvements
1. **Stride Scheduling**: Deterministic version with better short-term fairness
2. **Hierarchical Tickets**: Support for nested ticket allocations
3. **Ticket Inheritance**: Handle ticket passing between related processes
4. **Load Balancing**: Distribute tickets across multiple CPU cores

### Configuration Options
```rust
// High priority job
scheduler.add_job(1, 1000);

// Medium priority jobs  
scheduler.add_job(2, 100);
scheduler.add_job(3, 100);

// Background job
scheduler.add_job(4, 10);
```

## Real-World Applications

### Use Cases
- **CPU Scheduling**: Primary use in operating systems
- **Resource Allocation**: Network bandwidth, memory allocation
- **Load Balancing**: Distributing requests across servers
- **Game Mechanics**: Random selection with weighted probabilities

### Production Considerations
- **Ticket Conservation**: Ensure tickets aren't lost or duplicated
- **Scalability**: Consider more efficient data structures for large job counts
- **Fairness Metrics**: Monitor actual vs expected resource allocation
- **Security**: Prevent ticket manipulation attacks

## Educational Value

This implementation demonstrates:
- **Probabilistic Algorithms**: How randomness can solve scheduling problems
- **Data Structure Design**: Linked list traversal and management
- **Resource Management**: Fair allocation strategies
- **System Design**: Trade-offs between simplicity and performance

Perfect for understanding proportional-share scheduling concepts and the elegant simplicity of lottery-based resource allocation.