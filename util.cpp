#include <stdarg.h>

#include <gmp.h>

// From gmp-6.0.0a/mpz/inits.c
void mpz_inits (mpz_ptr x, ...) {
  va_list  ap;

  va_start (ap, x);

  while (x != NULL)
    {
      mpz_init (x);
      x = va_arg (ap, mpz_ptr);
    }
  va_end (ap);
}

