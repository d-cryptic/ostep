use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Job {
    id: u32,
    tickets: u32,
    pass: u32,
    stride: u32,
}

impl Job {
    fn new(id: u32, tickets: u32, large_number: u32) -> Self {
        let stride = if tickets > 0 {
            large_number / tickets
        } else {
            large_number
        };

        Job {
            id,
            tickets,
            pass: 0,
            stride,
        }
    }
}

// Implement ordering for priority queue (min-heap based on pass value)
impl PartialOrd for Job {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Job {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // compare by pass vaue (for min-heap, we want smallest pass first)
        other.pass.cmp(&self.pass)
    }
}

struct StrideScheduler {
    queue: BinaryHeap<Job>,
    large_number: u32,
    quantum: u32,
}

impl StrideScheduler {
    fn new(quantum: u32) -> Self {
        StrideScheduler {
            queue: BinaryHeap::new(),
            large_number: 10000, // large constant for stride calculation
            quantum,
        }
    }

    fn add_job(&mut self, id: u32, tickets: u32) {
        let job = Job::new(id, tickets, self.large_number);
        println!(
            "Adding Job {}: tickets={}, stride={}",
            id, tickets, job.stride
        );
        self.queue.push(job);
    }

    // stride scheduling decision code
    fn schedule(&mut self) -> Option<u32> {
        if self.queue.is_empty() {
            return None;
        }

        // curr = remove_min(queue) // pick client with min pass
        let mut curr = self.queue.pop()?;
        println!(
            "Scheduled Job {} (pass={}, stride={})",
            curr.id, curr.pass, curr.stride
        );

        // schedule(curr); //run for quantum
        // (In real system, this would run the job for quantum time)

        // curr->pass += curr->stride; // update pass using stride
        curr.pass += curr.stride;
        println!("  Updated Job {} pass to {}", curr.id, curr.pass);

        // insert(queue, curr); //return curr to queue
        self.queue.push(curr.clone());

        Some(curr.id)
    }

    fn show_queue(&self) {
        println!("Current queue state:");
        let mut jobs: Vec<_> = self.queue.iter().collect();
        jobs.sort_by_key(|job| job.pass);
        for job in jobs {
            println!("  Job {}: pass={}, stride={}", job.id, job.pass, job.stride);
        }
    }
}

fn main() {
    let mut scheduler = StrideScheduler::new(100); //100ms quantum

    // add jobs with different ticket allocations
    scheduler.add_job(1, 100); // Job 1: 100 tickets, stride = 100
    scheduler.add_job(2, 50); // Job 2: 50 tickets, stride = 200
    scheduler.add_job(3, 25); // Job 3: 25 tickets, stride = 400

    println!("\nInitial state:");
    scheduler.show_queue();

    println!("\nRunning stride scheduling for 10 rounds:");
    for round in 1..=10 {
        println!("\n--- Round {} ---", round);
        if let Some(job_id) = scheduler.schedule() {
            println!("Job {} runs for quantum", job_id);
        }

        scheduler.show_queue();
    }

    // show final pass values
    println!("\nFinal pass values after 10 rounds:");
    scheduler.show_queue();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stride_calculation() {
        let job1 = Job::new(1, 100, 10000);
        let job2 = Job::new(2, 50, 10000);
        let job3 = Job::new(3, 25, 10000);

        assert_eq!(job1.stride, 100); //10000 / 100
        assert_eq!(job2.stride, 200); // 10000 / 50
        assert_eq!(job3.stride, 400); // 10000 / 25
    }

    #[test]
    fn test_proportional_scheduling() {
        let mut scheduler = StrideScheduler::new(100);
        scheduler.add_job(1, 100); // Should run twice as often as job 2
        scheduler.add_job(2, 50);

        let mut job1_count = 0;
        let mut job2_count = 0;

        // run many scheduling decisions
        for _ in 0..200 {
            if let Some(job_id) = scheduler.schedule() {
                match job_id {
                    1 => job1_count += 1,
                    2 => job2_count += 1,
                    _ => {}
                }
            }
        }

        // Job 1 should run approximately twice as often as job 2
        let ratio = job1_count as f64 / job2_count as f64;
        assert!(
            (ratio - 2.0).abs() < 0.1,
            "Ratio should be close to 2.0, got {}",
            ratio
        );
    }
}
