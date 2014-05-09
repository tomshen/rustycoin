---
layout: post
title:  "Final Report"
date:   2014-05-09 14:12:00
image: false
video: false
comments: false
---

# Summary
In this project, we explored implementation of a [Riecoin](http://riecoin.org/) miner in the [Rust programming language](http://www.rust-lang.org/). We implemented a parallel prime cluster finding algorithm to find the proof of work for Riecoin. Our implementation achieves a 23-times speedup over the sequential algorithm on a 24-core Linux machine.

# Background
Riecoin is a cryptocurrency similar to Bitcoin. New units of currency (called Riecoins) can be produced through the process of mining. Every 2.5 minutes, a new block of currency is available to be mined. In order to mine the block and get the currency, the miner must be able to calculate a proof of work, which generally is a number meeting certain constraints that is difficult to calculate but easy to verify.

Riecoin's proof of work is to find a number *X* such that *basep* + *X* is the first prime of a prime cluster, where *basep* is a number that varies in size based on the current difficulty, and *X* must be in a certain range based on the current difficulty. The *difficulty* is adjusted based on how quickly miners were able to mine previous blocks, so that increases in computational power and/or improvements in mining algorithms don't cause too much inflation. Currently, *basep* is around 300 bits long.

A *prime cluster* is a sextuplet of primes of the form:
*p*, *p*+4, *p*+6, *p*+10, *p*+12, *p*+16.

A typical algorithm will sieve a number of smaller primes using the [Sieve of Eratosthenes](http://en.wikipedia.org/wiki/Sieve_of_Eratosthenes), then use [wheel factorization](http://en.wikipedia.org/wiki/Wheel_factorization) to find likely candidates for proofs of work, and finally check if candidates are valid using the [Rabin-Miller primality test](http://en.wikipedia.org/wiki/Miller%E2%80%93Rabin_primality_test). A more thorough explanation can be found in CMU professor Dave Andersen's [blog post](http://da-data.blogspot.com/2014/03/fast-prime-cluster-search-or-building.html).

Wheel factorization can be used to generate candidates because we can show that all proofs of work must fit a certain pattern (though the reverse is not necessarily true). For instance, sieving with small primes can be used to show that all valid proofs of work must be of the form 210*n* + 97 (note that 210 is a primorial, the product of primes 2,3,5,7).

While the initial sieving step is inherently sequential, wheel factorization can be parallelized, since it involves generation of candidates that fit several different patterns. Candidate generation for each pattern is independent, allowing that process to be parallelized. Additionally, the process of verifying proofs of work can trivially be parallelized, since we can check each candidate independently.

Another opportunity for parallelism is data parallelism: since we are looking for prime clusters within a certain range, we can split that range into several smaller ranges, and search each in parallel.

## Rust
We decided to work with experimental programming language Rust because it seemed to offer a better approach to systems programming than C++. It's build from the ground up to be memory safe, which solves one of the major issues with C/C++. Additionally, it was highly influenced by functional programming languages like SML, which means it has very nice features like algebraic data types (e.g. `Some(x) / None`), immutability by default, pattern matching, closures, higher-order-functions, and type inference. We expected Rust's functional flavor to improve programmer productivity and make reasoning about concurrency easier.

# Approach
Tell us how your implementation works. Your description should be sufficiently detailed to provide the course staff a basic understanding of your approach. Again, it might be very useful to include a figure here illustrating components of the system and/or their mapping to parallel hardware.

Describe the technologies used. What language/APIs? What machines did you target?
Describe how you mapped the problem to your target parallel machine(s). IMPORTANT: How do the data structures and operations you described in part 2 map to machine concepts like cores and threads. (or warps, thread blocks, gangs, etc.)
Did you change the original serial algorithm to enable better mapping to a parallel machine?
If your project involved many iterations of optimization, please describe this process as well. What did you try that did not work? How did you arrive at your solution? The notes you've been writing throughout your project should be helpful here. Convince us you worked hard to arrive at a good solution.
If you started with an existing piece of code, please mention it (and where it came from) here.

# Results
How successful were you at achieving your goals? We expect results sections to differ from project to project, but we expect your evaluation to be very thorough (your project evaluation is a great way to demonstrate you understood topics from this course). Here are a few ideas:

If your project was optimizing an algorithm, please define how you measured performance. Is it wall-clock time? Speedup? An application specific rate? (e.g., moves per second, images/sec)
Please also describe your experimental setup. What were the size of the inputs? How were requests generated?
Provide graphs of speedup or execute time. Please precisely define the configurations being compared. Is your baseline single-threaded CPU code? It is an optimized parallel implementation for a single CPU?
Recall the importance of problem size. Is it important to report results for different problem sizes for your project? Do different workloads exhibit different execution behavior?
IMPORTANT: What limited your speedup? Is it a lack of parallelism? (dependencies) Communication or synchronization overhead? Data transfer (memory-bound or bus transfer bound). Poor SIMD utilization due to divergence? As you try and answer these questions, we strongly prefer that you provide data and measurements to support your conclusions. If you are merely speculating, please state this explicitly. Performing a solid analysis of your implementation is a good way to pick up credit even if your optimization efforts did not yield the performance you were hoping for.
Deeper analysis: Can you break execution time of your algorithm into a number of distinct components. What percentage of time is spent in each region? Where is there room to improve?
Was your choice of machine target sound? (If you chose a GPU, would a CPU have been a better choice? Or vice versa.)

# References
Please provide a list of references used in the project.

# List of Work by Each Student
Equal work was performed by both project members.