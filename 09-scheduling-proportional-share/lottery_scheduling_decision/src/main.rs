use rand::Rng;

#[derive(Debug, Clone)]
struct Job {
    id: u32,
    tickets: u32,
    next: Option<Box<Job>>,
}

impl Job {
    fn new(id: u32, tickets: u32) -> Self {
        Job {
            id,
            tickets,
            next: None,
        }
    }
}

struct LotteryScheduler {
    head: Option<Box<Job>>,
    total_tickets: u32,
}

impl LotteryScheduler {
    fn new() -> Self {
        LotteryScheduler {
            head: None,
            total_tickets: 0,
        }
    }

    fn add_job(&mut self, id: u32, tickets: u32) {
        let mut new_job = Box::new(Job::new(id, tickets));
        new_job.next = self.head.take();
        self.head = Some(new_job);
        self.total_tickets += tickets;
    }

    // Lottery scheduling decision code
    fn schedule(&self) -> Option<u32> {
        if self.total_tickets == 0 {
            return None;
        }

        // counter: used to track if we have found the winner yet
        let mut counter = 0;

        // winner: call some random number generator to
        // get a value >= 0 and <= (totaltickets - 1)
        let mut rng = rand::thread_rng();
        let winner = rng.gen_range(0..self.total_tickets);

        // current: use this to walk through the list of jobs
        let mut current = &self.head;
        while let Some(job) = current {
            counter += job.tickets;
            if counter > winner {
                // found winner: return it
                return Some(job.id);
            }

            current = &job.next;
        }

        // should never reach here if total_tickets is correct
        None
    }
}

fn main() {
    let mut scheduler = LotteryScheduler::new();

    // add jobs with different ticket allocations
    scheduler.add_job(1, 100); //job 1: 100 tickets
    scheduler.add_job(2, 50); //job 2: 50 tickets
    scheduler.add_job(3, 25); //job 3: 25 tickets

    println!("Total tickets: {}", scheduler.total_tickets);
    println!("Running lottery scheduling 10 times:");

    // run lottery scheduling multiple times
    for i in 0..10 {
        if let Some(winner_id) = scheduler.schedule() {
            println!("Round {}: Job {} wins", i + 1, winner_id);
        }
    }
}
