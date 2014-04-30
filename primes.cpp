#include <iostream>
#include <string>

#include <gmp.h>

#define FERMAT_ITERS 1000

bool is_even(mpz_t n) {
  return mpz_divisible_ui_p(n, 2) != 0;
}

bool is_prime_fermat(mpz_t n, int k=FERMAT_ITERS) {
  mpz_t one, two, rem, nmo, a;
  gmp_randstate_t seed;

  mpz_inits(one, two, rem, nmo, a, NULL);
  gmp_randinit_default(seed);

  mpz_set_ui(one, 1);
  mpz_set_ui(two, 2);

  if (mpz_cmp(n, two) == 0) {
    return true;
  } else if (is_even(n)) {
    return false;
  }

  mpz_sub(nmo, n, one);
  for (int i = 0; i < k; i++) {
    mpz_urandomm(a, seed, n);
    mpz_powm_sec(rem, a, nmo, n);
    if (mpz_cmp(rem, one) != 0)
      return false;
  }
  return true;
}

int main(int argc, char** argv) {
  mpz_t input;
  mpz_init(input);
  if (argc < 2) {
    std::cout << "Usage: primes [integer to test for primality]" << std::endl;
    return 0;
  } else {
    mpz_set_str(input, argv[1], 10);
    if (is_prime_fermat(input))
      std::cout << argv[1] << " is prime." << std::endl;
    else
      std::cout << argv[1] << " is composite." << std::endl;
    return 0;
  }
}
