# Questions

1. Run a few randomly-generated problems with just two jobs and
two queues; compute the MLFQ execution trace for each. Make
your life easier by limiting the length of each job and turning off
I/Os.

2. How would you run the scheduler to reproduce each of the exam-
ples in the chapter?

3. How would you configure the scheduler parameters to behave just
like a round-robin scheduler?

4. Craft a workload with two jobs and scheduler parameters so that
one job takes advantage of the older Rules 4a and 4b (turned on
with the -S flag) to game the scheduler and obtain 99% of the CPU
over a particular time interval.

5. Given a system with a quantum length of 10 ms in its highest queue,
how often would you have to boost jobs back to the highest priority
level (with the -B flag) in order to guarantee that a single long-
running (and potentially-starving) job gets at least 5% of the CPU?

6. One question that arises in scheduling is which end of a queue to
add a job that just finished I/O; the -I flag changes this behavior
for this scheduling simulator. Play around with some workloads
and see if you can see the effect of this flag.