use std::io::{self, Read, Write};
use crate::lib::{CryptoError};
use std::error::Error;

static BASE64_TABLE: [char; 64] = [
    'A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P',
    'Q','R','S','T','U','V','W','X','Y','Z',
    'a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p',
    'q','r','s','t','u','v','w','x','y','z',
    '0','1','2','3','4','5','6','7','8','9',
    '+','/',
];

fn to_base64(bytes: &[u8]) -> String {
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

pub fn run() -> Result<(), Box<dyn Error>> {
	let mut stdin = io::stdin();
	let mut stdout = io::stdout();
	
	let mut input = String::new();
	stdin.read_to_string(&mut input)?;
	let input = input.trim();
	let hexed = hex_to_bytes(&input)?;
	let output = to_base64(&hexed);
	
	stdout.write_all(output.as_bytes())?;	
	Ok(())
}