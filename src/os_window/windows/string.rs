// "awi" crate - Licensed under the MIT LICENSE
//  * Copyright (c) 2017-2018  Jeron A. Lau <jeron.lau@plopgrizzly.com>

// Windows uses UTF-16
pub fn native(what: &str) -> Vec<u8> {
	let mut rtn : Vec<u8> = Vec::new();
	for c in what.encode_utf16() {
		rtn.push((c % 255) as u8);
		rtn.push((c / 255) as u8);
	}
	rtn.push(0);
	rtn.push(0);
	rtn
}
