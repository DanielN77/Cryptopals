use std::io::{self};
use std::fs;
use std::error::Error;
use crate::lib::{fixed_xor, gen_repeat_key, from_base64, get_key, get_score, CryptoError};

fn hamming_distance(n1: &[u8], n2: &[u8]) -> usize {
	n1.iter().zip(n2.iter())
	.map(|(x,y)| (x ^ y).count_ones() as usize).sum()
}

fn find_key_sizes(start: usize, end: usize, bytes: &[u8]) -> Result<Vec<usize>, CryptoError> {
    if end < start {
        return Err(CryptoError::StartLessThanEnd);
    }

    let mut candidates: Vec<(f64, usize)> = Vec::new();

    for key_size in start..=end {
        let trimmed_size = bytes.len() / key_size * key_size;
        if trimmed_size == 0 {
            continue;
        }
        let blocks: Vec<&[u8]> = bytes[..trimmed_size].chunks(key_size).collect();
        let num_blocks = 16.min(blocks.len());
        if num_blocks < 2 {
            continue;
        }

        let mut total_dist = 0.0;
        for i in 0..num_blocks - 1 {
            for j in i + 1..num_blocks {
                total_dist += hamming_distance(blocks[i], blocks[j]) as f64;
            }
        }

        let num_pairs = (num_blocks * (num_blocks - 1) / 2) as f64;
        let avg_per_byte = total_dist / (num_pairs * key_size as f64);
		candidates.push((avg_per_byte, key_size));
        // if avg_per_byte < best_edit_dist {
        //     best_edit_dist = avg_per_byte;
        //     best_key_size = Some(key_size);
        // }
    }
	candidates.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    Ok(candidates.into_iter().take(10).map(|(_, size)| size).collect())
}

fn transpose_cipher(key_size: usize, bytes: &[u8]) -> Option<Vec<Vec<u8>>> {
	if key_size == 0 { return None;}
	Some((0..key_size).map(|i| {
		bytes.iter().skip(i).step_by(key_size).copied().collect()
	}).collect())
}

fn repeat_bytes(key: &[u8], len: usize) -> Vec<u8> {
    key.iter().cycle().take(len).copied().collect()
}
		
pub fn run() -> Result<(), Box<dyn Error>> {
    let base64 = fs::read_to_string("test_data/task6.txt")?;
    let cipher = from_base64(base64.trim());

    let top_key_sizes = find_key_sizes(2, 40, &cipher)?;
    eprintln!("Top keys: {:?}", top_key_sizes);

    let mut best_score = f32::INFINITY;
    let mut best_plaintext = String::new();
    let mut best_key = Vec::new();

    for &key_size in &top_key_sizes {
        let transposed = transpose_cipher(key_size, &cipher).unwrap();

        let mut key_bytes = Vec::with_capacity(key_size);
        let mut valid = true;
        for block in transposed {
            match get_key(block) {
                Ok(k) => key_bytes.push(k),
                Err(_) => {
                    continue;
                }
            }
        }
        if key_bytes.len() != key_size {
            continue;
        }

        let repeated = repeat_bytes(&key_bytes, cipher.len());
        let plain = fixed_xor(&cipher, &repeated)?;
        let score = get_score(&plain);

        if score < best_score {
            best_score = score;
            best_plaintext = String::from_utf8_lossy(&plain).to_string();
            best_key = key_bytes;
        }
    }

	println!("Key: {:?}", String::from_utf8_lossy(&best_key));
	println!("Decrypted text:\n{}", best_plaintext);
    Ok(())
}