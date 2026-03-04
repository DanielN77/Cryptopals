static BASE64_TABLE: [char; 64] = [
    'A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P',
    'Q','R','S','T','U','V','W','X','Y','Z',
    'a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p',
    'q','r','s','t','u','v','w','x','y','z',
    '0','1','2','3','4','5','6','7','8','9',
    '+','/',
];

pub fn to_base64(bytes: &[u8]) -> String {
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