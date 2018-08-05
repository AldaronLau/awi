// Copyright Jeron A. Lau 2017-2018.
// Dual-licensed under either the MIT License or the Boost Software License,
// Version 1.0.  (See accompanying file LICENSE_1_0.txt or copy at
// https://www.boost.org/LICENSE_1_0.txt)

pub mod modifiers;

const BIT64 : u64 =
	0b1000000000000000000000000000000000000000000000000000000000000000u64;
const BIT16 : u16 = 0b1000000000000000u16;

pub(crate) const NUM1: u8 = 0;
pub(crate) const NUM2: u8 = 1;
pub(crate) const NUM3: u8 = 2;
pub(crate) const NUM4: u8 = 3;
pub(crate) const NUM5: u8 = 4;
pub(crate) const NUM6: u8 = 5;
pub(crate) const NUM7: u8 = 6;
pub(crate) const NUM8: u8 = 7;
pub(crate) const NUM9: u8 = 8;
pub(crate) const NUM0: u8 = 9;
pub(crate) const MINUS: u8 = 10;
pub(crate) const EQUAL_SIGN: u8 = 11;
pub(crate) const BACKSPACE: u8 = 12;
pub(crate) const TAB: u8 = 13;
pub(crate) const Q: u8 = 14;
pub(crate) const W: u8 = 15;
pub(crate) const E: u8 = 16;
pub(crate) const R: u8 = 17;
pub(crate) const T: u8 = 18;
pub(crate) const Y: u8 = 19;
pub(crate) const U: u8 = 20;
pub(crate) const I: u8 = 21;
pub(crate) const O: u8 = 22;
pub(crate) const P: u8 = 23;
pub(crate) const BRACKET_OPEN: u8 = 24;
pub(crate) const BRACKET_CLOSE: u8 = 25;
pub(crate) const BACKSLASH: u8 = 26;
pub(crate) const COMPOSE: u8 = 27;
pub(crate) const A: u8 = 28;
pub(crate) const S: u8 = 29;
pub(crate) const D: u8 = 30;
pub(crate) const F: u8 = 31;
pub(crate) const G: u8 = 32;
pub(crate) const H: u8 = 33;
pub(crate) const J: u8 = 34;
pub(crate) const K: u8 = 35;
pub(crate) const L: u8 = 36;
pub(crate) const SEMICOLON: u8 = 37;
pub(crate) const APOSTROPHE: u8 = 38;
pub(crate) const ENTER: u8 = 39;
pub(crate) const LSHIFT: u8 = 40;
pub(crate) const Z: u8 = 41;
pub(crate) const X: u8 = 42;
pub(crate) const C: u8 = 43;
pub(crate) const V: u8 = 44;
pub(crate) const B: u8 = 45;
pub(crate) const N: u8 = 46;
pub(crate) const M: u8 = 47;
pub(crate) const COMMA: u8 = 48;
pub(crate) const PERIOD: u8 = 49;
pub(crate) const SLASH: u8 = 50;
pub(crate) const RSHIFT: u8 = 51;
pub(crate) const LCTRL: u8 = 52;
pub(crate) const ALT: u8 = 53;
pub(crate) const SPACE: u8 = 54;
pub(crate) const RCTRL: u8 = 55;
pub(crate) const UP: u8 = 56;
pub(crate) const DOWN: u8 = 57;
pub(crate) const LEFT: u8 = 58;
pub(crate) const RIGHT: u8 = 59;

pub(crate) const EXT_BACKTICK: u8 = 64;
pub(crate) const EXT_DELETE: u8 = 65;
pub(crate) const EXT_INSERT: u8 = 66;
pub(crate) const EXT_NUM_LOCK: u8 = 67;
pub(crate) const EXT_PAGE_UP: u8 = 68;
pub(crate) const EXT_PAGE_DOWN: u8 = 69;
pub(crate) const EXT_HOME: u8 = 70;
pub(crate) const EXT_END: u8 = 71;
pub(crate) const EXT_ASTERISK: u8 = 72;
pub(crate) const EXT_PLUS: u8 = 73;
pub(crate) const EXT_ALT_GR: u8 = 74;

/// A Computer keyboard.
pub struct Keyboard {
	keys: u64,
	exts: u16,
	keyo: u64,
	exto: u16,
}

impl Keyboard {
	/// Create a keyboard.
	pub fn new() -> Keyboard {
		Keyboard { keys: 0u64, exts: 0u16, keyo: 0u64, exto: 0u16 }
	}

	/// Press a key.
	pub fn press(&mut self, key: u8) {
		if key < 64 {
			self.keys |= BIT64 >> key as usize;
		} else {
			self.exts |= BIT16 >> (key as usize - 64);
		}
	}

	/// Release a key.
	pub fn release(&mut self, key: u8) {
		if key < 64 {
			self.keys &= !(BIT64 >> key as usize);
		} else {
			self.exts &= !(BIT16 >> (key as usize - 64));
		}
	}

	/// Get whether a key is pressed (`true`) or not (`false`).
	pub fn get(&mut self, key: u8) -> bool {
		if key < 64 {
			(self.keys & (BIT64 >> key as usize)) != 0
		} else {
			(self.exts & (BIT16 >> (key as usize - 64))) != 0
		}
	}

	/// Add keyboard input to the input queue.
	pub fn add(&mut self, queue: &mut ::input::InputQueue) {
		let variants = [
			NUM1, NUM2, NUM3, NUM4, NUM5, NUM6, NUM7, NUM8, NUM9,
			NUM0, MINUS, EQUAL_SIGN, BACKSPACE,
			TAB, Q, W, E, R, T, Y, U, I, O, P, BRACKET_OPEN,
			BRACKET_CLOSE, BACKSLASH,
			COMPOSE, A, S, D, F, G, H, J, K, L, SEMICOLON,
			APOSTROPHE, ENTER,
			LSHIFT, Z, X, C, V, B, N, M, COMMA, PERIOD, SLASH,
			RSHIFT,
			LCTRL, ALT, SPACE, RCTRL, UP, DOWN,
			LEFT, RIGHT,
			EXT_BACKTICK, EXT_DELETE, EXT_INSERT, EXT_NUM_LOCK,
			EXT_PAGE_UP, EXT_PAGE_DOWN, EXT_HOME, EXT_END,
			EXT_ASTERISK, EXT_PLUS, EXT_ALT_GR
		];

		for v in variants.iter() {
			self.a(queue, *v);
		}

		// Set old keyboard state.
		self.keyo = self.keys;
		self.exto = self.exts;
	}

	fn a(&mut self, queue: &mut ::input::InputQueue, key: u8) {
		if key < 64 {
			if (self.keys & (BIT64 >> key as usize)) != 0 {
				queue.key(key, Some(
					(self.keyo & (BIT64 >> key as usize))==0
				));
			} else {
				if (self.keyo & (BIT64 >> key as usize)) != 0 {
					queue.key(key, None);
				}
			}
		} else {
			if (self.exts & (BIT16 >> (key as usize - 64))) != 0 {
				queue.key(key, Some(
					(self.exto & (BIT16 >> (key as usize - 64)))
						== 0
				));
			} else {
				if (self.exto & (BIT16 >> (key as usize - 64)))
					!= 0
				{
					queue.key(key, None);
				}
			}
		}
	}
}
