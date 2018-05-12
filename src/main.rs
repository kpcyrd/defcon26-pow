#![cfg_attr(feature = "nightly", feature(test))]
#[cfg(feature = "nightly")]
extern crate test;

extern crate num_cpus;
extern crate sha2;
extern crate byteorder;
extern crate num_bigint;
extern crate num_traits;

use std::env;
use std::io::prelude::*;
use byteorder::{LittleEndian, WriteBytesExt};
use sha2::{Sha256, Digest};
use num_bigint::BigUint;
use num_traits::Zero;


#[inline]
fn pow_hash(challenge: &str, solution: u64) -> Vec<u8> {
    let mut wtr = vec![];
    wtr.write(challenge.as_bytes()).unwrap();
    wtr.write_u64::<LittleEndian>(solution).unwrap();
    Vec::from(Sha256::digest(&wtr).as_slice())
}

#[inline]
fn check_pow(challenge: &str, n: u8, solution: u64) -> bool {
    let h = pow_hash(challenge, solution);
    let num = BigUint::from_bytes_be(&h);

    let op = 2u64.pow(n as u32);

    (num % op).is_zero()
}

fn solve_pow(challenge: &str, n: u8) -> u64 {
    let mut candidate = 0;

    loop {
        if check_pow(challenge, n, candidate) {
            break;
        }
        candidate += 1;
    }

    candidate
}

fn main() {
    let mut args = env::args().skip(1);
    let challenge = args.next().expect("challenge missing");
    let n = args.next().expect("n missing");
    let n = n.parse::<u8>().expect("n is not u8");
    println!("Solving challenge: {:?}, n: {:?}", challenge, n);

    let solution = solve_pow(&challenge, n);

    println!("Solution: {:?} -> {:?}", solution, pow_hash(&challenge, solution));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(feature = "nightly")]
    use test::Bencher;

    #[test]
    fn test_check_pow_valid() {
        let valid = check_pow("e2ZgIzlOpe", 26, 52644528);
        assert!(valid);
    }

    #[test]
    fn test_check_pow_invalid() {
        let valid = check_pow("e2ZgIzlOpe", 26, 1);
        assert!(!valid);
    }

    #[test]
    fn test_pow_hash() {
        let hash = pow_hash("e2ZgIzlOpe", 52644528);

        let hash = hash.into_iter()
                        .map(|b| format!("{:02x}", b))
                        .collect::<String>();

        assert_eq!("a51496f8ce009bab48108eaaa085b749b39c8707ae622e8d446a5c9228000000", hash);
    }

    #[bench]
    #[cfg(feature = "nightly")]
    fn bench_pow_hash(b: &mut Bencher) {
        b.iter(|| pow_hash("e2ZgIzlOpe", 52644528));
    }

    #[bench]
    #[cfg(feature = "nightly")]
    fn bench_pow_valid(b: &mut Bencher) {
        b.iter(|| check_pow("e2ZgIzlOpe", 26, 52644528));
    }
}
