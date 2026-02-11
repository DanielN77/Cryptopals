use std::io::{self, Read, Write};

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

fn main() -> io::Result<()>{
        let stdin = io::stdin();
        //let mut stdout = io::stdout();

        let mut input1 = String::new();
	let mut input2 = String::new();

	println!("First number:");
        stdin.read_line(&mut input1)?;
	println!("Second number:");
	stdin.read_line(&mut input2)?;
	
	let bytes1 = hex_to_bytes(&input1.trim()).unwrap();
	let bytes2 = hex_to_bytes(&input2.trim()).unwrap();
	
	let output = fixed_xor(&bytes1, &bytes2).unwrap();
	for b in output {
		print!("{:02x}", b);
	}	
	Ok(())
}
	
