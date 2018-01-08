// Aldaron's Window Interface
// Copyright (c) 2017 Jeron Aldaron Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE
//
// src/input/keyboard/mod.rs

pub mod modifiers;
mod key;
pub mod msg;

pub use self::key::Key;

const BIT64 : u64 =
	0b1000000000000000000000000000000000000000000000000000000000000000u64;
const BIT16 : u16 = 0b1000000000000000u16;

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
	pub fn press(&mut self, key: Key) {
		if (key as u8) < 64 {
			self.keys |= BIT64 >> key as usize;
		} else {
			self.exts |= BIT16 >> (key as usize - 64);
		}
	}

	/// Release a key.
	pub fn release(&mut self, key: Key) {
		if (key as u8) < 64 {
			self.keys &= !(BIT64 >> key as usize);
		} else {
			self.exts &= !(BIT16 >> (key as usize - 64));
		}
	}

	/// Get whether a key is pressed (`true`) or not (`false`).
	pub fn get(&mut self, key: Key) -> bool {
		if (key as u8) < 64 {
			(self.keys & (BIT64 >> key as usize)) != 0
		} else {
			(self.exts & (BIT16 >> (key as usize - 64))) != 0
		}
	}

	/// Add keyboard input to the input queue.
	pub fn add(&mut self, queue: &mut ::input::InputQueue) {
		use Key::*;

		let variants = [
			Num1, Num2, Num3, Num4, Num5, Num6, Num7, Num8, Num9,
			Num0, Minus, EqualSign, Backspace,
			Tab, Q, W, E, R, T, Y, U, I, O, P, BracketOpen,
			BracketClose, BackSlash,
			Compose, A, S, D, F, G, H, J, K, L, Semicolon,
			Apostrophe, Enter,
			LShift, Z, X, C, V, B, N, M, Comma, Period, Slash,
			RShift,
			LCtrl, Alt, Space, RCtrl, Up, Down,
			Left, Right,
			ExtBacktick, ExtDelete, ExtInsert, ExtNumLock,
			ExtPageUp, ExtPageDown, ExtHome, ExtEnd,
			ExtAsterisk, ExtPlus, ExtAltGr
		];

		for v in variants.iter() {
			self.a(queue, *v);
		}

		// Set old keyboard state.
		self.keyo = self.keys;
		self.exto = self.exts;
	}

	fn a(&mut self, queue: &mut ::input::InputQueue, key: Key) {
		if (key as u8) < 64 {
			if (self.keys & (BIT64 >> key as usize)) != 0 {
				if (self.keyo & (BIT64 >> key as usize)) == 0 {
					queue.key_down(key);
				} else {
					queue.key_hold(key);
				}
			} else {
				if (self.keyo & (BIT64 >> key as usize)) != 0 {
					queue.key_up(key);
				}
			}
		} else {
			if (self.exts & (BIT16 >> (key as usize - 64))) != 0 {
				if (self.exto & (BIT16 >> (key as usize - 64)))
					== 0
				{
					queue.key_down(key);
				} else {
					queue.key_hold(key);
				}
			} else {
				if (self.exto & (BIT16 >> (key as usize - 64)))
					!= 0
				{
					queue.key_up(key);
				}
			}
		}
	}
}
