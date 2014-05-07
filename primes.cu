#include <iostream>
#include <stdint.h>
#include <string.h>
#include <vector>

#include <thrust/host_vector.h>
#include <thrust/device_vector.h>
#include <thrust/copy.h>
#include <thrust/execution_policy.h>
#include <ctime>
#include <curand_kernel.h>

#include "util.h"
#include "big_integer.h"
#include "primes.h"

bigi cluster_offsets[6] = {0, 4, 2, 4, 2, 4};

__device__
bool is_even(bigi n) {
  bigi one = bigi(1);
  bigi zero = bigi(0);
  bigi x = n & one;
  return x == zero;
}

__device__
bigi mod_exp(bigi base, bigi exponent, bigi mod) {
  bigi result = 1;
  bigi base_acc = base;
  bigi exp_acc = exponent;
  bigi one = bigi(1);
  while (exp_acc > bigi(0)) {
    bigi x = exp_acc & one;
    if (x == one) {
      bigi temp = result * base_acc;
      result = temp;

      result = result % mod;
    }
    base_acc = base_acc * base_acc;
    base_acc = base_acc % mod;
    exp_acc >>= one;
  }
  return result;
}

__device__
bool is_prime_fermat(bigi n, int k=1000) {
  bigi rem, a;

  curandState state;
  curand_init((unsigned int)n, 0, 0, &state);

  if (n == bigi(2)) {
    return true;
  } else if (is_even(n)) {
    return false;
  }

  for (int i = 0; i < k; i++) {
    bigi nmo = n - bigi(1);
    a = bigi(curand(&state)) % nmo;
    a += 1; // 1 <= a < n
    rem = mod_exp(a, nmo, n);
    if (rem != bigi(1))
      return false;
  }

  return true;
}
struct is_valid_pow {

  __device__
  bool operator()(const bigi candidate) {
    bigi cluster_offsets[6] = {0, 4, 2, 4, 2, 4};
    bigi val = candidate;
    for (int i = 0; i < 6; i++) {
      val += cluster_offsets[i];
      if (!is_prime_fermat(val))
        return false;
    }
    return true;
  }

};

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

bool candidate_killed_by(bigi candidate, bigi prime) {
  bigi p = candidate;
  for(int i = 0; i < 6; i++) {
    p += cluster_offsets[i];
    if (p % prime == bigi(0))
      return true;
  }
  return false;
}


thrust::host_vector<bigi> add_next_prime(thrust::host_vector<bigi> offsets,
    bigi max_val, bigi prime, bigi primorial) {
  bigi base = 0;
  bigi counter = 0;

  thrust::host_vector<bigi> new_offsets;

  while (counter < prime) {
    if (base > max_val)
      break;
    for (thrust::host_vector<bigi>::iterator o = offsets.begin();
        o != offsets.end(); o++) {
      bigi val = base + *o;
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

std::vector<bigi> generate_prime_clusters(bigi max_val, uint32_t max_sieve,
    bool verbose) {
  uint32_t primorial_start = 7;

  bigi primorial = 210;
  bigi big97 = 97;

  thrust::host_vector<bigi> offsets;
  offsets.push_back(big97);

  uint32_t *prime_test_table;
  uint32_t prime_test_size;
  DEBUG("Starting sieve")
  sieve(&prime_test_table, &prime_test_size, max_sieve);
  DEBUG("Finished sieve")

  DEBUG("Starting adding primes")

  for (uint32_t i = 0; i < prime_test_size; i++) {
    if (prime_test_table[i] <= primorial_start)
      continue;
    bigi prime = prime_test_table[i];
    offsets = add_next_prime(offsets, max_val, prime, primorial);
    primorial = primorial * prime;
    DEBUG(prime_test_table[i])
  }
  DEBUG("Finished adding primes")
  DEBUG("Checking if PoWs")

  thrust::device_vector<bigi> candidates = offsets;
  thrust::device_vector<bigi> clusters;

  thrust::copy_if(candidates.begin(), candidates.end(), clusters.begin(), is_valid_pow());

  thrust::host_vector<bigi> result = clusters;
  std::vector<bigi> primes(result.begin(), result.end());

  return primes;
}
