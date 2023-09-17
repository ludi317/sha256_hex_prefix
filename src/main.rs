use itertools::Itertools;
use sha2::{Digest, Sha256};
use std::env;
use std::io;
use std::time::Instant;


const NUMS: [u8; 16] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
const WORDS: [&str; 16] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "a", "b", "c",
    "d", "e", "f",
];
const SENTENCE_PREFIX: &str = "The SHA256 for this sentence begins with: ";
const SENTENCE_PREFIX_LEN: usize = SENTENCE_PREFIX.len();

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Give the length of the hex string prefix as an argument.");
        return;
    }
    let start = Instant::now();
    match args[1].parse::<usize>() {
        Ok(length) => describe_hex_string_prefix(length, &mut io::stdout()), // 120.7 secs
        Err(_) => println!("Please provide a valid length."),
    }
    println!("Time elapsed is: {:.1} seconds", start.elapsed().as_secs_f32());
}

fn describe_hex_string_prefix<W: io::Write>(length: usize, writer: &mut W) {

    let mut hasher = Sha256::new();

    let mut sentence = String::with_capacity(SENTENCE_PREFIX_LEN + (length - 1) * "seven, ".len() + "and seven.".len() - 1);
    sentence.push_str(SENTENCE_PREFIX);

    for combo in (0..length).map(|_| NUMS.iter()).multi_cartesian_product() {
        sentence.truncate(SENTENCE_PREFIX_LEN);
        for (index, &digit) in combo.iter().enumerate() {
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
            if (combo[i * 2] << 4 | combo[i * 2 + 1] & 0x0f) != checksum[i] {
                matches = false;
                break;
            }
        }
        if matches && length % 2 == 1 {
            if *combo[length - 1] != checksum[length / 2] >> 4 {
                matches = false;
            }
        }

        if matches {
            writeln!(writer, "{sentence}").unwrap();
            writeln!(writer, "{:02x}", checksum).unwrap();
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet() {
        let mut buffer = Vec::new();

        describe_hex_string_prefix(4, &mut buffer);

        assert_eq!(buffer, b"The SHA256 for this sentence begins with: zero, b, six and two.
0b62c3f1bf41205f419fd37a3e028e65a054d6ba913b10711ef53073e9096c85\n");
    }
}
