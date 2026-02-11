use std::io::{self};
use std::collections::{HashMap};

// English letter frequency found at https://pi.math.cornell.edu/~mec/2003-2004/cryptography/subs/frequencies.html
const LETTER_FREQ: [(char, f32); 26] = [('e',12.02),('t',9.1),('a',8.12),('o',7.68),('i',7.31),('n',6.95),('s',6.28),('r',6.02),('h',5.92),('d',4.32),('l',3.98),('u',2.88),('c',2.71),('m',2.61),('f',2.30),('y',2.11),('w',2.09),('g',2.03),('p',1.82),('b',1.49),('v',1.11),('k',0.69),('x',0.17),('q',0.11),('j',0.1),('z',0.07)];

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

fn get_freq(cipher: &[u8]) -> Vec<(char,f32)> {
	let mut counts = HashMap::new();
	for &byte in cipher {
		*counts.entry(byte).or_insert(0) += 1;
	}
	counts.into_iter().map(|(x,y)|(x as char, y as f32/cipher.len() as f32 *100.0)).collect()
}

fn get_score(freq_table: Vec<(char, f32)>) -> f32{
	let mut score = 0.0;
	for (value, freq) in freq_table {
		let mut found = false;
		if value == '=' {
			score += 5.0;
			continue;
		}
		for (letter, eng_freq) in LETTER_FREQ {
			if value == letter {
				score += (eng_freq - (eng_freq - freq).abs()).max(0.0);
				found = true;
				break;
			}
		}
		if !found {
			score -= 3.0;
		}
	}
	score
}
	
fn main() -> io::Result<()>{
        let stdin = io::stdin();
        //let mut stdout = io::stdout();
        let mut input = String::new();

        stdin.read_line(&mut input)?;
	
	let bytes = hex_to_bytes(&input.trim()).unwrap();
	let mut best_scores: Vec<(f32, String, u8)> = Vec::new();
	for key in 0..=255{
		let key_bytes: Vec<u8> = vec![key as u8; bytes.len()];
		let output = fixed_xor(&bytes, &key_bytes).unwrap();
		let freq = get_freq(&output);
		let score = get_score(freq);
		let text = String::from_utf8_lossy(&output).to_string();

		best_scores.push((score, text, key));
        	best_scores.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
        	if best_scores.len() > 5 {
            		best_scores.pop();
        	}	
	}
	for (score, text, key) in best_scores.iter(){
		println!("Score: {}, Text: {}, Key: {:x}", score, text, key);
	}
	Ok(())
}
	
