// lib/window/mod.rs
// Graphical Software Packager
// Copyright 2017 (c) Aldaron's Tech
// Copyright 2017 (c) Jeron Lau
// Licensed under the MIT LICENSE

use WindowOps;
use aci_png;

#[macro_export] macro_rules! connect {
	() => ( {
		Window::new(include!(concat!(env!("CARGO_MANIFEST_DIR"),
				"/target/res/src/name.rs")),
			include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"),
				"/res/icon.png")))
	} )
}

/// A window on Windows, Android, IOS, Wayland, XWindows, Direct to Display,
/// Aldaron's OS, Arduino, Nintendo Switch, A Web Page, or No OS.
pub struct Window {
	os_window: ::os_window::OSWindow,
	dimensions: (u32, u32),
}

impl Window {
	/// Create a window, using `title` as the title, and `icon` as the
	/// window icon.  The format of icon is as follows:
	/// `(width, height, pixels)`.  You can load icons with aci.
	pub fn new(title: &str, icon: &'static [u8]) -> Window {
		let icon = match aci_png::decode(icon) {
			Ok(icon) => icon,
			Err(err) => panic!("ERROR DECODING PNG {}", err),
		};
		let icon = (icon.0, icon.1, icon.2.as_slice());
		let os_window = ::os_window::OSWindow::create(title, icon);

		// Make the window visible.
		os_window.show();
		// Update the window.
		os_window.update();

		Window { os_window, dimensions: (0, 0) }
	}

	/// Redraw the window, and get any new user input.
	pub fn update(&mut self, queue: &mut ::InputQueue) -> () {
		self.os_window.update();

		queue.clear();

		while self.os_window.poll_event(queue, &mut self.dimensions) {}

		if queue.get_fullscreen() {
			self.fullscreen();
		}
	}

	/// Toggle whether the window is fullscreen.
	pub fn fullscreen(&self) {
		self.os_window.fullscreen();
	}

	/// Get the type of connection, plus native window and connection
	/// handles to pass to ffi.  See `WindowConnection` for more details.
	pub fn get_connection(&self) -> ::WindowConnection {
		self.os_window.get_connection()
	}

	/// Get the width and height of the window, as a tuple.
	pub fn get_dimensions(&self) -> (u32, u32) {
		self.dimensions
	}
}
