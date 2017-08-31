// Aldaron's Window Interface
// Copyright (c) 2017 Plop Grizzly, Jeron Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE
//
// src/os_window/unix/xcb/keyboard.rs

use ami::*;
use super::ffi as xcb;

pub struct Keyboard {
	connection: xcb::Connection,
	pub state: *mut Void,
	keymap: *mut Void,
	context: *mut Void,
}

impl Keyboard {
	pub fn create(connection: xcb::Connection) -> Keyboard {
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

		Keyboard { connection, context, keymap, state }
	}

	pub fn null(connection: xcb::Connection) -> Keyboard {
		Keyboard {
			connection,
			state: NULL.as_mut_ptr(),
			keymap: NULL.as_mut_ptr(),
			context: NULL.as_mut_ptr(),
		}
	}
}

impl Drop for Keyboard {
	fn drop(&mut self) -> () {
		unsafe {
			xcb::xkb_state_unref(self.connection, self.state);
			xcb::xkb_keymap_unref(self.connection, self.keymap);
			xcb::xkb_context_unref(self.connection, self.context);
		}
	}
}
