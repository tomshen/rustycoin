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

A new proof of work is issued every 2.5 minutes, so one important constraint is that our algorithm needs to be able to find a proof of work within that time. Since we're also competing with other miners to calculate the proof of work, the actual time our algorithm takes should be much lower than that.

A typical algorithm will sieve a number of smaller primes using the [Sieve of Eratosthenes](http://en.wikipedia.org/wiki/Sieve_of_Eratosthenes), then use [wheel factorization](http://en.wikipedia.org/wiki/Wheel_factorization) to find likely candidates for proofs of work, and finally check if candidates are valid using the [Rabin-Miller primality test](http://en.wikipedia.org/wiki/Miller%E2%80%93Rabin_primality_test). A more thorough explanation can be found in CMU professor Dave Andersen's [blog post](http://da-data.blogspot.com/2014/03/fast-prime-cluster-search-or-building.html).

Wheel factorization can be used to generate candidates because we can show that all proofs of work must fit a certain pattern (though the reverse is not necessarily true). For instance, sieving with small primes can be used to show that all valid proofs of work must be of the form 210*n* + 97 (note that 210 is a primorial, the product of primes 2,3,5,7).

While the initial sieving step is inherently sequential, wheel factorization can be parallelized, since it involves generation of candidates that fit several different patterns. Candidate generation for each pattern is independent, allowing that process to be parallelized. Additionally, the process of verifying proofs of work can trivially be parallelized, since we can check each candidate independently.

Another opportunity for parallelism is data parallelism: since we are looking for prime clusters within a certain range, we can split that range into several smaller ranges, and search each in parallel.

## Rust
We decided to work with experimental programming language Rust because it seemed to offer a better approach to systems programming than C++. It's build from the ground up to be memory safe, which solves one of the major issues with C/C++. Additionally, it was highly influenced by functional programming languages like SML, which means it has very nice features like algebraic data types (e.g. `Some(x) / None`), immutability by default, pattern matching, closures, higher-order-functions, and type inference. We expected Rust's functional flavor to improve programmer productivity and make reasoning about concurrency easier.

# Approach
We started by implementing a sequential version of the algorithm described above. We based our implementation heavily on the [C++ version used in Dave Andersen's miner](https://github.com/dave-andersen/fastrie/blob/master/xptMiner/xptMiner/riecoinMiner.cpp), and the simplified Python algorithm  explained in his blog post (linked above). Then, we began our parallel implementation.

In our initial proposal, we stated that we would try to parallelize the algorithm by moving computation to the GPU. Unfortunately, because Rust is a *pre-alpha* programming language, it lacks the libraries to do so. There exists an OpenCL library, [rust-opencl](https://github.com/luqmana/rust-opencl), but it was buggy and difficult to get working. Additionally, any device code had be written in C++, which defeated the purpose of writing the project in Rust.

Thus, we decided to parallelize with multiple CPU cores instead, using Rust's concurrency system. This system is based on the usage of lightweight *green* (not OS) threads (called *tasks*) and message passing. These tasks are scheduled and mapped to cores by Rust's internal scheduler, and data is send between tasks via channels.

We first tried parallelizing candidate verification. We spawned a new task for every candidate proof of work, and checked if it was a valid proof of work in parallel. This ended up slowing down our algorithm. We expected this to be because our units of work were too small (though in theory Rust's lightweight tasks shouldn't have this problem), so we tried verifying several candidates in each task. Unfortunately, since Rust does not support direct sharing of data between tasks, we had to clone portions of the candidates vector and send them to each task. This also caused our algorithm to become slower.

Then, we tried parallelizing wheel factorization. As we predicted above, we were able to parallelize candidate generation for each "pattern". However, once again, this caused our code to slow down. This was also due to the need to send a relatively large amount of data back and forth between tasks.

Overall, communication costs outweighed the benefits of parallelization within the algorithm.

However, data parallelization proved much more successful. We split the range we were searching in into several smaller ranges, and searched for primes in each range in separate tasks. This led to fairly good speedup, as we will detail below.

We also added support for early termination. Since finding a proof of work only requires finding one valid candidate in the search range, once we find a valid candidate in one task, we can stop searching in other threads. This proved difficult to implement in Rust, because it discourages use of shared state, and even more strongly use of shared mutable state. However, because we simply needed to share a single boolean flag, we were able to implement early termination with the use of a lot of "unsafe" blocks of code.

The target machine for our implementation was the unix2.andrew.cmu.edu server, because our work is highly compute-bound, and our method of parallelization benefits highly from many cores (in this case, 24).

## Bignums
Since our search range starts with a 300-bit number, we needed to use arbitrary-precision integers in all of our computations. Initially, we tried to use the bigint implementation in Rust's standard library, [BigUint](http://static.rust-lang.org/doc/master/num/bigint/struct.BigUint.html). However, that implementation was written fairly inefficiently in Rust, and thus was far too slow for our purposes. We instead chose to use a package we found on Github called [rust-bignum](https://github.com/jsanders/rust-bignum), which in theory was a drop-in replacement for Rust's standard bigint implementation. This implementation was a wrapper around [GMP](https://gmplib.org/), the most popular C library for abitrary precision numbers. However, this Rust library was buggy (we actually contributed a few bug fixes) and did not completely implement the standard library's interface, leading to many hours of confused debugging. Additionally, it still wasn't as fast as the actual GMP library, due to imperfect bindings.

# Results

We compared our parallelized proof of work algorithm to the sequential version we wrote in Rust on a 24-core Linux machine, the unix2.andrew.cmu.edu server. We considered a 64-bit base value and a 32-bit range (from 0xfedcba0900000000 to 0xfedcba09ffffffff). In the graphs below, we measured the time it took to find a prime sextuplet in the given range as we varied the number of tasks launched. 

![Graph of speedup](/media/graph1.jpg)

![Graph of speedup](/media/graph2.jpg)

As shown on the graphs, the speedup is almost linear for under 8 tasks. However, as we continue increasing the number of tasks, speedup drops off significantly. We see a decrease in speedup due to communication overhead, 

The Riecoin proof of work algorithm is designed to be resistant to parallelism. The size of the integers needed makes communicating results between tasks more difficult. Additionally, the complex computation involved in finding candidate primes makes dividing work among processors more difficult.

The need to handle bignums limited our ability to use the GPU. There does not exist a bignum library for Rust on the GPU. In fact, using the GPU for the proof of work algorithm in any language would be difficult, because there is not a fast, accessible library for bignums on GPU available.  

Ultimately, our algorithm is too slow to function as a Riecoin miner, because of the time constraint of 2.5 minutes per block. 

# References
* [fastrie](https://github.com/dave-andersen/fastrie) - David Andersen's Riecoin CPU miner
* [Rust bignum library](https://github.com/jsanders/rust-bignum)

# List of Work by Each Student
Equal work was performed by both project members.