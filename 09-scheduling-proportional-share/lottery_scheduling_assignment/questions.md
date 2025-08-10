# Questions

1. Compute the solutions for simulations with 3 jobs and random seeds
of 1, 2, and 3.

2. Now run with two specific jobs: each of length 10, but one (job 0)
with 1 ticket and the other (job 1) with 100 (e.g., -l 10:1,10:100).
What happens when the number of tickets is so imbalanced? Will
job 0 ever run before job 1 completes? How often? In general, what
does such a ticket imbalance do to the behavior of lottery schedul-
ing?

3. When running with two jobs of length 100 and equal ticket alloca-
tions of 100 (-l 100:100,100:100), how unfair is the scheduler?
Run with some different random seeds to determine the (probabilis-
tic) answer; let unfairness be determined by how much earlier one
job finishes than the other.

4. How does your answer to the previous question change as the quan-
tum size (-q) gets larger?

5. Can you make a version of the graph that is found in the chapter?
What else would be worth exploring? How would the graph look
with a stride scheduler?