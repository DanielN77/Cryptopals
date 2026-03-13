use std::fs;
use std::io;
use std::error::Error;
use crate::lib::{fixed_xor, CryptoError};

fn gen_repeat_key(key: String, len: usize) -> Result<Vec<u8>, CryptoError>{
	let key = key.trim();
	if key.len() == 0 { return Err(CryptoError::EmptyKey); }
	if key.trim().len() > len { return Err(CryptoError::KeyTooLong); }
	Ok(key.as_bytes().iter().cycle().take(len).copied().collect())
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let plaintext = fs::read_to_string("test_data/task5.txt")?;
    let plaintext = plaintext.trim().as_bytes();
    
    println!("Key:");
    let mut key = String::new();
    io::stdin().read_line(&mut key)?;
    let key: &str = key.trim();
    
    let repeated_key = gen_repeat_key(key.to_string(), plaintext.len())?;
    let output = fixed_xor(plaintext, &repeated_key)?;
    
    for b in output {
        print!("{:02x}", b);
    }    
    Ok(())
}
	
