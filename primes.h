#include <stdint.h>

#include <gmpxx.h>

#define FERMAT_ITERS 1000

void sieve(uint32_t** prime_test_table, uint32_t* prime_test_size,
    uint32_t prime_test_limit);

bool is_prime_fermat(mpz_class n, int k=FERMAT_ITERS);

bool is_valid_pow(mpz_class candidate);
