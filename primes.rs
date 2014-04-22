extern crate num;
extern crate rand;
extern crate bignum = "bignum#0.1.1-pre";

use bignum::{BigUint, ToBigUint, RandBigInt};
use num::{div_mod_floor, Integer};
use std::num::{Zero, One, pow, Float, FromPrimitive};
use rand::{task_rng, Rng};
use rand::distributions::range::SampleRange;
use std::vec::Vec;
use std::iter;
use std::clone::Clone;
use std::ops::{Shr, BitAnd};

static pow_offsets : &'static[uint] = &[0u, 4u, 6u, 10u, 12u, 16u];

#[inline]
fn big<T: ToBigUint>(i : T) -> BigUint {
    i.to_biguint().unwrap()
}

// Source: https://gist.github.com/jsanders/8739134#file-generate_big_primes-rs-L35
fn mod_exp<T: Integer + Clone + Shr<uint,T> + BitAnd<T,T>>
    (base: &T, exponent: &T, modulus: &T) -> T {

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


// Rabin-Miller primality test, probabilistic
fn try_composite(a : &BigUint, d : &BigUint, n : &BigUint, s : &BigUint) -> bool {

    let one : BigUint = One::one();
    if mod_exp(a, d, n) == one {
        return false
    }
    let mut i = 0u;
    loop {
        if big(i) == *s {
            break
        }
        if mod_exp(a, &(pow(one+one, i) * *d), n) == *n-one {
            return false
        }
        i += 1;
    }
    true

}
fn rm_is_prime(n : &BigUint) -> bool {

    let (zero, one): (BigUint, BigUint) = (Zero::zero(), One::one());
    let two = one + one;

    assert!(n >= &two);

    let mut s = zero.clone();
    let mut d = *n-one;
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

    let mut rng = task_rng();
    for _ in range(0, num_trials) {
        let a = rng.gen_biguint_range(&two, n);
        if try_composite(&a, &d, n, &s) {
            return false
        }
    }
    true

}

fn fermat_is_prime(n : &BigUint) -> bool {
    let (zero, one): (BigUint, BigUint) = (Zero::zero(), One::one());
    let two = one + one;

    if n == &two {
        true
    } else if n.is_even() {
        false
    } else if mod_exp(&two, &(n-one), n) == one {
        true
    } else {
        false
    }
}

fn is_valid_pow(prime : &BigUint) -> bool {
    for offset in [big(0u), big(4u), big(6u), big(10u), big(12u), big(16u)].iter() {
        if !fermat_is_prime(&(*prime + *offset)) {
            return false;
        }
    }
    true
}


// Source: http://rosettacode.org/wiki/Sieve_of_Eratosthenes#Rust
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

fn gen_prime(max_val : &BigUint, verbose : bool) -> uint {

    fn candidate_killed_by(candidate : &BigUint, prime : &BigUint) -> bool {
        let zero : BigUint = Zero::zero();
        for &offset in pow_offsets.iter() {
            let o : BigUint = FromPrimitive::from_uint(offset).unwrap();
            if (*candidate + o) % *prime == zero {
                return true;
            }
        }
        false
    }

    fn add_next_prime (max_val : &BigUint, offsets : Vec<BigUint>, prime : BigUint, primorial : BigUint) -> Vec<BigUint> {
        let (zero, one): (BigUint, BigUint) = (Zero::zero(), One::one());
        let mut base : BigUint = Zero::zero();
        let mut new_offsets : Vec<BigUint> = Vec::new();
        let mut counter = zero.clone();
        while counter < prime {
            if base > *max_val {
                break
            }
            for o in offsets.iter() {
                    let val = base + *o;
                    if val > *max_val {
                        break
                    }
                    if !candidate_killed_by(&val, &prime) {
                        new_offsets.push(val);
                    }
            }
            base = base + primorial;
            counter = counter + one;
        }
        return new_offsets
    }
    let primorial_max : uint = 29u;
    let primorial_start : uint = 7u;
    let mut primorial : uint = 210u;  // 2*3*5*7
    let mut offsets : Vec<BigUint> = Vec::from_slice([big(97u)]);
    for i in simple_sieve(primorial_max).iter() {
        if *i <= primorial_start {
            continue
        }
        offsets = add_next_prime(max_val, offsets, big(*i), big(primorial));
        primorial = primorial * *i;
    }
    let mut count : uint = 0;
    for o in offsets.iter() {
        if is_valid_pow(o) {
            count = count + 1;
            if verbose { println!("prime: {}", o) }
        }
    }
    count
}

#[cfg(test)]
mod test_primes {
    use super::{big, gen_prime};

    #[test]
    fn gen_prime_is_correct() {
        let v = gen_prime(&big(1000000000), false);
        println!("{}", v);
        assert!(v == 81u);
    }
}

fn main() {
    println!("{}", gen_prime(&big(1000000000000), true));
}