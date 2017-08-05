// lib/os_window/unix/xcb/native_connection.rs
// Graphical Software Packager
// Copyright 2017 (c) Aldaron's Tech
// Copyright 2017 (c) Jeron Lau
// Licensed under the MIT LICENSE

use ami::void_pointer::*;
use super::ffi as xcb;
use super::keyboard;

pub struct NativeConnection(pub xcb::Connection, pub u32, keyboard::Keyboard);

impl NativeConnection {
	pub fn create(xcb_dl: xcb::Dl) -> Self {
		let xcb = xcb_dl.dl_handle;

		if xcb.is_null() {
			return NativeConnection((NULL, xcb_dl), 0,
				keyboard::Keyboard::null((NULL, xcb_dl)));
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

	pub fn icon(self, icon: (u32, u32, &[u8])) -> Self {
		let mut vector : Vec<u32> = Vec::new();
		vector.push(icon.0);
		vector.push(icon.1);

		let width = icon.0 as usize;
		let height = icon.1 as usize;

		for i in 0usize..width as usize {
			for j in 0usize..height as usize {
				let mut pixel = 0x00000000;

				pixel |= 0x00010000 * icon.2[
					0 + 4 * (j + (width * i))] as u32;
				pixel |= 0x00000100 * icon.2[
					1 + 4 * (j + (width * i))] as u32;
				pixel |= 0x00000001 * icon.2[
					2 + 4 * (j + (width * i))] as u32;
				pixel |= 0x01000000 * icon.2[
					3 + 4 * (j + (width * i))] as u32;
				vector.push(pixel);
			}
		}

		unsafe {
			xcb::change_property(self.0, self.1, 6,
				xcb::get_atom(self.0, b"_NET_WM_ICON"),
				vector.as_slice());
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

	pub fn keyboard_state(&self) -> VoidPointer {
		self.2.state
	}
}

impl Drop for NativeConnection {
	fn drop(&mut self) -> () {
		unsafe { xcb::disconnect(self.0) }
	}
}

impl NativeConnection {
	pub fn failed(&self) -> bool {
		self .0 .0 == NULL
	}
}
