extern crate num;

use num::bigint::BigUint;
use num::bigint::ToBigUint;
use num::mod_floor;

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
    if n.eq(&two) || n.eq(&three) {
        true
    } else if n.eq(&one) || mod_floor(n, two).eq(&zero) {
        false
    } else {
        true
    }
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