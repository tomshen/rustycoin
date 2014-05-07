#include <iostream>
#include <stdint.h>

#include <thrust/host_vector.h>

#include "big_integer.h"
#include "primes.h"

#define expect(a, b, s) if (a != b) { std::cerr << "---\n" << s << "\n" << "Expected " << a << ", received " << b << "\n---\n" << std::endl; }

int main(int argc, char** argv) {
  bigi max_val = 100000000;
  uint32_t max_sieve = 29;//500000000;
  thrust::host_vector<bigi> clusters = generate_prime_clusters(max_val, max_sieve, true);
  expect(80, clusters.size(), "Didn't find right number of clusters.");
}
