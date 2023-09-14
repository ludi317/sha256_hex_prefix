use itertools::Itertools;
use sha2::{Digest, Sha256};
use std::env;

const NUMS: [u8; 16] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
const WORDS: [&str; 16] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "a", "b", "c",
    "d", "e", "f",
];

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Give the length of the hex string prefix as an argument.");
        return;
    }

    match args[1].parse::<usize>() {
        Ok(length) => describe_hex_string_prefix(length),
        Err(_) => println!("Please provide a valid length."),
    }
}

fn describe_hex_string_prefix(length: usize) {
    let mut hasher = Sha256::new();

    let mut sentence = String::with_capacity(128);
    sentence.push_str("The SHA256 for this sentence begins with: ");
    let base_len = sentence.len();

    for permutation in NUMS.iter().permutations(length) {
        sentence.truncate(base_len);
        for (index, &digit) in permutation.iter().enumerate() {
            sentence.push_str(WORDS[*digit as usize]);
            if index == length - 2 {
                sentence.push_str(" and ");
            } else if index < length - 2 {
                sentence.push_str(", ")
            }
        }
        sentence.push_str(".");

        hasher.update(&sentence.as_bytes());
        let checksum = hasher.finalize_reset();

        let mut matches = true;
        for i in 0..(length / 2) {
            if (permutation[i * 2] << 4 | permutation[i * 2 + 1] & 0x0f) != checksum[i] {
                matches = false;
                break;
            }
        }
        if matches && length % 2 == 1 {
            if *permutation[length - 1] != checksum[length / 2] >> 4 {
                matches = false;
            }
        }

        if matches {
            println!("{sentence}");
            println!("{:02x}", checksum);
        }
    }
}
