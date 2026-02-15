use std::fs;
use std::io::{self, Read};
use std::fmt;
use std::error::Error;

#[derive(Debug)]
enum CryptoError {
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

fn hex_val(b: u8) -> Option<u8> {
        match b {
                b'0'..=b'9' => Some(b - b'0'),
                b'a'..=b'f' => Some(b - b'a' + 10),
                b'A'..=b'F' => Some(b - b'A' + 10),
                _ => None,
        }
}

fn hex_to_bytes(input: &str) -> Result<Vec<u8>, CryptoError> {
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

fn fixed_xor(buf1: &[u8], buf2: &[u8]) -> Result<Vec<u8>, CryptoError>{
	if buf1.len() != buf2.len(){
		return Err(CryptoError::BufferLengthMismatch);
	}
	Ok(buf1.iter().zip(buf2).map(|(x,y)| x ^ y).collect())
}

// Expects a clean input, i.e. run trim() first
fn gen_repeat_key(key: &str, len: usize) -> Result<Vec<u8>, CryptoError>{
	 println!("DEBUG: Key length: {}, Plaintext length: {}", key.len(), len);
	println!("DEBUG: Key bytes: {:?}", key.as_bytes());
	if key.len() == 0 { return Err(CryptoError::EmptyKey); }
	if key.len() > len { return Err(CryptoError::KeyTooLong); }
	Ok(key.as_bytes().iter().cycle().take(len).copied().collect())
}

fn hamming_distance(n1: &[u8], n2: &[u8]) -> usize {
	n1.iter().zip(n2.iter())
	.map(|(x,y)| (x ^ y).count_ones() as usize).sum()
}

fn find_key_size(start: usize, end: usize, bytes: &[u8]) -> Result<Option<usize>, CryptoError> {
	if end < start { return Err(CryptoError::StartLessThanEnd); }

	let mut best_edit_dist = f64::MAX;
	let mut best_block_size = None;
	
	// Compares average of all distances between adjacent blocks
	for block_size in start..=end {
		let trimmed_size = bytes.len() / block_size * block_size; // Remove excess
		let blocks: Vec<&[u8]> = bytes[..trimmed_size].chunks(block_size).collect();
		let mut distances: Vec<f64> = Vec::new();
		if blocks.len() < 2 { continue; }; // Not break; since excess might lead to 2 blocks again

		for i in 0..blocks.len()-1{
			distances.push(hamming_distance(&blocks[i], &blocks[i+1]) as f64);
		}

		if distances.is_empty(){ continue; }
		let avg = (distances.iter().sum::<f64>()) / ((distances.len() * block_size) as f64);
		println!("Block size: {}, Avg: {}", block_size, avg);
		if avg < best_edit_dist {
			best_edit_dist = avg;
			best_block_size = Some(block_size);
		}
	} 
	Ok(best_block_size)
}
		
fn main() -> Result<(), Box<dyn Error>>{

	let plaintext = fs::read_to_string("Task6.txt")?;
	let plaintext = plaintext.trim().as_bytes();

	let mut key = String::new();
	io::stdin().read_line(&mut key);
	let key = key.trim();
	let repeated_key = gen_repeat_key(&key, plaintext.len())?;
	let output = fixed_xor(&plaintext, &repeated_key)?;
	let length = find_key_size(2,40, &output)?;
	println!("{:?}", length);
	/*
	for b in output {
		print!("{:02x}", b);
	}*/	
	Ok(())
}
	
