use core::f32;
use std::fmt;
use std::error::Error;
use std::collections::{HashMap};

#[derive(Debug)]
pub enum CryptoError {
	InvalidHexLength,
	InvalidHexDigit,
	BufferLengthMismatch,
	KeyTooLong,
	EmptyKey,
	StartLessThanEnd,
}

impl fmt::Display for CryptoError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			CryptoError::InvalidHexLength => write!(f, "Incorrect hex length"),
			CryptoError::InvalidHexDigit => write!(f, "Incorrect hex digit"),
			CryptoError::BufferLengthMismatch => write!(f, "Buffer lengths need to be the same"),
			CryptoError::KeyTooLong => write!(f, "Key can't be longer than plaintext"),
			CryptoError::EmptyKey => write!(f, "Key can't be empty"),
			CryptoError::StartLessThanEnd => write!(f, "First key candidate needs to be less than the last"),
		}
	}
}

impl Error for CryptoError {}

// From Task 1

static BASE64_TABLE: [char; 64] = [
    'A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P',
    'Q','R','S','T','U','V','W','X','Y','Z',
    'a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p',
    'q','r','s','t','u','v','w','x','y','z',
    '0','1','2','3','4','5','6','7','8','9',
    '+','/',
];

pub fn to_base64(bytes: &[u8]) -> String {
	let mut output = String::new();
	let mut i = 0;
	
	while i < bytes.len() {
		let b0: u8 = bytes[i];
		let b1 = *bytes.get(i + 1).unwrap_or(&0);
		let b2 = *bytes.get(i + 2).unwrap_or(&0);

		let s0: u8 = b0 >> 2;
	        let s1: u8 = ((b0 & 0x03) << 4) | (b1 >> 4);
	        let s2: u8 = ((b1 & 0xf) << 2) | (b2 >> 6);
	        let s3: u8 = b2 & 0x3f;

		output.push(BASE64_TABLE[s0 as usize]);
		output.push(BASE64_TABLE[s1 as usize]);

		match bytes.len() - i {
			1 => {
				output.push('=');
				output.push('=');
			     }
			2 => {
				output.push(BASE64_TABLE[s2 as usize]);
                                output.push('=');
			     }
			_ => {
                                output.push(BASE64_TABLE[s2 as usize]);
                                output.push(BASE64_TABLE[s3 as usize]);
                             }
		}
		i += 3;
	}
	output
}

fn base64_value(c: char) -> Option<u8> {
    match c {
        'A'..='Z' => Some(c as u8 - b'A'),
        'a'..='z' => Some(c as u8 - b'a' + 26),
        '0'..='9' => Some(c as u8 - b'0' + 52),
        '+' => Some(62),
        '/' => Some(63),
        _ => None,
    }
}

pub fn from_base64(input: &str) -> Vec<u8> {
    let mut output = Vec::new();

    let clean: Vec<char> = input
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect();

    let mut i = 0;

    while i < clean.len() {
        let c0 = clean[i];
        let c1 = clean[i + 1];
        let c2 = clean[i + 2];
        let c3 = clean[i + 3];

        let v0 = base64_value(c0).expect("invalid base64");
        let v1 = base64_value(c1).expect("invalid base64");
        let v2 = if c2 == '=' { 0 } else { base64_value(c2).expect("invalid base64") };
        let v3 = if c3 == '=' { 0 } else { base64_value(c3).expect("invalid base64") };

        let b0 = (v0 << 2) | (v1 >> 4);
        output.push(b0);

        if c2 != '=' {
            let b1 = ((v1 & 0x0f) << 4) | (v2 >> 2);
            output.push(b1);
        }

        if c3 != '=' {
            let b2 = ((v2 & 0x03) << 6) | v3;
            output.push(b2);
        }

        i += 4;
    }

    output
}

fn hex_val(b: u8) -> Option<u8> {
	match b {
		b'0'..=b'9' => Some(b - b'0'),
		b'a'..=b'f' => Some(b - b'a' + 10),
		b'A'..=b'F' => Some(b - b'A' + 10),
		_ => None,
	}
}

pub fn hex_to_bytes(input: &str) -> Result<Vec<u8>, CryptoError> {
        if input.len() % 2 != 0{
                return Err(CryptoError::InvalidHexLength);
        }

        let mut result: Vec<u8> = Vec::with_capacity(input.len() / 2);

        for pair in input.as_bytes().chunks(2){
                let hi = hex_val(pair[0]).ok_or(CryptoError::InvalidHexDigit)?;
                let lo = hex_val(pair[1]).ok_or(CryptoError::InvalidHexDigit)?;
                result.push(hi << 4 | lo);
        }
        Ok(result)
}

// From Task 2

pub fn fixed_xor(buf1: &[u8], buf2: &[u8]) -> Result<Vec<u8>, CryptoError>{
	if buf1.len() != buf2.len(){
		return Err(CryptoError::BufferLengthMismatch);
	}
	Ok(buf1.iter().zip(buf2).map(|(x,y)| x ^ y).collect())
}

// From Task 3

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

pub fn get_score(bytes: &[u8]) -> f32 {
	let text = String::from_utf8_lossy(bytes).to_string();
	let chi_squared = chi_squared(&text);
	let non_printable_count = bytes.iter()
        .filter(|&&b| !(b==0x20
             || b==0x21 
             || (b > 0x40 && b < 0x5b) 
             || (b > 0x60 && b < 0x7b)
             || b==0x2e)) //https://www.ascii-code.com/
        .count() as f32;
	chi_squared + non_printable_count * 100.0 //For every non-printable char, give huge penalty
}

pub fn get_key(bytes: Vec<u8>) -> Result<u8, CryptoError> {
    // let mut best_scores: Vec<(f32, String, u8)> = Vec::new();
    let mut best_score: f32 = f32::INFINITY;
    let mut best_key: u8 = 0;
    for key in 0..=255 {
        let key_bytes: Vec<u8> = vec![key as u8; bytes.len()];
        let output = fixed_xor(&bytes, &key_bytes)?;
        let score = get_score(&output);

        // best_scores.push((score, text, key));
        // best_scores.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        // if best_scores.len() > 5 {
        //     best_scores.pop();
        // }
        if score < best_score {
            best_score = score;
            best_key = key;
        }

    }
	Ok(best_key)
}

pub fn get_key_with_text(bytes: Vec<u8>) -> Result<(f32, String, u8), CryptoError> {
    let mut best_score: f32 = f32::INFINITY;
    let mut best_text: String = String::new();
    let mut best_key: u8 = 0;
    for key in 0..=255 {
        let key_bytes: Vec<u8> = vec![key as u8; bytes.len()];
        let output = fixed_xor(&bytes, &key_bytes)?;
        let text = String::from_utf8_lossy(&output).to_string();
        let score = get_score(&output);

        if score < best_score {
            best_score = score;
            best_text = text;
            best_key = key;
        }

    }
	Ok((best_score, best_text, best_key))
}

// From Task 5

// Expects a clean input, i.e. run trim() first
pub fn gen_repeat_key(key: &str, len: usize) -> Result<Vec<u8>, CryptoError>{
	if key.len() == 0 { return Err(CryptoError::EmptyKey); }
	if key.len() > len { return Err(CryptoError::KeyTooLong); }
	Ok(key.as_bytes().iter().cycle().take(len).copied().collect())
}