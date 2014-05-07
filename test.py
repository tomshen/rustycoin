#!/usr/bin/env python3
import subprocess
import sys
import unittest

import primes

class TestProofOfWork(unittest.TestCase):
    def test_is_prime(self):
        for n in range(2,10**20):
            rs_out = subprocess.check_output(['./primes', str(n)],
                    stderr=subprocess.STDOUT)
            rs_out = True if rs_out.strip().capitalize() == b'True' else False
            py_out = primes.is_probable_prime(n)
            self.assertEqual(py_out, rs_out, 'Primality tests disagree on %d' % n)
            if rs_out:
                print('%d' % n)

if __name__ == '__main__':
    unittest.main()
