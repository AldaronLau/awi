// lib/os_window/unix/xcb/native_connection.rs
// Graphical Software Packager
// Copyright 2017 (c) Aldaron's Tech
// Copyright 2017 (c) Jeron Lau
// Licensed under the MIT LICENSE

use ami::*;
use super::ffi as xcb;
use super::keyboard;

pub struct NativeConnection(pub xcb::Connection, pub u32, keyboard::Keyboard);

impl NativeConnection {
	pub fn new(xcb_dl: xcb::Dl) -> Self {
		let xcb = xcb_dl.dl_handle;

		if xcb.is_null() {
			return NativeConnection((NULL.as_mut_ptr(), xcb_dl), 0,
				keyboard::Keyboard::null((NULL.as_mut_ptr(), xcb_dl)));
		}

		let connection = (unsafe { xcb::connect(xcb) }, xcb_dl);
		let window = unsafe {
			let window = xcb::generate_id(connection);
			let screen = xcb::screen_root(connection);

			xcb::create_window(connection, window, screen);

			window
		};
		let keyboard = keyboard::Keyboard::create(connection);

		NativeConnection(connection, window, keyboard)
	}

	pub fn title(self, title: &str) -> Self {
		let title = title.as_bytes();

		unsafe { xcb::change_property_title(self.0, self.1, title) }

		self
	}

	pub fn icon(self, icon: &[u32]) -> Self {
		unsafe {
			xcb::change_property(self.0, self.1, 6,
				xcb::get_atom(self.0, b"_NET_WM_ICON"), icon);
		}

		self
	}

	pub fn update(&self) -> () {
		unsafe { xcb::flush(self.0) }
	}

	pub fn show(&self) -> () {
		unsafe { xcb::map_window(self.0, self.1) }
	}

	pub fn connection(&self) -> xcb::Connection {
		self.0
	}

	pub fn keyboard_state(&self) -> *mut Void {
		self.2.state
	}
}

impl Drop for NativeConnection {
	fn drop(&mut self) -> () {
		unsafe { xcb::destroy_window(self.0, self.1) }
		unsafe { xcb::disconnect(self.0) }
	}
}

impl NativeConnection {
	pub fn failed(&self) -> bool {
		self .0 .0 == NULL.as_mut_ptr()
	}
}
