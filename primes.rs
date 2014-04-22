extern crate num;
extern crate rand;
extern crate bignum = "bignum#0.1.1-pre";

use bignum::{BigUint, ToBigUint,RandBigInt};
use num::{div_mod_floor, Integer};
use std::num::{Zero, One, pow, Float};
use rand::task_rng;
use std::vec::Vec;
use std::iter;
use std::clone::Clone;
use std::ops::{Shr, BitAnd};

fn to_biguint (i : u64) -> BigUint {
    i.to_biguint().unwrap()
}

// Source: https://gist.github.com/jsanders/8739134#file-generate_big_primes-rs-L35
fn mod_exp<T: Integer + Clone + Shr<uint, T> + BitAnd<T,T>>(base: &T, exponent: &T, modulus: &T) -> T {
    let (zero, one): (T, T) = (Zero::zero(), One::one());
    let mut result = one.clone();
    let mut baseAcc = base.clone();
    let mut exponentAcc = exponent.clone();

    while exponentAcc > zero {
        // Accumulate current base if current exponent bit is 1
        if (exponentAcc & one) == one {
            result = result.mul(&baseAcc);
            result = result.rem(modulus);
        }
        // Get next base by squaring
        baseAcc = baseAcc * baseAcc;
        baseAcc = baseAcc % *modulus;

        // Get next bit of exponent
        exponentAcc = exponentAcc.shr(&1);
    }
    result
}


/* Since we use Miller-Rabin, this function technically finds if a number is
 * *probably* correct.
 */
fn is_prime(n : &BigUint) -> bool {
    let zero : BigUint = Zero::zero();
    let one : BigUint = One::one();
    let two = one + one;
    assert!(n >= &two);
    let mut s = zero.clone();
    let mut d = n-one;
    let num_trials = 5;
    if n == &two {
        return true
    } else if n == &one || n.is_even() {
        return false
    } else {
        loop {
            let (quotient, remainder) = div_mod_floor(d.clone(), two.clone());
            if remainder == one { break }
            s = s + one;
            d = quotient;
        }
    }

    fn try_composite(a : &BigUint, d : &BigUint, n : &BigUint, s : &BigUint) -> bool {
        let one = to_biguint(1);
        let two = to_biguint(2);
        if mod_exp(a, d, n) == one {
            return false
        }
        let mut i : uint = 0;
        loop {
            if &i.to_biguint().unwrap() == s {
                break
            }
            if mod_exp(a, &(pow(two.clone(), i) * *d), n) == n-one {
                return false
            }
            i += 1;
        }
        true
    }

    let mut rng = task_rng();
    for _ in range(0, num_trials) {
        let a = rng.gen_biguint_range(&two, n);
        if try_composite(&a, &d, n, &s) {
            return false
        }
    }
    true
}

fn is_valid_pow(prime : uint) -> bool {
    for &offset in [0u, 4u, 6u, 10u, 12u, 16u].iter() {
        if !fermat_is_prime(prime + offset) {
            return false;
        }
    }
    true
}

fn fermat_is_prime(n : uint) -> bool {
    if n == 2 {
        true
    } else if n.is_even() {
        false
    } else if mod_exp(&2, &(n-1), &n) == 1 {
        true
    } else {
        false
    }
}

fn gen_prime(max_val : uint, verbose : bool) -> uint {
    //http://rosettacode.org/wiki/Sieve_of_Eratosthenes#Rust
    fn simple_sieve(limit: uint) -> ~[uint] {
        #[inline]
        fn int_sqrt(n: uint) -> uint { (n as f64).sqrt() as uint }

        if limit < 2 {
            return ~[];
        }

        let mut primes = Vec::from_elem(limit + 1, true);

        for prime in iter::range_inclusive(2, int_sqrt(limit) + 1) {
            if *primes.get(prime) {
                for multiple in iter::range_step(prime * prime, limit + 1, prime) {
                    *primes.get_mut(multiple) = false
                }
            }
        }
        iter::range_inclusive(2, limit).filter(|&n| *primes.get(n)).collect()
    }

    fn candidate_killed_by (candidate : uint, prime : uint) -> bool {
        let offsets : Vec<uint> = Vec::from_slice([0u, 4u, 6u, 10u, 12u, 16u]);
        for offset in offsets.iter() {
            if (candidate + *offset) % prime == 0 {
                return true;
            }
        }
        false
    }

    fn add_next_prime (max_val : uint, offsets : Vec<uint>, prime : uint, primorial : uint) -> Vec<uint> {
        let mut base : uint = 0u;
        let mut new_offsets : Vec<uint> = Vec::new();
        for _ in range(0u, prime) {
            if base > max_val {
                break
            }
            for o in offsets.iter() {
                    let val = base + *o;
                    if val > max_val {
                        break
                    }
                    if !candidate_killed_by(val, prime) {
                        new_offsets.push(val);
                    }
            }
            base = base + primorial;
        }
        return new_offsets
    }
    let primorial_max : uint = 29u;
    let primorial_start : uint = 7u;
    let mut primorial : uint = 210u;  // 2*3*5*7
    let mut offsets : Vec<uint> = Vec::from_slice([97u]);
    for i in simple_sieve(primorial_max).iter() {
        if *i <= primorial_start {
            continue
        }
        offsets = add_next_prime(max_val, offsets, *i, primorial);
        primorial = primorial * *i;
    }
    let mut count : uint = 0;
    for &o in offsets.iter() {
        if is_valid_pow(o) {
            count = count + 1;
            if verbose { println!("prime: {}", o) }
        }
    }
    count
}

#[cfg(test)]
mod test_primes {
    use super::gen_prime;

    #[test]
    fn gen_prime_is_correct() {
        let v = gen_prime(1000000000, true);
        println!("{}", v);
        assert!(v == 81u);
    }
}

fn main() {
    println!("{}", gen_prime(1000000000, true));
}