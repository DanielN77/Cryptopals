use std::io::{BufRead, BufReader};
use std::fs::File;
use crate::lib::{hex_to_bytes, get_key_with_text};
use std::error::Error;

pub fn run() -> Result<(), Box<dyn Error>> {
    let reader = BufReader::new(File::open("test_data/task4.txt").expect("Can't open text file"));
	let mut best_score: (f32, String, u8) = (f32::INFINITY, String::new(), 0);

	for line in reader.lines() {
		let bytes = hex_to_bytes(&line?.trim()).unwrap();
		let good_score = get_key_with_text(bytes)?;
		if good_score.0 < best_score.0 {
			best_score = good_score;
		}
	}
	println!("Text: {}, Key: {}, Score: {}", best_score.1, best_score.2, best_score.0);
	Ok(())
}
	
