use std::fs;
use std::io;
use std::fmt;
use std::error::Error;

#[derive(Debug)]
enum CryptoError {
	InvalidHexLength,
	InvalidHexDigit,
	BufferLengthMismatch,
	KeyTooLong,
	EmptyKey,
}

impl fmt::Display for CryptoError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			CryptoError::InvalidHexLength => write!(f, "Incorrect hex length"),
			CryptoError::InvalidHexDigit => write!(f, "Incorrect hex digit"),
			CryptoError::BufferLengthMismatch => write!(f, "Buffer lengths need to be the same"),
			CryptoError::KeyTooLong => write!(f, "Key can't be longer than plaintext"),
			CryptoError::EmptyKey => write!(f, "Key can't be empty"),
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

fn gen_repeat_key(key: String, len: usize) -> Result<Vec<u8>, CryptoError>{
	let key = key.trim();
	if key.len() == 0 { return Err(CryptoError::EmptyKey); }
	if key.trim().len() > len { return Err(CryptoError::KeyTooLong); }
	Ok(key.as_bytes().iter().cycle().take(len).copied().collect())
}

fn main() -> Result<(), Box<dyn Error>>{
	let plaintext = fs::read_to_string("Task5.txt")?;
	let plaintext = plaintext.trim().as_bytes();
	
	println!("Key:");
	let mut key = String::new();
        let stdin = io::stdin();
	stdin.read_line(&mut key)?;
	
	let repeated_key = gen_repeat_key(key, plaintext.len())?;
	let output = fixed_xor(&plaintext, &repeated_key)?;
	for b in output {
		print!("{:02x}", b);
	}	
	Ok(())
}
	
