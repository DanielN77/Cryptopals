use std::io::{self, Read, Write};
use Cryptopals::to_base64;

fn hex_val(b: u8) -> Option<u8> {
	match b {
		b'0'..=b'9' => Some(b - b'0'),
		b'a'..=b'f' => Some(b - b'a' + 10),
		b'A'..=b'F' => Some(b - b'A' + 10),
		_ => None,
	}
}

// Converts a string representation of hex values to the actual values
fn hex_to_bytes(input: &str) -> Result<Vec<u8>, &'static str> {
	if input.len() % 2 != 0{
		return Err("Incorrect Hex Length");
	}
	
	let mut result: Vec<u8> = Vec::with_capacity(input.len() / 2);
	
	for pair in input.as_bytes().chunks(2){
		let hi = hex_val(pair[0]).ok_or("Bad hex")?;
		let lo = hex_val(pair[1]).ok_or("Bad hex")?;
		result.push(hi << 4 | lo);
	}
	Ok(result)
}
	
fn main() -> io::Result<()>{
	let mut stdin = io::stdin();
	let mut stdout = io::stdout();
	
	let mut input = String::new();
	stdin.read_to_string(&mut input)?;
	let hexed = hex_to_bytes(&input).unwrap();
	let output = to_base64(&hexed);
	
	stdout.write_all(output.as_bytes())?;	
	Ok(())
}
