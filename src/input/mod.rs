// Aldaron's Window Interface
// Copyright (c) 2017 Jeron Aldaron Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE
//
// src/input/mod.rs

pub(crate) mod cursor;
pub(crate) mod keyboard;
mod joystick;
mod ffi;

pub use self::keyboard::Key;
pub use self::keyboard::msg::Msg;
pub use self::cursor::Click;
pub use self::joystick::Joystick;
pub use self::joystick::Button;

/// Input to the window, that's put into the input queue, when an event has
/// occurred.
#[derive(PartialEq)]
#[derive(Copy, Clone)]
pub enum Input {
	/// The window has just been resized.
	Resize,
	/// The user has switched to this window (in focus).
	Resume,
	/// The user has switched to a different window (out of focus).
	Pause,
	/// One of the following has happenned,
	///
	/// - A key has been pressed on a physical keyboard.
	/// - A key has been pressed on an on-screen keyboard.
	KeyPress(Key),
	/// One of the following has happenned,
	///
	/// - A key is being held down on a physical keyboard.
	/// - A key is being held down on an on-screen keyboard.
	KeyHold(Key),
	/// One of the following has happenned,
	///
	/// - A key has been released on a physical keyboard.
	/// - A key has been released on an on-screen keyboard.
	KeyRelease(Key),
	/// The user has inputted text.
	Text(char),
	/// One of the following has happenned,
	///
	/// - A keyboard shortcut has been used.
	/// - A graphical shortcut has been used.
	Msg(Msg),
	/// `Cursor(x, y)`: One of the following has happenned,
	///
	/// - The user moves the cursor with the mouse.
	/// - The user moves the cursor with the touchpad.
	/// - The last place that the user touched the touchscreen changes.
	Cursor(Option<(f32,f32)>),
	/// `CursorPress(click, x, y)`: A mouse button has been pressed.
	CursorPress(Click, (f32,f32)),
	/// `CursorRelease(click, x, y)`: A mouse button has been released.
	CursorRelease(Click, Option<(f32,f32)>),
	/// One of the following has happenned,
	///
	/// - The mouse wheel has been scrolled up.
	/// - The touchpad has been used to scroll up.
	ScrollUp(f32,f32),
	/// One of the following has happenned,
	///
	/// - The mouse wheel has been scrolled down.
	/// - The touchpad has been used to scroll down.
	ScrollDown(f32,f32),
	/// One of the following has happenned,
	///
	/// - The mouse wheel has been scrolled left.
	/// - The touchpad has been used to scroll left.
	ScrollLeft(f32,f32),
	/// One of the following has happenned,
	///
	/// - The mouse wheel has been scrolled right.
	/// - The touchpad has been used to scroll right.
	ScrollRight(f32,f32),
	/// One of the following has happenned,
	///
	/// - The joystick has moved to a different position.
	/// - The C-pad has moved.
	/// - The on-screen joystick 1 has moved.
	JoystickMove(f32, f32),
	/// One of the following has happenned,
	///
	/// - The joystick's POV hat has moved.
	/// - The POV-Joystick has moved.
	/// - The on-screen joystick 2 has moved.
	JoystickPov(f32, f32),
	/// One of the following has happenned,
	///
	/// - The joystick's throttle has moved.
	/// - The on-screen throttle has moved.
	JoystickThrottle(f32),
	/// One of the following has happenned,
	///
	/// - One of the joystick's buttons has been pressed.
	/// - An on-screen button has been pressed.
	JoystickButtonDown(Button),
	/// One of the following has happenned,
	///
	/// - One of the joystick's buttons has been released.
	/// - An on-screen button has been released.
	JoystickButtonUp(Button),
	/// Keyboard Shortcut - Align
	ScAlign(::afi_docf::Align),
	/// Keyboard Shortcut - Emphasis
	ScEmphasis(::afi_docf::Emphasis),
	/// Keyboard Shortcut - Text Color
	ScColor(::afi_docf::FontColor),
}

impl ::std::fmt::Display for Input {
	fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result<> {
		use Input::*;

		match *self {
			Resize => write!(f, "Resize"),
			Resume => write!(f, "Resume"),
			Pause => write!(f, "Pause"),
			KeyPress(key) => write!(f, "Key Press {}", key),
			KeyHold(key) => write!(f, "Key Hold {}", key),
			KeyRelease(key) => write!(f, "Key Release {}", key),
			Text(chr) => write!(f, "Text {}", chr),
			Msg(msg) => write!(f, "Message {}", msg),
			Cursor(xy) => write!(f, "Cursor {:?}", xy),
			CursorPress(c, xy) => write!(f, "Cursor Press {} {:?}", c, xy),
			CursorRelease(c, xy) => write!(f, "Cursor Release {} {:?}", c, xy),
			ScrollUp(x,y) => write!(f, "Scroll Up ({}, {})",x,y),
			ScrollDown(x,y) => write!(f, "Scroll Down ({}, {})",x,y),
			ScrollLeft(x,y) => write!(f, "Scroll Left ({}, {})",x,y),
			ScrollRight(x,y) => write!(f, "Scroll Right ({}, {})",x,y),
			_ => write!(f, "FIXME: Unknown") // FIXME
		}
	}
}

pub enum ScrollWheel {
	Up,
	Down,
	Left,
	Right,
}

trait CoordToFloat {
	fn to_f32(self) -> f32;
}

impl CoordToFloat for u32 {
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
	fullscreen: bool,
}

impl InputQueue {
	/// Get an empty InputQueue.
	#[inline(always)]
	pub fn new() -> InputQueue {
		let queue = Vec::new();
		let mods = keyboard::modifiers::Modifiers::create();
		let fullscreen = false;

		InputQueue { queue, mods, fullscreen }
	}

	/// Returns an iterator over the InputQueue.
	#[inline(always)]
	pub fn iter(&self) -> ::std::slice::Iter<Input> {
		self.queue.iter()
	}

	#[inline(always)]
	pub fn get_fullscreen(&self) -> bool {
		self.fullscreen
	}

	#[inline(always)]
	pub fn clear(&mut self) {
		self.fullscreen = false;
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
	pub fn resize(&mut self, wh: &mut (u32,u32), d: (u32,u32)) {
		// Only if new dimensions differ from old.
		if *wh != d {
			*wh = d;
			self.input(Input::Resize);
		}
	}

	#[inline(always)]
	pub fn fullscreen(&mut self) {
		self.fullscreen = true
	}

	#[inline(always)]
	pub fn key_down(&mut self, key: Key) {
		match key {
			_ => self.input(Input::KeyPress(key)),
		}
	}

	#[inline(always)]
	pub fn key_hold(&mut self, key: Key) {
		self.input(Input::KeyHold(key));
	}

	#[inline(always)]
	pub fn key_up(&mut self, key: Key) {
		self.input(Input::KeyRelease(key));
	}

	#[inline(always)]
	pub fn scroll(&mut self, wh: (u32, u32), c: (i16, i16),
		direction: ScrollWheel, times: usize)
	{
		let xy = cursor_coordinates(wh, c);

		if let Some((x, y)) = xy {
			match direction {
				ScrollWheel::Up => self.push(Input::ScrollUp(x,y),times),
				ScrollWheel::Down => self.push(Input::ScrollDown(x,y),times),
				ScrollWheel::Left => self.push(Input::ScrollLeft(x,y),times),
				ScrollWheel::Right => self.push(Input::ScrollRight(x,y),times),
			}
		}
	}

	#[inline(always)]
	pub fn button_down(&mut self, wh: (u32, u32), c: (i16, i16),
		button_id: Click)
	{
		let xy = cursor_coordinates(wh, c);

		if let Some(xy) = xy {
			self.input(Input::CursorPress(button_id, xy));
		}
	}

	#[inline(always)]
	pub fn button_up(&mut self, wh: (u32, u32), c: (i16, i16),
		button_id: Click)
	{
		let xy = cursor_coordinates(wh, c);

		self.input(Input::CursorRelease(button_id, xy));
	}

	#[inline(always)]
	pub fn cursor_move(&mut self, wh: (u32,u32), c: (i16,i16)) {
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
		self.input(Input::Msg(Msg::Back));
	}

	#[inline(always)]
	pub fn text(&mut self, string: String) {
		let chars = string.char_indices();

		for c in chars {
			self.input(Input::Text(c.1));
		}
	}	

	#[inline(always)]
	fn push(&mut self, input: Input, repeat: usize) {
		for _ in 0..repeat {
			self.input(input);
		}
	}

	#[inline(always)]
	fn input(&mut self, input: Input) -> () {
		self.mods.update(&mut self.queue, input)
	}
}
