extern crate num;
extern crate rand;

use num::bigint::{BigUint, ToBigUint,RandBigInt};
use num::{mod_floor, div_mod_floor};
use std::num::pow;
use rand::{task_rng, Rng};

use std::os;

fn to_biguint (i : u64) -> BigUint {
    i.to_biguint().unwrap()
}



/* Since we use Miller-Rabin, this function technically finds if a number is
 * *probably* correct.
 */
fn is_prime(n : BigUint) -> bool {
    let zero = to_biguint(0);
    let one = to_biguint(1);
    let two = to_biguint(2);
    let three = to_biguint(3);
    let mut s = zero;
    let mut d = n-one;
    let num_trials = 20;
    if n.eq(&two) || n.eq(&three) {
        return true
    } else if n.eq(&one) || mod_floor(n, two).eq(&zero) {
        return false
    } else {
        while true {
            let (quotient, remainder) = div_mod_floor(d, two); 
            if remainder == one { break }
            s = s + one;
            d = quotient;
        }
    }

    fn try_composite(a : BigUint, d : BigUint, n : BigUint, s : BigUint) -> bool {
        let one = to_biguint(1);
        if (mod_floor(pow(a, d), n) == one) { return false }
        for i in range(0, s) {
            if (mod_floor(pow(a, (pow(2,i) * d)), n) == n-one) { return false }            
        }
        return true;
    }

    for i in range(0, num_trials) {
        let mut rng = task_rng();
        let a : BigUint = rng.gen_biguint_range(&two, &n);
        if try_composite(a, d, n, s) { return false }
    }
    return true;
}

fn main () {
    let args : ~[~str] = os::args();
    if args.len() < 2 {
        println!("You need to provide a natural number as an input argument.")
    } else {
        match from_str::<BigUint>(args[1]) {
            Some(n) => {
                let p : bool = is_prime(n.clone());
                println!("Is {} is prime? {}", n, if p {"Yes"} else {"No"});
            },
            None => println!(
                "You need to provide a natural number as an input argument.")
        };
    }

}