extern crate num;
extern crate rand;
extern crate time;
extern crate sync;

extern crate bignum = "bignum#0.1.1-pre";

use bignum::{BigUint, ToBigUint, RandBigInt};
use num::{div_mod_floor, Integer};
use std::num::{Zero, One, pow, Float, FromPrimitive};
use rand::{task_rng, Rng};
use rand::distributions::range::SampleRange;
use std::vec::Vec;
use std::iter;
use std::iter::{FromIterator};
use std::clone::Clone;
use std::ops::{Shr, BitAnd};
use std::bool;

use sync::Arc;

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

fn is_valid_pow(prime : &BigUint) -> Option<BigUint> {
    for offset in [big(0u), big(4u), big(6u), big(10u), big(12u), big(16u)].iter() {
        if !fermat_is_prime(&(*prime + *offset)) {
            return None;
        }
    }
    Some(prime.clone())
}

// Simple sieve
fn simple_sieve(max_sieve : uint, verbose : bool) -> Vec<uint> {
    #[inline]
    fn int_sqrt(n: uint) -> uint { (n as f64).sqrt() as uint }

    let mut primes = Vec::from_elem(max_sieve+1, true);
    *primes.get_mut(0) = false;
    *primes.get_mut(1) = false;
    for prime in range(2, int_sqrt(max_sieve) + 1) {
        if *primes.get(prime) {
            for multiple in iter::range_step(prime * prime, max_sieve + 1, prime) {
                *primes.get_mut(multiple) = false
            }
            if verbose { println!("Sieved {}", prime) }
        }
    }
    if verbose { println!("Finished sieving") }
    range(2, max_sieve+1).filter(|&n| *primes.get(n)).collect()
}
fn candidate_killed_by(candidate : &BigUint, prime : &BigUint) -> bool {
    let zero : BigUint = Zero::zero();
    for &offset in pow_offsets.iter() {
        let o : BigUint = big(offset);
        if (*candidate + o) % *prime == zero {
            return true;
        }
    }
    false
}

fn add_next_prime (max_val : &BigUint, offsets : Vec<BigUint>,
    prime : BigUint, primorial : BigUint, min_val : BigUint) -> Vec<BigUint> {
    let (zero, one): (BigUint, BigUint) = (Zero::zero(), One::one());
    let mut base : BigUint = zero.clone();
    let mut new_offsets : Vec<BigUint> = Vec::new();
    let mut counter = zero.clone();
    while counter < prime {
        if base + min_val > *max_val {
            break
        }
        for o in offsets.iter() {
            let val = min_val + base + *o;
            if val > *max_val {
                break
            }
            if !candidate_killed_by(&val, &prime) {
                new_offsets.push(val - min_val);
            }
        }
        base = base + primorial;
        counter = counter + one;
    }
    new_offsets
}

fn wheel_sieve(sieved : &Vec<uint>, base_val : &BigUint, max_val : &BigUint) -> Vec<BigUint> {
    let primorial_start : uint = 7u;
    let mut primorial : uint = 210u;  // 2*3*5*7
    let mut offsets : Vec<BigUint> = Vec::from_slice([big(97u)]);

    //let mut start_time = time::precise_time_s();
    let mut min_val : BigUint = Zero::zero();
    for &i in sieved.iter() {
        if i <= primorial_start {
            continue
        }
        min_val = (base_val / big(primorial)) * big(primorial);
        offsets = add_next_prime(max_val, offsets, big(i), big(primorial),
            min_val.clone());
        primorial = primorial * i;
    }
    //println!("Time to sieve sieve: {}", time::precise_time_s() - start_time);

    offsets.retain(|o| is_valid_pow(&(o+min_val)) != None);
    offsets
}

fn gen_prime(base_val : &BigUint, max_val : &BigUint, max_sieve : uint, verbose : bool) -> uint {
    let sieved = simple_sieve(max_sieve, verbose);
    let offsets = wheel_sieve(&sieved, base_val, max_val);
    for o in offsets.iter() {
        println!("{}", o);
    }
    offsets.len()
}

fn gen_prime_par(base_val : &BigUint, max_val : &BigUint, max_sieve : uint,
    num_tasks : uint, verbose : bool) -> uint {
    let sieved = Arc::new(simple_sieve(max_sieve, verbose));

    let mut count = 0;
    let (tx, rx) = channel();
    for i in range(0, num_tasks) {
        let start_val = base_val + (big(i) * (max_val-*base_val)) / big(num_tasks);
        let end_val = base_val + (big(i+1) * (max_val-*base_val)) / big(num_tasks);
        let child_tx = tx.clone();
        let child_sieved = sieved.clone();
        spawn(proc() {
            child_tx.send(wheel_sieve(child_sieved.deref(), &start_val, &end_val).len());
        });
    }

    for i in range(0, num_tasks) {
        count += rx.recv();
    }
    count
}

#[cfg(test)]
mod test_primes {
    use super::{big, gen_prime, simple_sieve};

    #[test]
    fn gen_prime_is_correct() {
        assert!(gen_prime(&big(0), &big(100000000), 29, false) == 81u);
    }

    #[test]
    fn simple_sieve_is_correct() {
        assert!(simple_sieve(&mut Vec::new(), 5000000, false).len() == 348513);
    }
}

fn main() {
    let args = std::os::args();
    let min_val = big(0);
    let max_val = big(1000000000);
    if args.len() == 3 && args[1] == "-p".to_owned() {
        let num_tasks = from_str::<uint>(args[2]).unwrap();
        println!("{}", gen_prime_par(&min_val, &max_val, 210, num_tasks, true));
    } else if args[1] == "-s".to_owned() {
        println!("{}", gen_prime(&min_val, &max_val, 210, true));
    }
}