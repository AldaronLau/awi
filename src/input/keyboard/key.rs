// Aldaron's Window Interface
// Copyright (c) 2017 Jeron Aldaron Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE
//
// src/input/keyboard/key.rs

/// This enum represents a physical key on a keyboard.  There a 75 possible keys
#[derive(PartialEq, Eq)]
#[derive(Copy, Clone)]
#[repr(u8)]
pub enum Key {
	// Note: These rows are not necessarily the rows these keys are found.
	// Row1
	/// 1
	Num1 = 0u8,
	/// 2
	Num2 = 1,
	/// 3
	Num3 = 2,
	/// 4
	Num4 = 3,
	/// 5
	Num5 = 4,
	/// 6
	Num6 = 5,
	/// 7
	Num7 = 6,
	/// 8
	Num8 = 7,
	/// 9
	Num9 = 8,
	/// 0
	Num0 = 9,
	/// \-
	Minus = 10,
	/// \=
	EqualSign = 11,
	/// Backspace
	Backspace = 12,
	// Row2
	/// Tab
	Tab = 13,
	/// Q
	Q = 14,
	/// W
	W = 15,
	/// E
	E = 16,
	/// R
	R = 17,
	/// T
	T = 18,
	/// Y
	Y = 19,
	/// U
	U = 20,
	/// I
	I = 21,
	/// O
	O = 22,
	/// P
	P = 23,
	/// {
	BracketOpen = 24,
	/// }
	BracketClose = 25,
	/// Backslash
	BackSlash = 26,
	// Row3
	/// Compose (CAPS LOCK)
	Compose = 27,
	/// A
	A = 28,
	/// S
	S = 29,
	/// D
	D = 30,
	/// F
	F = 31,
	/// G
	G = 32,
	/// H
	H = 33,
	/// J
	J = 34,
	/// K
	K = 35,
	/// L
	L = 36,
	/// ;
	Semicolon = 37,
	/// '
	Apostrophe = 38,
	/// Enter
	Enter = 39,
	// Row4
	/// Left Shift
	LShift = 40,
	/// Z
	Z = 41,
	/// X
	X = 42,
	/// C
	C = 43,
	/// V
	V = 44,
	/// B
	B = 45,
	/// N
	N = 46,
	/// M
	M = 47,
	/// ,
	Comma = 48,
	/// .
	Period = 49,
	/// /
	Slash = 50,
	/// Right Shift
	RShift = 51,
	// Row5
	/// Left CTRL
	LCtrl = 52,
	/// Alt (Left)
	Alt = 53,
	/// Space
	Space = 54,
	/// Right Control
	RCtrl = 55,
	/// Up Arrow Key
	Up = 56,
	/// Down Arrow Key
	Down = 57,
	/// Left Arrow Key
	Left = 58,
	/// Right Arrow Key
	Right = 59,
	// Ext ( May require 2 keys to be pressed on some platforms )
	/// `
	ExtBacktick = 64,
	/// Delete
	ExtDelete = 65,
	/// Insert
	ExtInsert = 66,
	/// NumLock
	ExtNumLock = 67,
	/// Page Up
	ExtPageUp = 68,
	/// Page Down
	ExtPageDown = 69,
	/// Home
	ExtHome = 70,
	/// End
	ExtEnd = 71,
	/// \*
	ExtAsterisk = 72,
	/// \+
	ExtPlus = 73,
	/// AltGr (Right Alt)
	ExtAltGr = 74
}

impl ::std::fmt::Display for Key {
	fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
		use Key::*;

		// TODO: Write in keyboard layout & language of the user.
		write!(f, "{}", match *self {
			Num1 => "1",
			Num2 => "2",
			Num3 => "3",
			Num4 => "4",
			Num5 => "5",
			Num6 => "6",
			Num7 => "7",
			Num8 => "8",
			Num9 => "9",
			Num0 => "0",
			Minus => "-",
			EqualSign => "=",
			Backspace => "Backspace",
			Tab => "tab",
			Q => "Q",
			W => "W",
			E => "E",
			R => "R",
			T => "T",
			Y => "Y",
			U => "U",
			I => "I",
			O => "O",
			P => "P",
			BracketOpen => "[",
			BracketClose => "]",
			BackSlash => "\\",
			Compose => "Compose",
			A => "A",
			S => "S",
			D => "D",
			F => "F",
			G => "G",
			H => "H",
			J => "J",
			K => "K",
			L => "L",
			Semicolon => ";",
			Apostrophe => "'",
			Enter => "enter",
			LShift => "Left Shift",
			RShift => "Right Shift",
			Z => "Z",
			X => "X",
			C => "C",
			V => "V",
			B => "B",
			N => "N",
			M => "M",
			Comma => ",",
			Period => ".",
			Slash => "/",
			LCtrl => "Left Ctrl",
			RCtrl => "Right Ctrl",
			Alt => "Alt",
			ExtAltGr => "AltGr",
			Space => "space",
			Up => "Up",
			Down => "Down",
			Left => "Left",
			Right => "Right",
			ExtBacktick => "`",
			ExtDelete => "Delete",
			ExtInsert => "Insert",
			ExtNumLock => "NumLock",
			ExtPageUp => "PageUp",
			ExtPageDown => "PageDown",
			ExtHome => "Home",
			ExtEnd => "End",
			ExtAsterisk => "*",
			ExtPlus => "+",
		})
	}
}

impl Key {
	// create a `Key` from keycode
	pub(crate) fn new(physical_key: u32) -> Option<Key> {
		use os_window::key;

		Some( match physical_key {
			key::ext::BACKTICK => Key::ExtBacktick,
			key::ext::NUM_PAD_PLUS => Key::ExtPlus,
			key::ext::NUM_PAD_ASTERISK => Key::ExtAsterisk,
			key::SLASH | key::ext::NUM_PAD_SLASH => Key::Slash,
			key::ENTER | key::ext::NUM_PAD_ENTER => Key::Enter,
			key::NUM_1 | key::ext::NUM_PAD_1 => Key::Num1,
			key::NUM_2 | key::ext::NUM_PAD_2 => Key::Num2,
			key::NUM_3 | key::ext::NUM_PAD_3 => Key::Num3,
			key::NUM_4 | key::ext::NUM_PAD_4 => Key::Num4,
			key::NUM_5 | key::ext::NUM_PAD_5 => Key::Num5,
			key::NUM_6 | key::ext::NUM_PAD_6 => Key::Num6,
			key::NUM_7 | key::ext::NUM_PAD_7 => Key::Num7,
			key::NUM_8 | key::ext::NUM_PAD_8 => Key::Num8,
			key::NUM_9 | key::ext::NUM_PAD_9 => Key::Num9,
			key::NUM_0 | key::ext::NUM_PAD_0 => Key::Num0,
			key::PERIOD | key::ext::NUM_PAD_PERIOD => Key::Period,
			key::MINUS | key::ext::NUM_PAD_MINUS => Key::Minus,
			key::EQUAL_SIGN => Key::EqualSign,
			key::BACKSPACE => Key::Backspace,
			key::TAB => Key::Tab,
			key::Q => Key::Q,
			key::W => Key::W,
			key::E => Key::E,
			key::R => Key::R,
			key::T => Key::T,
			key::Y => Key::Y,
			key::U => Key::U,
			key::I => Key::I,
			key::O => Key::O,
			key::P => Key::P,
			key::BRACKET_OPEN => Key::BracketOpen,
			key::BRACKET_CLOSE => Key::BracketClose,
			key::LEFT_CTRL => Key::LCtrl,
			key::RIGHT_CTRL => Key::RCtrl,
			key::LEFT_SHIFT => Key::LShift,
			key::RIGHT_SHIFT => Key::RShift,
			key::LEFT_ALT => Key::Alt,
			key::ext::ALT_GR => Key::ExtAltGr,
			key::CAPS_LOCK => Key::Compose,
			key::A => Key::A,
			key::S => Key::S,
			key::D => Key::D,
			key::F => Key::F,
			key::G => Key::G,
			key::H => Key::H,
			key::J => Key::J,
			key::K => Key::K,
			key::L => Key::L,
			key::SEMICOLON => Key::Semicolon,
			key::APOSTROPHE => Key::Apostrophe,
			key::BACKSLASH => Key::BackSlash,
			key::Z => Key::Z,
			key::X => Key::X,
			key::C => Key::C,
			key::V => Key::V,
			key::B => Key::B,
			key::N => Key::N,
			key::M => Key::M,
			key::COMMA => Key::Comma,
			key::SPACE => Key::Space,
			key::ext::NUMLOCK => Key::ExtNumLock,
			key::ext::HOME => Key::ExtHome,
			key::ext::END => Key::ExtEnd,
			key::ext::PAGE_UP => Key::ExtPageUp,
			key::ext::PAGE_DOWN => Key::ExtPageDown,
			key::ext::INSERT => Key::ExtInsert,
			key::ext::DELETE => Key::ExtDelete,
			key::UP => Key::Up,
			key::LEFT => Key::Left,
			key::RIGHT => Key::Right,
			key::DOWN => Key::Down,
			_ => return None,
		} )
	}
}
