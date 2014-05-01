#include <iostream>
#include <stdint.h>
#include <string.h>

#include <gmp.h>

#include "primes.h"
#include "util.h"


bool is_even(mpz_t n) {
  return mpz_divisible_ui_p(n, 2) != 0;
}

bool is_prime_fermat(mpz_t n, int k) {
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
    mpz_powm(rem, a, nmo, n);
    if (mpz_cmp(rem, one) != 0)
      return false;
  }
  return true;
}

bool is_valid_pow(mpz_t candidate) {

  unsigned int offsets [5] = {4, 2, 4, 2, 4};
  
  mpz_t offset;
  mpz_init(offset);
  for (int i = 0; i < 5; i++) {
    mpz_set_ui(offset, offsets[i]);
    
    mpz_add(candidate, candidate, offset);
    
    if (!is_prime_fermat(candidate)) return false;
  }
  
  return true;
}

void sieve(uint32_t** prime_test_table, uint32_t* prime_test_size,
    uint32_t prime_test_limit) {
  /* Source: http://git.io/-m1Ypw
   * We use uint8 instead of bool because otherwise our array would be too
   * large to index into. */
  *prime_test_table = (uint32_t*)malloc(sizeof(uint32_t)*(prime_test_limit/4+10));
  if (prime_test_table == NULL) {
    perror("could not allocate prime test table");
    exit(-1);
  }
  *prime_test_size = 0;

  uint8_t* vfComposite = (uint8_t*)malloc(sizeof(uint8_t)*(prime_test_limit+7)/8);
  if (vfComposite == NULL) {
    perror("could not allocate vfComposite table");
    exit(-1);
  }
  memset(vfComposite, 0x00, sizeof(uint8_t)*(prime_test_limit+7)/8);
  for (unsigned int nFactor = 2; nFactor * nFactor < prime_test_limit; nFactor++)
  {
    if(vfComposite[nFactor>>3] & (1<<(nFactor&7)))
      continue;
    for (unsigned int nComposite = nFactor * nFactor; nComposite < prime_test_limit; nComposite += nFactor)
      vfComposite[nComposite>>3] |= 1<<(nComposite&7);
  }
  for (unsigned int n = 2; n < prime_test_limit; n++)
  {
    if ((vfComposite[n>>3] & (1<<(n&7))) == 0)
    {
      (*prime_test_table)[*prime_test_size] = n;
      (*prime_test_size)++;
    }
  }
  *prime_test_table = (uint32_t*)realloc(*prime_test_table, sizeof(uint32_t)*(*prime_test_size));
  free(vfComposite);
}

