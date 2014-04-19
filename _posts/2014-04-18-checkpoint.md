---
layout: post
title:  "15-418 S14 Final Project - Project Checkpoint"
image: false
video: false
comments: false
---

# Current Progress
We have completed a sequential Riecoin proof of work algorithm in Rust. The two main components we have implemented are the Rabin-Miller primality test and wheel factorization. These are the algorithms used by David Andersen in his [CPU miner](https://github.com/dave-andersen/fastrie). We researched other methods of prime verification and generation, but it seems that these are the most efficient for the large integers we are dealing with.
We began by implementing these algorithms for bignums. However, we realized that a lot of the functionality we need for the miner is not yet implemented in the built-in Rust bignum library. For now, our algorithm assumes that we are dealing with unsigned 64 bit integers. In the next week, we plan to add the needed functionality to the bignum library, so that we can handle the large integers needed for the miner. 
Also, we decided to use an external [Rust bignum library](https://github.com/jsanders/rust-bignum), after poor performance on Rabin-Miller with the built-in library. This improved performance on Rabin-Miller substantially. We will be modifying this library, rather than the built-in bignum library. 
We have also written some unit tests for Rabin-Miller and wheel factorization. We will be using the built-in Rust unit testing. 

# Challenges
* Modifying the bignum library to include what we need for the proof of work algorithm
* Doing arithmetic on very large integers 
* Optimizing our proof of work algorithm to beat the CPU implementation by reducing shared information

# Revised Goals
We are definitely on schedule to complete a proof of work algorithm that uses the GPU and improves upon CPU performance. Now that we have written the sequential algorithm, adapting it to use OpenCL should be easily completed in the next couple weeks. We are also confident that we will be able to optimize our algorithm further by parallelizing additional components of the algorithm. Given our revised schedule below, we should have time to implement a complete Riecoin miner. The only thing that may prevent us from achieving these goals is adding to the bignum library, which may be more difficult than expected.

##Plan to achieve
* Add functionality to bignum library
* Use GPU for parallel prime cluster searching
* Achieve better performance than CPU miner on a GHC machine

##Hope to achieve
* Parallelize rest of computation (fast prime verification)
* Implement complete Riecoin miner

# Remaining Schedule
| Dates             | What We Plan to Do                                                                           |
|-------------------|----------------------------------------------------------------------------------------------|
| Apr. 17 - Apr. 19 | Complete sequential miner, project checkpoint                                                |
| Apr. 20 -  Apr. 23| Make cluster work with bignums, optimize sequential cluster searching                        |
| Apr. 24 - Apr. 26 | Add parallelization by moving prime cluster searching computation to GPU                     |
| Apr. 27 - Apr. 30 | Parallelize prime verification, improve algorithm by looking for further parallelism         |
| May 1 - May 3     | Implement complete miner                                                                     |
| May 4 - May 7     | Performance testing, final optimizations                                                     |
| May 8 - May 9     | Bug fixing, final write up                                                                   |

# Demo
For our final demo, we plan to run our miner across many of the GHC machines. We also plan to show graphs showing the speedup our miner achieved over the CPU miner. 