#include <iostream>

#include <gmp.h>

#include "primes.h"

#define expect(a, b, s) if (a != b) std::cout << s << "\n" << "Expected " << a << ", received " << b << std::endl;

void test_primality(unsigned long int n) {
  mpz_t input;
  mpz_init(input);
  mpz_set_ui(input, n);
  if (is_prime_fermat(input))
    std::cout << n << " is prime." << std::endl;
  else
    std::cout << n << " is composite." << std::endl;
}

void test_sieve() {
  uint32_t *prime_test_table;
  uint32_t prime_test_size;
  uint32_t prime_sieve_max = 900000000;
  sieve(&prime_test_table, &prime_test_size, prime_sieve_max);
  uint32_t exp_size = 46009215u;
  expect(exp_size, prime_test_size, "Prime sieve doesn't find correct number of primes");
  free(prime_test_table);
}

void test_is_valid_pow() {
  mpz_t input;
  mpz_init(input);
  
  mpz_set_ui(input, 97);
  expect(true, is_valid_pow(input), "POW checker gives false negative on 97");
  
  mpz_set_ui(input, 103);
  expect(false, is_valid_pow(input), "POW checker gives false positive on 103");

  mpz_set_ui(input, 14812867);
  expect(true, is_valid_pow(input), "POW checker gives false negative on 14812867");
}


int main(int argc, char** argv) {
  test_sieve();
  test_is_valid_pow();
}
