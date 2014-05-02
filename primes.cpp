#include <iostream>
#include <stdint.h>
#include <string.h>
#include <vector>

#include <cuda.h>
#include <cuda_runtime.h>
#include <driver_functions.h>
#include <gmpxx.h>

#include "primes.h"
#include "util.h"

static uint32_t cluster_offsets[6] = {0, 4, 2, 4, 2, 4};

bool is_even(mpz_class n) {
  return mpz_divisible_2exp_p(n.get_mpz_t(), 1) != 0;
}

bool is_prime_fermat(mpz_class n, int k) {
  mpz_class rem, a;

  gmp_randstate_t seed;
  gmp_randinit_default(seed);


  if (n == 2) {
    return true;
  } else if (is_even(n)) {
    return false;
  }

  for (int i = 0; i < k; i++) {
    mpz_class nmo = n - 1;
    mpz_urandomm(a.get_mpz_t(), seed, nmo.get_mpz_t()); // 0 <= a < n-1
    a += 1; // 1 <= a < n
    mpz_powm(rem.get_mpz_t(), a.get_mpz_t(), nmo.get_mpz_t(), n.get_mpz_t());
    if (rem != 1)
      return false;
  }

  return true;
}

bool is_valid_pow(mpz_class candidate) {
  mpz_class val = candidate;
  for (int i = 0; i < 6; i++) {
    val += cluster_offsets[i];
    if (!is_prime_fermat(val))
      return false;
  }
  return true;
}

void sieve(uint32_t** prime_test_table, uint32_t* prime_test_size,
    uint32_t prime_test_limit) {
  /* Source: http://git.io/-m1Ypw
   * We use uint8 instead of bool because otherwise our array would be too
   * large to index into. */
  *prime_test_table = (uint32_t*)malloc(sizeof(uint32_t)
      *(prime_test_limit/4+10));
  if (prime_test_table == NULL) {
    perror("could not allocate prime test table");
    exit(-1);
  }
  *prime_test_size = 0;

  uint8_t* vfComposite = (uint8_t*)malloc(sizeof(uint8_t)
      *(prime_test_limit+7)/8);
  if (vfComposite == NULL) {
    perror("could not allocate vfComposite table");
    exit(-1);
  }
  memset(vfComposite, 0x00, sizeof(uint8_t)*(prime_test_limit+7)/8);
  for (unsigned int nFactor = 2; nFactor * nFactor < prime_test_limit;
      nFactor++) {
    if(vfComposite[nFactor>>3] & (1<<(nFactor&7)))
      continue;
    for (unsigned int nComposite = nFactor * nFactor;
        nComposite < prime_test_limit; nComposite += nFactor)
      vfComposite[nComposite>>3] |= 1<<(nComposite&7);
  }
  for (unsigned int n = 2; n < prime_test_limit; n++) {
    if ((vfComposite[n>>3] & (1<<(n&7))) == 0) {
      (*prime_test_table)[*prime_test_size] = n;
      (*prime_test_size)++;
    }
  }
  *prime_test_table = (uint32_t*)realloc(*prime_test_table,
    sizeof(uint32_t)*(*prime_test_size));
  free(vfComposite);
}

bool candidate_killed_by(mpz_class candidate, mpz_class prime) {
  mpz_class p = candidate;
  for(int i = 0; i < 6; i++) {
    p += cluster_offsets[i];
    if (p % prime == 0)
      return true;
  }
  return false;
}


std::vector<mpz_class> add_next_prime(std::vector<mpz_class> offsets,
    mpz_class max_val, mpz_class prime, mpz_class primorial) {
  mpz_class base = 0;
  mpz_class counter = 0;

  std::vector<mpz_class> new_offsets;

  while (counter < prime) {
    if (base > max_val)
      break;
    for (std::vector<mpz_class>::iterator o = offsets.begin();
        o != offsets.end(); o++) {
      mpz_class val = base + *o;
      if (val > max_val)
        break;
      if (!candidate_killed_by(val, prime))
        new_offsets.push_back(val);
    }
    base + primorial;
    counter += 1;
  }

  return new_offsets;
}

uint32_t generate_prime_clusters(mpz_class max_val, uint32_t max_sieve,
    bool verbose) {
  uint32_t primorial_start = 7;

  mpz_class primorial = 210;
  mpz_class big97 = 97;

  std::vector<mpz_class> offsets;
  offsets.push_back(big97);

  uint32_t *prime_test_table;
  uint32_t prime_test_size;
  sieve(&prime_test_table, &prime_test_size, max_sieve);

  for (uint32_t i = primorial_start+1; i < prime_test_size; i++) {
    mpz_class prime = i;
    offsets = add_next_prime(offsets, max_val, prime, primorial);
    primorial *= i;
  }
  uint32_t count = 0;
  for (std::vector<mpz_class>::iterator o = offsets.begin();
      o != offsets.end(); o++) {
    if (is_valid_pow(*o)) {
      count++;
      if (verbose)
        std::cout << "Found valid POW " << *o << std::endl;
    }
  }
  return count;
}
