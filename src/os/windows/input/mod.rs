// Copyright Jeron A. Lau 2017-2018.
// Dual-licensed under either the MIT License or the Boost Software License,
// Version 1.0.  (See accompanying file LICENSE_1_0.txt or copy at
// https://www.boost.org/LICENSE_1_0.txt)

pub mod key {
	use winapi::ctypes::c_int;

	pub const A: c_int = 65;
	pub const B: c_int = 66;
	pub const C: c_int = 67;
	pub const D: c_int = 68;
	pub const E: c_int = 69;
	pub const F: c_int = 70;
	pub const G: c_int = 71;
	pub const H: c_int = 72;
	pub const I: c_int = 73;
	pub const J: c_int = 74;
	pub const K: c_int = 75;
	pub const L: c_int = 76;
	pub const M: c_int = 77;
	pub const N: c_int = 78;
	pub const O: c_int = 79;
	pub const P: c_int = 80;
	pub const Q: c_int = 81;
	pub const R: c_int = 82;
	pub const S: c_int = 83;
	pub const T: c_int = 84;
	pub const U: c_int = 85;
	pub const V: c_int = 86;
	pub const W: c_int = 87;
	pub const X: c_int = 88;
	pub const Y: c_int = 89;
	pub const Z: c_int = 90;
	pub const NUM_1: c_int = 49;
	pub const NUM_2: c_int = 50;
	pub const NUM_3: c_int = 51;
	pub const NUM_4: c_int = 52;
	pub const NUM_5: c_int = 53;
	pub const NUM_6: c_int = 54;
	pub const NUM_7: c_int = 55;
	pub const NUM_8: c_int = 56;
	pub const NUM_9: c_int = 57;
	pub const NUM_0: c_int = 48;
	pub const MINUS: c_int = 189;
	pub const EQUAL_SIGN: c_int = 187;
	pub const BACKSPACE: c_int = 8;
	pub const TAB: c_int = 9;
	pub const BRACKET_OPEN: c_int = 219;
	pub const BRACKET_CLOSE: c_int = 221;
	pub const ENTER: c_int = 13;
	pub const LEFT_CTRL: c_int = 17;
	pub const RIGHT_CTRL: c_int = 17 | (0b_1_0001_1101 << 16);
	pub const SEMICOLON: c_int = 186;
	pub const APOSTROPHE: c_int = 222;
	pub const LEFT_SHIFT: c_int = 16;
	pub const RIGHT_SHIFT: c_int = 16 | (0b_0011_0110 << 16);
	pub const BACKSLASH: c_int = 220;
	pub const COMMA: c_int = 188;
	pub const PERIOD: c_int = 190;
	pub const LEFT_ALT: c_int = 18;
	pub const CAPS_LOCK: c_int = 20;
	pub const SPACE: c_int = 32;
	pub const SLASH: c_int = 111;
	pub const UP: c_int = 38;
	pub const DOWN: c_int = 40;
	pub const LEFT: c_int = 37;
	pub const RIGHT: c_int = 39;

	pub mod ext {
		use winapi::ctypes::c_int;

		pub const NUM_PAD_1: c_int = 97;
		pub const NUM_PAD_2: c_int = 98;
		pub const NUM_PAD_3: c_int = 99;
		pub const NUM_PAD_4: c_int = 100;
		pub const NUM_PAD_5: c_int = 101;
		pub const NUM_PAD_6: c_int = 102;
		pub const NUM_PAD_7: c_int = 103;
		pub const NUM_PAD_8: c_int = 104;
		pub const NUM_PAD_9: c_int = 105;
		pub const NUM_PAD_0: c_int = 96;
		pub const NUM_PAD_MINUS: c_int = 109;
		pub const NUM_PAD_ENTER: c_int = 13;
		pub const NUM_PAD_PERIOD: c_int = 110;
		pub const NUM_PAD_ASTERISK: c_int = 106;
		pub const NUM_PAD_PLUS: c_int = 107;
		pub const NUM_PAD_SLASH: c_int = 191;
		pub const BACKTICK: c_int = 192;
		pub const NUMLOCK: c_int = 144;
		pub const ALT_GR: c_int = 18 | (0b_1_0011_1000 << 16);
		#[allow(unused)]
		pub const FULLSCREEN: c_int = 122;
		pub const HOME: c_int = 36;
		pub const END: c_int = 35;
		pub const PAGE_UP: c_int = 33;
		pub const PAGE_DOWN: c_int = 34;
		pub const INSERT: c_int = 45;
		pub const DELETE: c_int = 46;
	}

	pub mod lib {
		use winapi::ctypes::c_int;

		pub const ESCAPE: c_int = 27;
	}
}
