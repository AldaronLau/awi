// Aldaron's Window Interface
// Copyright (c) 2017 Jeron Aldaron Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE
//
// src/input/keyboard/key.rs

/// This enum represents a physical key on a keyboard.  There a 71 possible keys
#[derive(PartialEq, Eq)]
#[derive(Copy, Clone)]
#[repr(u8)]
pub enum Key {
	// Note: These rows are not necessarily the rows these keys are found.
	// Row1
	Num1 = 0u8, Num2 = 1, Num3 = 2, Num4 = 3, Num5 = 4, Num6 = 5, Num7 = 6,
	Num8 = 7, Num9 = 8, Num0 = 9, Minus = 10, EqualSign = 11,
	Backspace = 12,
	// Row2
	Tab = 13, Q = 14, W = 15, E = 16, R = 17, T = 18, Y = 19, U = 20,
	I = 21, O = 22, P = 23, BracketOpen = 24, BracketClose = 25,
	BackSlash = 26,
	// Row3
	Compose = 27, A = 28, S = 29, D = 30, F = 31, G = 32, H = 33, J = 34,
	K = 35, L = 36, Semicolon = 37, Apostrophe = 38, Enter = 39,
	// Row4
	LShift = 40, Z = 41, X = 42, C = 43, V = 44, B = 45, N = 46,
	M = 47, Comma = 48, Period = 49, Slash = 50, RShift = 51,
	// Row5
	LCtrl = 52, Alt = 53, Space = 54, RCtrl = 55, Up = 56, Down = 57,
	Left = 58, Right = 59,
	// Ext ( May require 2 keys to be pressed on some platforms )
	ExtBacktick = 64, ExtDelete = 65, ExtInsert = 66, ExtNumLock = 67,
	ExtPageUp = 68, ExtPageDown = 69, ExtHome = 70, ExtEnd = 71,
	ExtAsterisk = 72, ExtPlus = 73, ExtAltGr = 74
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
