#include <iostream>
#include <stdint.h>
#include <string.h>
#include <vector>

#include <gmp.h>

#include "primes.h"
#include "util.h"

static uint32_t cluster_offsets[6] = {0, 4, 2, 4, 2, 4};

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
    for (unsigned int nComposite = nFactor * nFactor;
        nComposite < prime_test_limit; nComposite += nFactor)
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
  *prime_test_table = (uint32_t*)realloc(*prime_test_table,
      sizeof(uint32_t)*(*prime_test_size));
  free(vfComposite);
}

bool candidate_killed_by(mpz_t candidate, mpz_t prime) {
  mpz_t p, modp;
  mpz_init_set(p, candidate);
  for(int i = 0; i < 6; i++) {
    mpz_add_ui(p, p, cluster_offsets[i]);
    mpz_mod(modp, p, prime);
    if (mpz_cmp_ui(modp, 0) == 0)
      return true;
  }
  return false;
}


std::vector<mpz_t> add_next_prime(std::vector<mpz_t> offsets, mpz_t max_val,
    mpz_t prime, mpz_t primorial) {
  mpz_t base, counter, val;
  mpz_inits(base, counter, NULL);
  mpz_set_ui(base, 0);
  mpz_set_ui(counter, 0);

  std::vector<mpz_t> new_offsets;

  while (mpz_cmp(counter, prime) < 0) {
    if (mpz_cmp(base, max_val) > 0)
      break;
    for (std::vector<mpz_t>::iterator o = offsets.begin(); o != offsets.end();
        o++) {
      mpz_init(val);
      mpz_add(val, base, *o);
      if (mpz_cmp(val, max_val) > 0)
        break;
      if (!candidate_killed_by(val, prime))
        new_offsets.push_back(val);
    }
    mpz_add(base, base, primorial);
    mpz_add_ui(counter, counter, 1u);
  }

  return new_offsets;
}

uint32_t generate_prime_clusters(mpz_t max_val, uint32_t max_sieve, bool verbose) {
  uint32_t primorial_start = 7;

  mpz_t primorial, big97, prime;
  mpz_inits(primorial, big97, prime, NULL);
  mpz_set_ui(primorial, 210);
  mpz_set_ui(big97, 97);

  std::vector<mpz_t> offsets;
  offsets.push_back(big97);

  uint32_t *prime_test_table;
  uint32_t prime_test_size;
  sieve(&prime_test_table, &prime_test_size, max_sieve);

  for (uint32_t i = primorial_start+1; i < prime_test_size; i++) {
    mpz_set_ui(prime, i);
    offsets = add_next_prime(offsets, max_val, prime, primorial);
    mpz_mul_ui(primorial, primorial, i);
  }
  uint32_t count = 0;
  for (std::vector<mpz_t>::iterator o = offsets.begin(); o != offsets.end();
      o++) {
    if (is_valid_pow(*o)) {
      count++;
      if (verbose)
        std::cout << "Found valid POW " << *o << std::endl;
    }
  }
  return count;
}
