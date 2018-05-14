#![cfg_attr(feature = "nightly", feature(test))]
#[cfg(feature = "nightly")]
extern crate test;

extern crate num_cpus;
extern crate sha2;
extern crate byteorder;
extern crate num_bigint;
extern crate num_traits;
extern crate rayon;

use rayon::prelude::*;

use std::env;
use byteorder::{LittleEndian, WriteBytesExt};
use sha2::{Sha256, Digest};
use num_bigint::BigUint;
use num_traits::Zero;

const MAX_CANDIDATES: u64 = 100_000_000;

#[inline]
fn pow_hash(challenge: &str, solution: u64) -> Vec<u8> {
    let mut wtr = Vec::from(challenge);
    wtr.write_u64::<LittleEndian>(solution).unwrap();
    Sha256::digest(&wtr).to_vec()
}

#[inline]
fn check_pow(challenge: &str, n: u8, solution: u64) -> bool {
    let h = pow_hash(challenge, solution);
    let num = BigUint::from_bytes_be(&h);

    // 32 bit is sufficient, if we want to support max n = 30 next year
    let op = 2u32.pow(u32::from(n));

    (num % op).is_zero()
}

#[inline]
fn solve_pow(candidate: u64, challenge: &str, n: u8) -> Option<u64> {
    if check_pow(&challenge, n, candidate) {
        Some(candidate)
    } else {
        None
    }
}

fn hexdump(bytes: &[u8]) -> String {
    bytes.into_iter()
        .map(|b| format!("{:02x}", b))
        .collect::<String>()
}

fn main() {
    let mut args = env::args().skip(1);
    let challenge = args.next().expect("challenge missing");
    let n = args.next().expect("n missing");
    let n = n.parse::<u8>().expect("n is not u8");
    println!("Solving challenge: {:?}, n: {:?}", challenge, n);

    let solution = (0..MAX_CANDIDATES).into_par_iter()
                                      .map(|x| solve_pow(x, &challenge, n))
                                      .find_any(|x| x.is_some());

    // take solution
    println!("Solution: {:?} -> {:?}", solution, hexdump(&pow_hash(&challenge, solution.unwrap().unwrap())));
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
        assert_eq!("a51496f8ce009bab48108eaaa085b749b39c8707ae622e8d446a5c9228000000", hexdump(&hash));
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
