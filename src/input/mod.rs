// Copyright Jeron A. Lau 2017-2018.
// Dual-licensed under either the MIT License or the Boost Software License,
// Version 1.0.  (See accompanying file LICENSE_1_0.txt or copy at
// https://www.boost.org/LICENSE_1_0.txt)

pub(crate) mod keyboard;

/// Input to the window, that's put into the input queue, when an event has
/// occurred.
#[derive(PartialEq, Copy, Clone)]
pub enum Input {
	/// The window has just been resized.
	Resize,
	/// The user has switched to this window (in focus).
	Resume,
	/// The user has switched to a different window (out of focus).
	Pause,
	/// The user has inputted text.
	Text(char),
	/// Keyboard Shortcut - (CTRL-L) Align Left
	AlignLeft,
	/// Keyboard Shortcut - (CTRL-;) Align Center
	AlignCenter,
	/// Keyboard Shortcut - (CTRL-') Align Right
	AlignRight,
	/// Keyboard Shortcut - (CTRL-ENTER) Align Justified
	AlignJustified,
	/// Keyboard Shortcut - (CTRL-6) Emphasis Broken Underline
	EmphasisBrokenUnderline,
	/// Keyboard Shortcut - (CTRL-7) Emphasis Continuous Overline
	EmphasisOverline,
	/// Keyboard Shortcut - (CTRL-8) Emphasis Bold
	EmphasisBold,
	/// Keyboard Shortcut - (CTRL-9) Emphasis InvertColor
	EmphasisInvertColor,
	/// Keyboard Shortcut - (CTRL-0) Emphasis None
	EmphasisNone,
	/// Keyboard Shortcut - (CTRL-MINUS) Emphasis Strike Out
	EmphasisStrikeOut,
	/// Keyboard Shortcut - (CTRL-EQUALS) Emphasis Double Underline
	EmphasisDoubleUnderline,
	/// Keyboard Shortcut - (CTRL-U) Emphasis Underline
	EmphasisUnderline,
	/// Keyboard Shortcut - (CTRL-I) Emphasis Italic
	EmphasisItalic,
	/// Keyboard Shortcut - Select All (CTRL-A)
	Select,
	/// Keyboard Shortcut - Copy (CTRL-C)
	Copy,
	/// Keyboard Shortcut - Cancel (ALT-C)
	Cancel,
	/// Keyboard Shortcut - Delete (SHIFT-BACKSPACE)
	Delete,
	/// Keyboard Shortcut - Find (CTRL-F)
	Find,
	/// Keyboard Shortcut - Help
	Help,
	/// Keyboard Shortcut - Info
	Info,
	/// Request to exit the current screen - Back key / (Esc)
	Back,
	/// Request to exit the app - 'X' button on app's window / (Ctrl-Q)
	Quit,
	/// Keyboard Shortcut - Close (Ctrl-W)
	Close,
	/// Keyboard Shortcut - Open (Ctrl-O)
	Open(Option<&'static str>),
	/// Keyboard Shortcut - Share (Ctrl-S)
	Share,
	/// Keyboard Shortcut - Save Copy (Ctrl-Shift-S)
	SaveCopy,
	/// Keyboard Shortcut - Undo (Ctrl-Z)
	Undo,
	/// Keyboard Shortcut - Redo (Ctrl-Shift-Z or Ctrl-Y)
	Redo,
	/// Keyboard Shortcut - Cut (Ctrl-X)
	Cut,
	/// Keyboard Shortcut - Paste (Ctrl-V)
	Paste,
	/// Keyboard Shortcut - Print (Ctrl-P)
	Print,
	/// Cursor moved
	Cursor(Option<(f32,f32)>),
	/// Left Click (Some(Just Clicked) = Pressed, Cursor XY)
	LeftButton(Option<bool>, Option<(f32, f32)>),
	/// Middle Click (or SHIFT-Click) (Some(Just Clicked) = Pressed, Cursor XY)
	MiddleButton(Option<bool>, Option<(f32, f32)>),
	/// Right Click (or CTRL-Click) (Some(Just Clicked) = Pressed, Cursor XY)
	RightButton(Option<bool>, Option<(f32, f32)>),
	/// Touch (on a touchscreen) (Some(Just Clicked) = Pressed, Cursor XY)
	Touch(Option<bool>, Option<(f32, f32)>),
	/// Touchpad / Mousewheel scroll (x, y) - (-1, -1) is up / left, (1, 1)
	/// is down / right (Scroll XY, Cursor XY)
	Scroll((f32, f32), Option<(f32, f32)>),
	// Note: These rows are not necessarily the rows these keys are found.
	// Row1
	/// 1
	Num1(Option<bool>),
	/// 2
	Num2(Option<bool>),
	/// 3
	Num3(Option<bool>),
	/// 4
	Num4(Option<bool>),
	/// 5
	Num5(Option<bool>),
	/// 6
	Num6(Option<bool>),
	/// 7
	Num7(Option<bool>),
	/// 8
	Num8(Option<bool>),
	/// 9
	Num9(Option<bool>),
	/// 0
	Num0(Option<bool>),
	/// \-
	Minus(Option<bool>),
	/// \=
	EqualSign(Option<bool>),
	/// Backspace
	Backspace(Option<bool>),
	// Row2
	/// Tab
	Tab(Option<bool>),
	/// Q
	Q(Option<bool>), // = 14,
	/// W
	W(Option<bool>), // = 15,
	/// E
	E(Option<bool>), // = 16,
	/// R
	R(Option<bool>), // = 17,
	/// T
	T(Option<bool>), // = 18,
	/// Y
	Y(Option<bool>), // = 19,
	/// U
	U(Option<bool>), // = 20,
	/// I
	I(Option<bool>), // = 21,
	/// O
	O(Option<bool>), // = 22,
	/// P
	P(Option<bool>), // = 23,
	/// {
	BracketOpen(Option<bool>), // = 24,
	/// }
	BracketClose(Option<bool>), // = 25,
	/// Backslash
	Backslash(Option<bool>), // = 26,
	// Row3
	/// Compose (CAPS LOCK)
	Compose(Option<bool>), // = 27,
	/// A
	A(Option<bool>), // = 28,
	/// S
	S(Option<bool>), // = 29,
	/// D
	D(Option<bool>), // = 30,
	/// F
	F(Option<bool>), // = 31,
	/// G
	G(Option<bool>), // = 32,
	/// H
	H(Option<bool>), // = 33,
	/// J
	J(Option<bool>), // = 34,
	/// K
	K(Option<bool>), // = 35,
	/// L
	L(Option<bool>), // = 36,
	/// ;
	Semicolon(Option<bool>), // = 37,
	/// '
	Apostrophe(Option<bool>), // = 38,
	/// Enter
	Enter(Option<bool>), // = 39,
	// Row4
	/// Left Shift
	LShift(Option<bool>), // = 40,
	/// Z
	Z(Option<bool>), // = 41,
	/// X
	X(Option<bool>), // = 42,
	/// C
	C(Option<bool>), // = 43,
	/// V
	V(Option<bool>), // = 44,
	/// B
	B(Option<bool>), // = 45,
	/// N
	N(Option<bool>), // = 46,
	/// M
	M(Option<bool>), // = 47,
	/// ,
	Comma(Option<bool>), // = 48,
	/// .
	Period(Option<bool>), // = 49,
	/// /
	Slash(Option<bool>), // = 50,
	/// Right Shift
	RShift(Option<bool>), // = 51,
	// Row5
	/// Left CTRL
	LCtrl(Option<bool>), // = 52,
	/// Alt (Left)
	Alt(Option<bool>), // = 53,
	/// Space
	Space(Option<bool>), // = 54,
	/// Right Control
	RCtrl(Option<bool>), // = 55,
	/// Up Arrow Key
	Up(Option<bool>), // = 56,
	/// Down Arrow Key
	Down(Option<bool>), // = 57,
	/// Left Arrow Key
	Left(Option<bool>), // = 58,
	/// Right Arrow Key
	Right(Option<bool>), // = 59,
	// Ext ( May require 2 keys to be pressed on some platforms )
	/// `
	ExtBacktick(Option<bool>), // = 64,
	/// Delete
	ExtDelete(Option<bool>), // = 65,
	/// Insert
	ExtInsert(Option<bool>), // = 66,
	/// NumLock
	ExtNumLock(Option<bool>), // = 67,
	/// Page Up
	ExtPageUp(Option<bool>), // = 68,
	/// Page Down
	ExtPageDown(Option<bool>), // = 69,
	/// Home
	ExtHome(Option<bool>), // = 70,
	/// End
	ExtEnd(Option<bool>), // = 71,
	/// \*
	ExtAsterisk(Option<bool>), // = 72,
	/// \+
	ExtPlus(Option<bool>), // = 73,
	/// AltGr (Right Alt)
	ExtAltGr(Option<bool>), // = 74
	/// Controller: Main joystick movement.
	CMove(usize, f32, f32),
	/// Controller: Camera / C joystick movement.
	CCamera(usize, f32, f32),
	/// Controller: Left Throttle movement.
	CThrottleL(usize, f32),
	/// Controller: Right Throttle movement.
	CThrottleR(usize, f32),
	/// Controller: Accept (A Button / Left Top Button - Missle / Circle)
	CAccept(usize, Option<bool>),
	/// Controller: Cancel (B Button / Side Button / Cross)
	CCancel(usize, Option<bool>),
	/// Controller: Execute (X Button / Trigger / Triangle)
	CExecute(usize, Option<bool>),
	/// Controller: Action (Y Button / Right Top Button / Square)
	CAction(usize, Option<bool>),
	/// Controller: Left Button (0: L Trigger, 1: LZ / L Bumper).  0 is
	/// farthest away from user, incrementing as buttons get closer.
	CL(usize, u8, Option<bool>),
	/// Controller: Right Button (0: R Trigger, 1: Z / RZ / R Bumper). 0 is
	/// farthest away from user, incrementing as buttons get closer.
	CR(usize, u8, Option<bool>),
	/// Controller: Pause Menu (Start Button)
	CMenu(usize, Option<bool>),
	/// Controller: Show Controls (Guide on XBox, Select on PlayStation).
	/// Use as alternative for Menu -> "Controls".
	CControls(usize),
	/// Controller: Exit This Screen (Back on XBox).  Use as alternative for
	/// Menu -> "Quit" or Cancel, depending on situation.
	CExit(usize),
	/// Controller: HAT/DPAD Up Button
	CUp(usize, Option<bool>),
	/// Controller: HAT/DPAD Down Button
	CDown(usize, Option<bool>),
	/// Controller: Hat/D-Pad left button
	CLeft(usize, Option<bool>),
	/// Controller: Hat/D-Pad right button.
	CRight(usize, Option<bool>),
	/// Controller: Movement stick Push
	CMoveStick(usize, Option<bool>),
	/// Controller: Camera stick Push
	CCamStick(usize, Option<bool>),
	/// Controller: Device Plugged-In
	CPluggedIn(usize, i32),
	/// Controller: Device Un-Plugged
	CUnPlugged(usize, i32),
}

use self::Input::*;

impl ::std::fmt::Display for Input {
	fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result<> {
		// TODO: Write in language of the user.
		match self {
			Resize => write!(f, "Resize"),
			Resume => write!(f, "Resume"),
			Pause => write!(f, "Pause"),
			Text(chr) => write!(f, "Text {}", chr),
			Select => write!(f, "Select"),
			Copy => write!(f, "Copy"),
			Cancel => write!(f, "Cancel"),
			Delete => write!(f, "Delete"),
			Find => write!(f, "Find"),
			Help => write!(f, "Help"),
			Info => write!(f, "Info"),
			Back => write!(f, "Back"),
			Quit => write!(f, "Quit"),
			Close => write!(f, "Close"),
			Open(_) => write!(f, "Open..."),
			Share => write!(f, "Share..."),
			SaveCopy => write!(f, "Save A Copy..."),
			Undo => write!(f, "Undo"),
			Redo => write!(f, "Redo"),
			Cut => write!(f, "Cut"),
			Paste => write!(f, "Paste"),
			Print => write!(f, "Print"),
			Cursor(xy) => write!(f, "Cursor {:?}", xy),
			LeftButton(state, xy) => write!(f, "Left Click {:?} {:?}", state, xy),
			MiddleButton(state, xy) => write!(f, "Middle Click {:?} {:?}", state, xy),
			RightButton(state, xy) => write!(f, "Right Click {:?} {:?}", state, xy),
			Touch(state, xy) => write!(f, "Touch {:?} {:?}", state, xy),
			Scroll(sxy, xy) => write!(f, "Scroll {:?} {:?}",sxy,xy),
			Num1(state) => write!(f, "1 {:?}", state),
			Num2(state) => write!(f, "2 {:?}", state),
			Num3(state) => write!(f, "3 {:?}", state),
			Num4(state) => write!(f, "4 {:?}", state),
			Num5(state) => write!(f, "5 {:?}", state),
			Num6(state) => write!(f, "6 {:?}", state),
			Num7(state) => write!(f, "7 {:?}", state),
			Num8(state) => write!(f, "8 {:?}", state),
			Num9(state) => write!(f, "9 {:?}", state),
			Num0(state) => write!(f, "0 {:?}", state),
			Minus(state) => write!(f, "- {:?}", state),
			EqualSign(state) => write!(f, "= {:?}", state),
			Backspace(state) => write!(f, "Backspace {:?}", state),
			Tab(state) => write!(f, "tab {:?}", state),
			Q(state) => write!(f, "Q {:?}", state),
			W(state) => write!(f, "W {:?}", state),
			E(state) => write!(f, "E {:?}", state),
			R(state) => write!(f, "R {:?}", state),
			T(state) => write!(f, "T {:?}", state),
			Y(state) => write!(f, "Y {:?}", state),
			U(state) => write!(f, "U {:?}", state),
			I(state) => write!(f, "I {:?}", state),
			O(state) => write!(f, "O {:?}", state),
			P(state) => write!(f, "P {:?}", state),
			BracketOpen(state) => write!(f, "[ {:?}", state),
			BracketClose(state) => write!(f, "] {:?}", state),
			Backslash(state) => write!(f, "\\ {:?}", state),
			Compose(state) => write!(f, "Compose {:?}", state),
			A(state) => write!(f, "A {:?}", state),
			S(state) => write!(f, "S {:?}", state),
			D(state) => write!(f, "D {:?}", state),
			F(state) => write!(f, "F {:?}", state),
			G(state) => write!(f, "G {:?}", state),
			H(state) => write!(f, "H {:?}", state),
			J(state) => write!(f, "J {:?}", state),
			K(state) => write!(f, "K {:?}", state),
			L(state) => write!(f, "L {:?}", state),
			Semicolon(state) => write!(f, "; {:?}", state),
			Apostrophe(state) => write!(f, "' {:?}", state),
			Enter(state) => write!(f, "enter {:?}", state),
			LShift(state) => write!(f, "Left Shift {:?}", state),
			RShift(state) => write!(f, "Right Shift {:?}", state),
			Z(state) => write!(f, "Z {:?}", state),
			X(state) => write!(f, "X {:?}", state),
			C(state) => write!(f, "C {:?}", state),
			V(state) => write!(f, "V {:?}", state),
			B(state) => write!(f, "B {:?}", state),
			N(state) => write!(f, "N {:?}", state),
			M(state) => write!(f, "M {:?}", state),
			Comma(state) => write!(f, ", {:?}", state),
			Period(state) => write!(f, ". {:?}", state),
			Slash(state) => write!(f, "/ {:?}", state),
			LCtrl(state) => write!(f, "Left Ctrl {:?}", state),
			RCtrl(state) => write!(f, "Right Ctrl {:?}", state),
			Alt(state) => write!(f, "Alt {:?}", state),
			ExtAltGr(state) => write!(f, "AltGr {:?}", state),
			Space(state) => write!(f, "space {:?}", state),
			Up(state) => write!(f, "Up {:?}", state),
			Down(state) => write!(f, "Down {:?}", state),
			Left(state) => write!(f, "Left {:?}", state),
			Right(state) => write!(f, "Right {:?}", state),
			ExtBacktick(state) => write!(f, "` {:?}", state),
			ExtDelete(state) => write!(f, "Delete {:?}", state),
			ExtInsert(state) => write!(f, "Insert {:?}", state),
			ExtNumLock(state) => write!(f, "NumLock {:?}", state),
			ExtPageUp(state) => write!(f, "PageUp {:?}", state),
			ExtPageDown(state) => write!(f, "PageDown {:?}", state),
			ExtHome(state) => write!(f, "Home {:?}", state),
			ExtEnd(state) => write!(f, "End {:?}", state),
			ExtAsterisk(state) => write!(f, "* {:?}", state),
			ExtPlus(state) => write!(f, "+ {:?}", state),
			CMove(i, x, y) => write!(f, "C{} Move ({}, {})", i, x, y),
			CCamera(i, x, y) => write!(f, "C{} Camera ({}, {})", i, x, y),
			CThrottleL(i, x) => write!(f, "C{} ThrottleL ({})", i, x),
			CThrottleR(i, x) => write!(f, "C{} ThrottleR ({})", i, x),
			CAccept(i, s) => write!(f, "C{} Accept {:?}", i, s),
			CCancel(i, s) => write!(f, "C{} Cancel {:?}", i, s),
			CExecute(i, s) => write!(f, "C{} Execute {:?}", i, s),
			CAction(i, s) => write!(f, "C{} Action {:?}", i, s),
			CL(i, a, s) => write!(f, "C{} L-{} {:?}", i, a, s),
			CR(i, a, s) => write!(f, "C{} R-{} {:?}", i, a, s),
			CMenu(i, s) => write!(f, "C{} Menu {:?}", i, s),
			CControls(i) => write!(f, "C{} Controls", i),
			CExit(i) => write!(f, "C{} Exit", i),
			CUp(i, s) => write!(f, "C{} Up {:?}", i, s),
			CDown(i, s) => write!(f, "C{} Down {:?}", i, s),
			CLeft(i, s) => write!(f, "C{} Left {:?}", i, s),
			CRight(i, s) => write!(f, "C{} Right {:?}", i, s),
			CMoveStick(i, s) => write!(f, "C{} Movement Stick Push {:?}", i, s),
			CCamStick(i, s) => write!(f, "C{} Camera Stick Push {:?}", i, s),
			CPluggedIn(i, x) => write!(f, "C{} Device Plugged-In {:x}", i, x),
			CUnPlugged(i, x) =>  write!(f, "C{} Device Un-Plugged {:x}", i, x),
			_ => write!(f, "FIXME: Unknown") // FIXME
		}
	}
}

trait CoordToFloat {
	fn to_f32(self) -> f32;
}

impl CoordToFloat for u16 {
	fn to_f32(self) -> f32 { self as f32 }
}

impl CoordToFloat for i16 {
	fn to_f32(self) -> f32 { self as f32 }
}

fn cursor_coordinates<T, U>(wh: (T, T), xy: (U, U)) -> Option<(f32, f32)>
	where U: CoordToFloat, T: CoordToFloat
{
	let x = xy.0.to_f32();
	let y = xy.1.to_f32();
	let w = wh.0.to_f32();
	let h = wh.1.to_f32();
	let xy = (x * 2.0 / w - 1.0, y * 2.0 / h - 1.0);

	if xy.0 > 1.0 || xy.0 < -1.0 || xy.1 > 1.0 || xy.1 < -1.0 {
		None
	} else {
		Some(xy)
	}
}

pub struct InputQueue {
	queue: Vec<Input>,
	mods: keyboard::modifiers::Modifiers,
}

impl InputQueue {
	/// Get an empty InputQueue.
	#[inline(always)]
	pub fn new() -> InputQueue {
		let queue = Vec::new();
		let mods = keyboard::modifiers::Modifiers::create();

		InputQueue { queue, mods }
	}

	/// Returns an iterator over the InputQueue.
	#[inline(always)]
	pub fn iter(&self) -> ::std::slice::Iter<Input> {
		self.queue.iter()
	}

	#[inline(always)]
	pub fn clear(&mut self) {
		self.queue.clear()
	}

	#[inline(always)]
	pub fn len(&self) -> usize {
		self.queue.len()
	}

	#[inline(always)]
	pub fn is_empty(&self) -> bool {
		self.queue.len() == 0
	}

	#[inline(always)]
	pub fn pop(&mut self) -> Option<Input> {
		self.queue.pop()
	}

	#[inline(always)]
	pub fn last(&self) -> Input {
		self.queue[self.queue.len() - 1]
	}

	#[inline(always)]
	pub fn resize(&mut self, wh: &mut (u16, u16), d: (u16, u16)) {
		// Only if new dimensions differ from old.
		if *wh != d {
			*wh = d;
			self.input(Input::Resize);
		}
	}

	pub fn key(&mut self, key: u8, state: Option<bool>) {
		self.input(match key {
			keyboard::NUM1 => Input::Num1(state),
			keyboard::NUM2 => Input::Num2(state),
			keyboard::NUM3 => Input::Num3(state),
			keyboard::NUM4 => Input::Num4(state),
			keyboard::NUM5 => Input::Num5(state),
			keyboard::NUM6 => Input::Num6(state),
			keyboard::NUM7 => Input::Num7(state),
			keyboard::NUM8 => Input::Num8(state),
			keyboard::NUM9 => Input::Num9(state),
			keyboard::NUM0 => Input::Num0(state),
			keyboard::MINUS => Input::Minus(state),
			keyboard::EQUAL_SIGN => Input::EqualSign(state),
			keyboard::BACKSPACE => Input::Backspace(state),
			keyboard::TAB => Input::Tab(state),
			keyboard::Q => Input::Q(state),
			keyboard::W => Input::W(state),
			keyboard::E => Input::E(state),
			keyboard::R => Input::R(state),
			keyboard::T => Input::T(state),
			keyboard::Y => Input::Y(state),
			keyboard::U => Input::U(state),
			keyboard::I => Input::I(state),
			keyboard::O => Input::O(state),
			keyboard::P => Input::P(state),
			keyboard::BRACKET_OPEN => Input::BracketOpen(state),
			keyboard::BRACKET_CLOSE => Input::BracketClose(state),
			keyboard::BACKSLASH => Input::Backslash(state),
			keyboard::COMPOSE => Input::Compose(state),
			keyboard::A => Input::A(state),
			keyboard::S => Input::S(state),
			keyboard::D => Input::D(state),
			keyboard::F => Input::F(state),
			keyboard::G => Input::G(state),
			keyboard::H => Input::H(state),
			keyboard::J => Input::J(state),
			keyboard::K => Input::K(state),
			keyboard::L => Input::L(state),
			keyboard::SEMICOLON => Input::Semicolon(state),
			keyboard::APOSTROPHE => Input::Apostrophe(state),
			keyboard::ENTER => Input::Enter(state),
			keyboard::LSHIFT => Input::LShift(state),
			keyboard::Z => Input::Z(state),
			keyboard::X => Input::X(state),
			keyboard::C => Input::C(state),
			keyboard::V => Input::V(state),
			keyboard::B => Input::B(state),
			keyboard::N => Input::N(state),
			keyboard::M => Input::M(state),
			keyboard::COMMA => Input::Comma(state),
			keyboard::PERIOD => Input::Period(state),
			keyboard::SLASH => Input::Slash(state),
			keyboard::RSHIFT => Input::RShift(state),
			keyboard::LCTRL => Input::LCtrl(state),
			keyboard::ALT => Input::Alt(state),
			keyboard::SPACE => Input::Space(state),
			keyboard::RCTRL => Input::RCtrl(state),
			keyboard::UP => Input::Up(state),
			keyboard::DOWN => Input::Down(state),
			keyboard::LEFT => Input::Left(state),
			keyboard::RIGHT => Input::Right(state),
			keyboard::EXT_BACKTICK => Input::ExtBacktick(state),
			keyboard::EXT_DELETE => Input::ExtDelete(state),
			keyboard::EXT_INSERT => Input::ExtInsert(state),
			keyboard::EXT_NUM_LOCK => Input::ExtNumLock(state),
			keyboard::EXT_PAGE_UP => Input::ExtPageUp(state),
			keyboard::EXT_PAGE_DOWN => Input::ExtPageDown(state),
			keyboard::EXT_HOME => Input::ExtHome(state),
			keyboard::EXT_END => Input::ExtEnd(state),
			keyboard::EXT_ASTERISK => Input::ExtAsterisk(state),
			keyboard::EXT_PLUS => Input::ExtPlus(state),
			keyboard::EXT_ALT_GR => Input::ExtAltGr(state),
			_ => return,
		})
	}

	#[inline(always)]
	pub fn scroll(&mut self, wh: (u16, u16), c: (i16, i16),
		scrolling: (f32, f32))
	{
		let xy = cursor_coordinates(wh, c);

		self.input(Input::Scroll(scrolling, xy))
	}

	#[inline(always)]
	pub fn left_button_release(&mut self, wh: (u16, u16), c: (i16, i16)) {
		let xy = cursor_coordinates(wh, c);

		self.input(Input::LeftButton(None, xy));
	}

	#[inline(always)]
	pub fn middle_button_release(&mut self, wh: (u16, u16), c: (i16, i16)) {
		let xy = cursor_coordinates(wh, c);

		self.input(Input::MiddleButton(None, xy));
	}

	#[inline(always)]
	pub fn right_button_release(&mut self, wh: (u16, u16), c: (i16, i16)) {
		let xy = cursor_coordinates(wh, c);

		self.input(Input::RightButton(None, xy));
	}

	#[inline(always)]
	pub fn touch_release(&mut self, wh: (u16, u16), c: (i16, i16)) {
		let xy = cursor_coordinates(wh, c);

		self.input(Input::Touch(None, xy));
	}

	#[inline(always)]
	pub fn left_button_press(&mut self, wh: (u16, u16), c: (i16, i16)) {
		let xy = cursor_coordinates(wh, c);

		self.input(Input::LeftButton(Some(true), xy));
	}

	#[inline(always)]
	pub fn middle_button_press(&mut self, wh: (u16, u16), c: (i16, i16)) {
		let xy = cursor_coordinates(wh, c);

		self.input(Input::MiddleButton(Some(true), xy));
	}

	#[inline(always)]
	pub fn right_button_press(&mut self, wh: (u16, u16), c: (i16, i16)) {
		let xy = cursor_coordinates(wh, c);

		self.input(Input::RightButton(Some(true), xy));
	}

	#[inline(always)]
	pub fn touch_press(&mut self, wh: (u16, u16), c: (i16, i16)) {
		let xy = cursor_coordinates(wh, c);

		self.input(Input::Touch(Some(true), xy));
	}

	#[inline(always)]
	pub fn cursor_move(&mut self, wh: (u16, u16), c: (i16,i16)) {
		let xy = cursor_coordinates(wh, c);

		self.input(Input::Cursor(xy));
	}

	#[inline(always)]
	pub fn cursor_leave(&mut self) {
		self.input(Input::Cursor(None));
	}

	#[inline(always)]
	pub fn pause(&mut self) {
		self.input(Input::Pause);
	}

	#[inline(always)]
	pub fn resume(&mut self) {
		self.input(Input::Resume);
	}

	#[inline(always)]
	pub fn back(&mut self) {
		self.input(Input::Back);
	}

	#[inline(always)]
	pub fn text(&mut self, string: String) {
		let chars = string.char_indices();

		for c in chars {
			self.input(Input::Text(c.1));
		}
	}

	#[inline(always)]
	fn input(&mut self, input: Input) -> () {
		self.mods.update(&mut self.queue, input)
	}

	#[inline(always)]
	pub fn stick(&mut self, cm: &mut ::stick::ControllerManager) {
		while let Some((js, i)) = cm.update() {
			use ::stick::Input::*;

			match i {
				Move(x, y) => self.input(Input::CMove(js, x, y)),
				Camera(x, y) => self.input(Input::CCamera(js, x, y)),
				ThrottleL(x) => self.input(Input::CThrottleL(js, x)),
				ThrottleR(x) => self.input(Input::CThrottleR(js, x)),
				Accept(s) => self.input(Input::CAccept(js, s)),
				Cancel(s) => self.input(Input::CCancel(js, s)),
				Execute(s) => self.input(Input::CExecute(js, s)),
				Action(s) => self.input(Input::CAction(js, s)),
				L(b, s) => self.input(Input::CL(js, b, s)),
				R(b, s) => self.input(Input::CR(js, b, s)),
				Menu(s) => self.input(Input::CMenu(js, s)),
				Controls => self.input(Input::CControls(js)),
				Exit => self.input(Input::CExit(js)),
				Up(s) => self.input(Input::CUp(js, s)),
				Down(s) => self.input(Input::CDown(js, s)),
				Left(s) => self.input(Input::CLeft(js, s)),
				Right(s) => self.input(Input::CRight(js, s)),
				MoveStick(s) => self.input(Input::CMoveStick(js, s)),
				CamStick(s) => self.input(Input::CCamStick(js, s)),
				PluggedIn(i) => self.input(Input::CPluggedIn(js, i)),
				UnPlugged(i) => self.input(Input::CUnPlugged(js, i)),
			}
		}
	}
}
