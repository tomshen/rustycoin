extern crate num;
extern crate rand;
extern crate bignum = "bignum#0.1.1-pre";

use bignum::{BigUint, ToBigUint,RandBigInt};
use num::{div_mod_floor, Integer, div_floor};
use std::num::{Zero, One, pow, Float};
use rand::task_rng;
use std::vec::Vec;
use std::iter;
use std::ops;
use std::iter::FromIterator;

use std::os;

fn to_biguint (i : u64) -> BigUint {
    i.to_biguint().unwrap()
}



// Source: https://gist.github.com/jsanders/8739134#file-generate_big_primes-rs-L35
fn mod_exp(base: &BigUint, exponent: &BigUint, modulus: &BigUint) -> BigUint {
    let (zero, one): (BigUint, BigUint) = (Zero::zero(), One::one());
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

//http://rosettacode.org/wiki/Sieve_of_Eratosthenes#Rust
fn int_sqrt(n: uint) -> uint {
    (n as f64).sqrt() as uint
}
fn simple_sieve(limit: uint) -> ~[uint] {

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

fn to_uint(n : &BigUint) -> uint {
    5
}

fn wheel_factorization(n : &BigUint, m : uint) {
    let one = to_biguint(1);
    let zero = to_biguint(0);
    let primes = simple_sieve(m);
    let M = primes.iter().fold(1, |x, y| {x*(*y)});
    let mut sieve = Vec::from_elem(M, true);
    for p in primes.iter() {
        *sieve.get_mut(p-1) = false;
        for j in iter::range_step(*p*(*p), M+1, *p) {
            *sieve.get_mut(j-1) = false;
        }
    }
    let mut k : Vec<uint> = range(0, M).map(|x| {x+1}).filter(|x| {*sieve.get(*x)}).collect();
    let N = n.clone();
    let M = M.to_biguint().unwrap();
    if N.mod_floor(&M) != zero { 
        let N = N + M;
    }
    let N = N.div_floor(&M);
    let maxP = int_sqrt(M*N);
    let mut sieve : Vec<Vec<bool>> = (range(0, k.len()).map(|_| {Vec::from_elem(to_uint(&N), true)})).collect(); //eventually, make bigVecs
    *sieve.get_mut(0).get_mut(0) = false;
    let mut row = zero;
    
    while row < N{
        let baseVal = M * row;
        for subset in range(0, k.len()) {
            if *sieve.get(subset).get(to_uint(&row)) {
                let p = baseVal + k.get(subset).to_biguint().unwrap();
                primes.push(p);
                if p > maxP { continue }
                
            }
        }
        row = row + one;
    }

}



#[test]
fn sieve_is_correct() {
    let v = simple_sieve(10);
    assert!(v == ~[2, 3, 5, 7]);
}

#[test]
fn tom_is_rude() {
    assert!(true);
}

fn main () {
    let args : ~[~str] = os::args();
    if args.len() < 2 {
        println!("You need to provide a natural number as an input argument.")     
    } else {
        match from_str::<BigUint>(args[1]) {
            Some(n) => {
                let p : bool = is_prime(&n);
                 
                println!("prime: {}", p);
            },
            None => {
                println!( "You need to provide a natural number as an input argument.")
            }
        };
    }

}
