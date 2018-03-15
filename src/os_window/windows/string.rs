// os_window/windows/string.rs -- Aldaron's Window Interface
// Copyright (c) 2017-2018  Jeron A. Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE

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
