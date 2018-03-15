// os_window/unix/xcb/native_connection.rs -- Aldaron's Window Interface
// Copyright (c) 2017-2018  Jeron A. Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE

use std::ptr::null_mut;
use libc::c_void;

use super::ffi as xcb;
use super::keyboard;

pub struct NativeConnection(pub xcb::Connection, pub u32, keyboard::Keyboard);

impl NativeConnection {
	pub fn new(xcb_dl: xcb::Dl, visual: Option<i32>) -> Self {
		let xcb = xcb_dl.dl_handle;

		if xcb.is_null() {
			return NativeConnection((null_mut(), xcb_dl), 0,
				keyboard::Keyboard::null((null_mut(), xcb_dl)));
		}

		let connection = (unsafe { xcb::connect(xcb) }, xcb_dl);
		let window = unsafe {
			let mut screen = xcb::screen_root(connection);
			let window = xcb::generate_id(connection);

			if let Some(v) = visual {
				println!("setting visual....");
				screen.1 = ::std::mem::transmute(v);
			}

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

	pub fn icon(self, icon: (u32, u32, &[u32])) -> Self {
		let mut ico = ::std::vec::Vec::new();

		ico.push(icon.0);
		ico.push(icon.1);
		ico.extend(icon.2);

		unsafe {
			xcb::change_property(self.0, self.1, 6,
				xcb::get_atom(self.0, b"_NET_WM_ICON"),
				ico.as_slice());
		}

		self
	}

	pub fn update(&self) -> () {
	}

	pub fn show(&self) -> () {
		unsafe { xcb::map_window(self.0, self.1) }
	}

	pub fn connection(&self) -> xcb::Connection {
		self.0
	}

	pub fn keyboard_state(&self) -> *mut c_void {
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
		self .0 .0 .is_null()
	}
}
