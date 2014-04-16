extern crate num;
extern crate rand;
extern crate bignum = "bignum#0.1.0-pre";

use bignum::{BigUint, ToBigUint,RandBigInt};
use num::{div_mod_floor, Integer};
use std::num::{Zero, One, pow};
use rand::task_rng;

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

fn main () {
    let args : ~[~str] = os::args();
    if args.len() < 2 {
        println!("You need to provide a natural number as an input argument.")
    } else {
        match from_str::<BigUint>(args[1]) {
            Some(n) => {
                let p : bool = is_prime(&n);
                println!("{}", p);
            },
            None => println!(
                "You need to provide a natural number as an input argument.")
        };
    }

}
