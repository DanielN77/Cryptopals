use std::fs;
use std::io;

fn hex_val(b: u8) -> Option<u8> {
        match b {
                b'0'..=b'9' => Some(b - b'0'),
                b'a'..=b'f' => Some(b - b'a' + 10),
                b'A'..=b'F' => Some(b - b'A' + 10),
                _ => None,
        }
}

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

fn fixed_xor(buf1: &[u8], buf2: &[u8]) -> Result<Vec<u8>, &'static str>{
	if buf1.len() != buf2.len(){
		return Err("Buffers need to be of the same size");
	}
	Ok(buf1.iter().zip(buf2).map(|(x,y)| x ^ y).collect())
}

fn gen_repeat_key(key: String, len: usize) -> Result<Vec<u8>, &'static str>{
	if key.trim().len() > len {
		return Err("Repeated key generation error: Input key length needs to be less than plaintext length");
	}
	let mut repeater = key.trim().as_bytes().iter().cycle();
	let mut repeated_key: Vec<u8> = Vec::new();
	for _ in 0..len{
		repeated_key.push(*repeater.next().expect("Repeated key generation error"));
	}
	Ok(repeated_key)
}

fn main() -> io::Result<()>{
        let stdin = io::stdin();
	let plaintext = fs::read_to_string("Set1Task5.txt")?;
	let mut key = String::new();
	
	let plaintext = plaintext.trim().as_bytes();
	println!("Key:");
	stdin.read_line(&mut key)?;
	
	let repeated_key = gen_repeat_key(key, plaintext.len()).unwrap();
	let output = fixed_xor(&plaintext, &repeated_key).unwrap();
	for b in output {
		print!("{:02x}", b);
	}	
	Ok(())
}
	
