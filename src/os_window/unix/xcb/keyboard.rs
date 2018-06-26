// "awi" - Aldaron's Window Interface
//
// Copyright Jeron A. Lau 2017-2018.
// Distributed under the Boost Software License, Version 1.0.
// (See accompanying file LICENSE_1_0.txt or copy at
// https://www.boost.org/LICENSE_1_0.txt)

use c_void;

use super::ffi as xcb;

pub struct Keyboard {
	pub state: *mut c_void,
	keymap: *mut c_void,
	context: *mut c_void,
}

impl Keyboard {
	pub fn create(connection: &xcb::Connection) -> Keyboard {
		unsafe { xcb::use_xkb_extension(connection) }
		let device_id = unsafe {
			xcb::xkb_get_core_keyboard_device_id(connection)
		};
		let context = unsafe { xcb::xkb_context_new(connection) };
		let keymap = unsafe {
			xcb::xkb_x11_keymap_new_from_device(connection, context,
				device_id)
		};
		let state = unsafe {
			xcb::xkb_x11_state_new_from_device(connection, keymap,
				device_id)
		};

		Keyboard { context, keymap, state }
	}

// TODO: ?
/*	pub fn null() -> Keyboard {
		Keyboard {
			state: null_mut(),
			keymap: null_mut(),
			context: null_mut(),
		}
	}*/
}

// TODO: Drop this keyboard
#[allow(unused)]
pub fn drop(keyboard: Keyboard, connection: &xcb::Connection) -> () {
	unsafe {
		xcb::xkb_state_unref(connection, keyboard.state);
		xcb::xkb_keymap_unref(connection, keyboard.keymap);
		xcb::xkb_context_unref(connection, keyboard.context);
	}
}
