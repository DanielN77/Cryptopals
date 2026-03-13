use openssl::symm::{decrypt, Cipher};
use crate::lib::{from_base64};
use std::error::Error;

pub fn run() -> Result<(), Box<dyn Error>> {
    let key = b"YELLOW SUBMARINE";

    let base64 = std::fs::read_to_string("test_data/task7.txt")?;
    let ciphertext = from_base64(&base64.replace('\n', ""));

    let plaintext = decrypt(
        Cipher::aes_128_ecb(),
        key,
        None,  
        &ciphertext
    ).unwrap();

    println!("{}", String::from_utf8_lossy(&plaintext));
    Ok(())
}