---
layout: post
title:  "Initial Proposal"
date:   2014-04-02 20:45:54
image: false
video: false
comments: false
---

# Summary
The goal of this project is to create the first open source Riecoin GPU miner. We plan on first implementing a simple serial CPU version, based on David Andersen's [CPU miner](https://github.com/dave-andersen/fastrie). Then, we plan on parallelizing on a single machine using [OpenCL](https://github.com/luqmana/rust-opencl). Finally, if we have time, we plan on parallelizing across multiple machines.

# Background
[Riecoin](http://riecoin.org) is a decentralized digital currency, similar to Bitcoin. It differs from Bitcoin in that its proof of work algorithm actually does something useful, rather than just check for sha256 collisions. In particular, the proof of work is based on finding a prime "constellation": a sextuplet of "consecutive" prime numbers (a sequence of primes of the form *n, n+4, n+6, n+10, n+12, n+16*). This prime constellation must be within a certain given range.

Currently, there exists a parallel CPU miner for Riecoin, as we mentioned in the summary. However, it is a very naive implementation due to not sharing data between threads. Additionally, GPU miners for digital currencies tend to perform much better due to how parallelizable most proofs of work are. However, an open source GPU miner does not currently exist for Riecoin.

# Challenge
The search range for the prime constellations starts at very large numbers: when Riecoin first launched, the target (start) number was 304-bits long, and the difficulty has been increasing ever since then. Additionally, since a new block is available every 2.5 minutes (and thus a new target number and limit), any miner must be able to search for and verify primality of large primes very quickly. This is where parallelization comes in.

The current state-of-the-art miner uses [wheel factorization](http://en.wikipedia.org/wiki/Wheel_factorization) to find primes. Our primary challenge (after successfully implementing this algorithm sequentially) is to find places where we can improve on and parallelize this algorithm.

Our main parallelization challenges are:

* Find candidate primes with sieving in parallel
  * Using OpenCL rather than CUDA because of better support
  * Sharing information rather than repeating computation (which is what currently is done in the CPU miner)
* Check primality of candidate primes in parallel
  * Finding and implementing a primality test that will benefit from parallelization
* Parallelizing arbitrary-precision integer arithmetic
  * Difficult to correctly implement in parallel
  * Only if we have extra time

# Resources
* [Rust](http://www.rust-lang.org/) - safe, concurrent systems programming language
* [OpenCL](https://www.khronos.org/opencl/) - GPU computation framework
  * [rust-opencl](https://github.com/luqmana/rust-opencl) - OpenCL bindings for Rust
* [fastrie](https://github.com/dave-andersen/fastrie) - Riecoin CPU minter
* David Andersen - both consulting with him in person and his blog posts (e.g. [his post on fast prime cluster searching](http://da-data.blogspot.com/2014/03/fast-prime-cluster-search-or-building.html))
* GHC 3000 machines

# Goals & Deliverables
## Plan to achieve
* Implement a sequential Riecoin proof of work algorithm
* Use GPU for parallel prime cluster searching
* Achieve better performance than CPU miner on a GHC machine

## Hope to achieve
* Parallelize rest of computation (fast prime verification)
* Implement complete Riecoin miner
* Parallelize arbitrary-precision integer arithmetic

# Platform
* We plan to implement our miner in Rust, because it runs nearly as fast as C++ while also cleanly including many higher-level language features.
* We will use OpenCL for GPU computation because there exists bindings for it in Rust (as opposed to CUDA).

# Proposed Schedule
| Week              | What We Plan to Do                                                                           |
|-------------------|----------------------------------------------------------------------------------------------|
| Mar. 30 - Apr. 5  | Choose project idea, write initial proposal                                                  |
| Apr. 6 - Apr. 12  | Implement sequential, CPU-based proof of work algorithm                                      |
| Apr. 13 - Apr. 19 | Add parallelization by moving prime cluster searching computation to GPU, project checkpoint |
| Apr. 20 - Apr. 26 | Improve algorithm and look for more areas of parallelization, parallelize prime verification |
| Apr. 27 - May 3   | Implement complete miner                                                                     |
| May 4 - May 9     | Performance testing, bug fixing, final write up                                              |
