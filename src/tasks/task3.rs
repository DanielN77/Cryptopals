use std::io::{self};
use std::collections::{HashMap};
use crate::lib::{hex_to_bytes, fixed_xor, CryptoError};
use std::error::Error;

// From https://norvig.com/mayzner.html
const ENGLISH_LETTER_FREQS: [(char, f32); 26] = [
    ('E', 12.49), ('T', 9.28), ('A', 8.04), ('O', 7.64), ('I', 7.57),
    ('N', 7.23), ('S', 6.51), ('R', 6.28), ('H', 5.05), ('L', 4.07),
    ('D', 3.82), ('C', 3.34), ('U', 2.73), ('M', 2.51), ('F', 2.40),
    ('P', 2.14), ('G', 1.87), ('W', 1.68), ('Y', 1.66), ('B', 1.48),
    ('V', 1.05), ('K', 0.54), ('X', 0.23), ('J', 0.16), ('Q', 0.12),
    ('Z', 0.09),
];

fn chi_squared(text: &str) -> f32 {
    let text = text.to_ascii_uppercase();
    let mut observed_counts: HashMap<char, usize> = HashMap::new();

    for c in text.chars() {
        if c.is_ascii_alphabetic() {
            *observed_counts.entry(c).or_insert(0) += 1;
        }
    }

    let total_letters = observed_counts.values().sum::<usize>() as f32;
    if total_letters == 0.0 {
        return f32::INFINITY;
    }

    let mut chi_squared = 0.0;
    for (letter, expected_freq) in ENGLISH_LETTER_FREQS.iter() {
        let observed_count = *observed_counts.get(letter).unwrap_or(&0) as f32;
        let expected_count = (expected_freq / 100.0) * total_letters;

        if expected_count > 0.0 {
            let diff = observed_count - expected_count;
            chi_squared += (diff * diff) / expected_count;
        }
    }
    chi_squared
}

fn get_score(bytes: &[u8]) -> f32 {
	let text = String::from_utf8_lossy(bytes).to_string();
	let chi_squared = chi_squared(&text);
	let non_printable_count = bytes.iter()
        .filter(|&&b| b < 0x20 || b > 0x7F) //https://www.ascii-code.com/
        .count() as f32;
	chi_squared + non_printable_count * 100.0 //For every non-printable char, give huge penalty
}

fn get_key_with_text(bytes: Vec<u8>) -> Result<Vec<(f32, String, u8)>, CryptoError> {
    let mut best_scores: Vec<(f32, String, u8)> = Vec::new();
    for key in 0..=255 {
        let key_bytes: Vec<u8> = vec![key as u8; bytes.len()];
        let output = fixed_xor(&bytes, &key_bytes)?;
        let text = String::from_utf8_lossy(&output).to_string();
        let score = get_score(&output);

        best_scores.push((score, text, key));
        best_scores.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        if best_scores.len() > 5 {
            best_scores.pop();
        }

    }
	Ok(best_scores)
}
	
pub fn run() -> Result<(), Box<dyn Error>> {
	let stdin = io::stdin();
	let mut input = String::new();
	stdin.read_line(&mut input)?;
	
	let bytes = hex_to_bytes(&input.trim())?;
	let best_scores = get_key_with_text(bytes)?;
    for (score, text, key) in best_scores.iter() {
        println!("Score: {:.2}, Key: 0x{:02X} ('{}'), Text: {}", score, key, *key as char, text);
    }

    Ok(())
}
	
