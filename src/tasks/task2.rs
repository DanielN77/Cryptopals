use std::io::{self};
use crate::lib::{hex_to_bytes, CryptoError};
use std::error::Error;

fn fixed_xor(buf1: &[u8], buf2: &[u8]) -> Result<Vec<u8>, CryptoError>{
	if buf1.len() != buf2.len(){
		return Err(CryptoError::BufferLengthMismatch);
	}
	Ok(buf1.iter().zip(buf2).map(|(x,y)| x ^ y).collect())
}

pub fn run() -> Result<(), Box<dyn Error>> {
	let stdin = io::stdin();
	let mut input1 = String::new();
	let mut input2 = String::new();

	println!("First number:");
    stdin.read_line(&mut input1)?;
	println!("Second number:");
	stdin.read_line(&mut input2)?;
	
	let bytes1 = hex_to_bytes(&input1.trim())?;
	let bytes2 = hex_to_bytes(&input2.trim())?;
	
	let output = fixed_xor(&bytes1, &bytes2)?;
	for b in output {
		print!("{:02x}", b);
	}	
	Ok(())
}
	
