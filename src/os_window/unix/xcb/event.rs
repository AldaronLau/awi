// "awi" crate - Licensed under the MIT LICENSE
//  * Copyright (c) 2017-2018  Jeron A. Lau <jeron.lau@plopgrizzly.com>

use libc::c_void;

use input;
use super::ffi as xcb;

const KEY_DOWN: u8 = 2;
const KEY_UP: u8 = 3;
const BUTTON_DOWN: u8 = 4;
const BUTTON_UP: u8 = 5;
const CURSOR_MOVE: u8 = 6;
const CURSOR_LEAVE: u8 = 8;
const GAIN_FOCUS: u8 = 9;
const LOSE_FOCUS: u8 = 10;
const WINDOW_RESIZE: u8 = 22;
const WINDOW_SELECT: u8 = 31;
const WINDOW_CLOSE: u8 = 128 | 33;

pub struct EventDetails {
	id: u8,
	detail: u32,
	xy: (i16, i16),
	wh: (u32, u32),
	utf8: Option<String>,
}

pub enum Event {
	Event(EventDetails),
	Stop,
}

impl Event {
	pub fn create(connection: xcb::Connection, state: *mut c_void) -> Event{
		let event = unsafe { xcb::poll_for_event(connection, state) };

		if let Some(e) = event {
			Event::Event(EventDetails {
				id: e.0,
				detail: e.1,
				xy: e.2,
				wh: (e .3 .0 as u32, e .3 .1 as u32),
				utf8: e.4
			})
		} else {
			Event::Stop
		}
	}

	pub fn poll(self, queue: &mut input::InputQueue, wh: &mut (u32, u32),
		keyboard: &mut ::Keyboard) -> bool
	{
		let e : EventDetails = if let Event::Event(details) = self {
			details
		} else {
			return false;
		};

		match e.id {
			KEY_DOWN => if let Some(key) = input::key(e.detail) {
				keyboard.press(key);
			} else if e.detail == 9 {
				queue.back();
			} else if e.detail == 95 {
				queue.fullscreen();
			},
			KEY_UP => if let Some(key) = input::key(e.detail) {
				keyboard.release(key);
			},
			BUTTON_DOWN => match e.detail {
				1 => queue.left_button_press(*wh, e.xy),
				2 => queue.middle_button_press(*wh, e.xy),
				3 => queue.right_button_press(*wh, e.xy),
				4 => queue.scroll(*wh, e.xy, (0.0, -1.0)),
				5 => queue.scroll(*wh, e.xy, (0.0, 1.0)),
				6 => queue.scroll(*wh, e.xy, (-1.0, 0.0)),
				7 => queue.scroll(*wh, e.xy, (1.0, 0.0)),
				uc => panic!("awi: Unknown Click {}!", uc)
			},
			BUTTON_UP => match e.detail {
				1 => queue.left_button_release(*wh, e.xy),
				2 => queue.middle_button_release(*wh, e.xy),
				3 => queue.right_button_release(*wh, e.xy),
				_ => {},
			},
			CURSOR_MOVE => queue.cursor_move(*wh, e.xy),
			CURSOR_LEAVE => queue.cursor_leave(),
			GAIN_FOCUS => queue.resume(),
			LOSE_FOCUS => queue.pause(),
			WINDOW_RESIZE => queue.resize(wh, e.wh),
			WINDOW_SELECT => println!("!SELECT!"),
			WINDOW_CLOSE => queue.back(),
			_ => { }, // ignore all other messages
		}

		if let Some(string) = e.utf8 {
			queue.text(string);
		}

		true
	}
}
