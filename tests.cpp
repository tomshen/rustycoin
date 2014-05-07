#include <iostream>

#include <gmpxx.h>

#include "primes.h"

#define expect(a, b, s) if (a != b) std::cerr << "---\n" << s << "\n" << "Expected " << a << ", received " << b << "\n---\n" << std::endl;

void test_is_prime_fermat() {
  mpz_class n = 7;
  expect(true, is_prime_fermat(n), "7 should be prime");
  n = 97;
  expect(true, is_prime_fermat(n), "97 should be prime");
}

void test_sieve() {
  uint32_t *prime_test_table;
  uint32_t prime_test_size;
  uint32_t prime_sieve_max = 900000000;
  sieve(&prime_test_table, &prime_test_size, prime_sieve_max);
  uint32_t exp_size = 46009215u;
  expect(exp_size, prime_test_size,
      "Prime sieve doesn't find correct number of primes");
  free(prime_test_table);
}

void test_is_valid_pow() {
  mpz_class input = 7;
  expect(true, is_valid_pow(input), "POW checker gives false negative on 7");

  input = 103;
  expect(false, is_valid_pow(input), "POW checker gives false positive on 103");

  input = 14812867;
  expect(true, is_valid_pow(input),
      "POW checker gives false negative on 14812867");
}

int main(int argc, char** argv) {
  //test_is_prime_fermat();
  //test_is_valid_pow();
  mpz_class max_val = 100000000;
  uint32_t max_sieve = 29;//500000000;
  uint32_t count = generate_prime_clusters(max_val, max_sieve, true);
  expect(80, count, "Didn't find right number of clusters.");
}
