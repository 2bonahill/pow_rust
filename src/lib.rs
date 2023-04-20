// A basic PoW implementation
#![allow(dead_code)] // TODO: remove
extern crate rand;
use byteorder::{LittleEndian, WriteBytesExt};
use rand::Rng;
use sha3::{Digest, Sha3_256};

const OUT_SIZE: usize = 8;
const IN_SIZE: usize = 32;
const DATA_SIZE: usize = 40;

type Hash = [u8; OUT_SIZE];
type InHash = [u8; IN_SIZE];
type Data = [u8; DATA_SIZE];

pub fn generate_pow(hash: &InHash, difficulty: u64) -> String {
    let target = get_target(difficulty);
    let entropy = get_random_seed();
    let mut data = get_data(&entropy, hash);
    let mut h = [0u8; OUT_SIZE];
    loop {
        compute_hash(&mut h, &data);
        if greater(&h, &target) {
            return hex::encode(data_to_nonce(&data));
        }
        if !next_data(&mut data, entropy.len()) {
            let new_entropy = get_random_seed();
            let new_data = get_data(&new_entropy, hash);
            data.clone_from_slice(&new_data);
        }
    }
}

// pub fn benchmark_pow(difficulty: u64) -> String {
//     let target = get_target(difficulty);
//     let mut data = vec![0u8; OUT_SIZE];
//     let mut h = [0u8; OUT_SIZE];
//     loop {
//         hash_func(&mut h, &data);
//         if greater(&h, &target) {
//             return hex::encode(data_to_nonce(&data));
//         }
//         if !next_data(&mut data.to_vec(), OUT_SIZE) {
//             data = vec![0u8; OUT_SIZE];
//         }
//     }
// }

// ok
fn compute_hash(hash: &mut [u8], data: &Data) {
    let mut sha3 = Sha3_256::new();
    sha3.update(data);
    let digest = sha3.finalize();
    hash.copy_from_slice(&digest[0..8]);
}

// ok
pub fn get_target(difficulty: u64) -> Hash {
    let big: u128 = 1 << 64;
    let target: u128 = big - (big / difficulty as u128);

    // make x little endian
    let mut lev: Vec<u8> = Vec::new();
    lev.write_u128::<LittleEndian>(target).unwrap();
    lev.resize(8, 0);
    let h: Hash = lev.try_into().unwrap_or_else(|v: Vec<u8>| {
        panic!(
            "Failed converting into [u8; 8]. Vec was of length {}",
            v.len()
        )
    });
    h
}

// ok
fn next_data(data: &mut Data, max_size: usize) -> bool {
    for i in 0..max_size {
        data[i] = data[i].wrapping_add(1);
        if data[i] != 0 {
            return true;
        }
    }
    false
}

// ok
fn greater(a: &Hash, b: &Hash) -> bool {
    for i in (0..OUT_SIZE).rev() {
        if a[i] == b[i] {
            continue;
        }
        return a[i] > b[i];
    }
    true
}

// ok
pub fn get_random_seed() -> Hash {
    let mut h: Hash = [0u8; OUT_SIZE];
    for i in 0..OUT_SIZE {
        h[i] = rand::thread_rng().gen();
    }
    h
}

// ok
fn get_data(entropy: &Hash, hash: &InHash) -> Data {
    let mut data = vec![0u8; DATA_SIZE];

    for i in 0..entropy.len() {
        data[i] = entropy[i];
    }

    for i in 0..hash.len() {
        data[i + entropy.len()] = hash[i];
    }

    data.try_into().unwrap_or_else(|v: Vec<u8>| {
        panic!(
            "Failed converting into [u8; DATA_SIZE]. Vec was of length {}",
            v.len()
        )
    })
}

// ok
fn data_to_nonce(data: &Data) -> Hash {
    let mut nonce: Hash = [0u8; 8];
    nonce.copy_from_slice(&data[0..8]);
    nonce
}

#[cfg(test)]
mod tests {

    use std::time::Instant;

    use super::*;

    #[test]
    fn test_hash_workflow() {
        // generate some random hash in data and set up the scene
        let in_hash: InHash = [0; IN_SIZE].map(|_| -> u8 { rand::thread_rng().gen() });
        let _target = get_target(3);
        let entropy = get_random_seed();

        // get data
        let data = get_data(&entropy, &in_hash);
        assert_eq!(data.len(), DATA_SIZE);

        // compute the  hash
        let mut hash: Hash = [0; OUT_SIZE];
        compute_hash(&mut hash, &data);
        dbg!(&hash);

        // the nonce will be the return value
        let _nonce = data_to_nonce(&data);
    }

    #[test]
    fn test_utils() {
        let mut next_data_test = [
            255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        let mut next_data_test2 = [
            255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];

        assert!(next_data(&mut next_data_test, 8));
        assert!(!next_data(&mut next_data_test2, 8));

        assert!(super::greater(
            &[0, 0, 0, 0, 0, 0, 0, 2],
            &[9, 0, 0, 0, 0, 0, 0, 1]
        ));
    }

    #[test]
    fn test_generate_pow() {
        let in_hash: InHash = [0; IN_SIZE].map(|_| -> u8 { rand::thread_rng().gen() });

        for i in 0..10 {
            let start = Instant::now();
            let x = generate_pow(&in_hash, 1 << i);
            let duration = start.elapsed();
            println!(
                "Round number: {} - {} // Time: {}ns",
                i,
                &x,
                duration.as_nanos()
            );
        }
    }
}
