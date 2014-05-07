#include <iostream>
#include <stdint.h>

#define DEBUG(x) std::cerr << x << std::endl;

/*
#include <gmpxx.h>

#define FERMAT_ITERS 1000

void sieve(uint32_t** prime_test_table, uint32_t* prime_test_size,
    uint32_t prime_test_limit);

bool is_prime_fermat(mpz_class n, int k=FERMAT_ITERS);

bool is_valid_pow(mpz_class candidate);

uint32_t generate_prime_clusters(mpz_class max_val, uint32_t max_sieve,
    bool verbose);
*/


#include <vector>

#include "big_integer.h"

#define bigi thrust::big_integer<128>

std::vector<bigi> generate_prime_clusters(bigi max_val, uint32_t max_sieve, bool verbose);
