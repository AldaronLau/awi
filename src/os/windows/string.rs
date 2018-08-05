// Copyright Jeron A. Lau 2017-2018.
// Dual-licensed under either the MIT License or the Boost Software License,
// Version 1.0.  (See accompanying file LICENSE_1_0.txt or copy at
// https://www.boost.org/LICENSE_1_0.txt)

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
