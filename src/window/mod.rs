// Aldaron's Window Interface
// Copyright (c) 2017 Plop Grizzly, Jeron Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE
//
// src/window/mod.rs

use afi;
use WindowOps;

/// Connect to a window.  This macro requires that aci_png be in scope, to load
/// the window's icon.
#[macro_export] macro_rules! connect {
	() => ( {
		Window::new(include!(concat!(env!("CARGO_MANIFEST_DIR"),
				"/target/res/src/name.rs")),
			aci_png::decode(
				include_bytes!(concat!(
					env!("CARGO_MANIFEST_DIR"),
					"/res/icon.png")))
				.unwrap())
	} )
}

/// A window on Windows, Android, IOS, Wayland, XWindows, Direct to Display,
/// Aldaron's OS, Arduino, Nintendo Switch, A Web Page, or No OS.
pub struct Window {
	os_window: ::os_window::OSWindow,
	input_queue: ::input::InputQueue,
	dimensions: (u32, u32),
	keyboard: ::Keyboard,
}

impl Window {
	/// Create a window, using `title` as the title, and `icon` as the
	/// window icon.  The format of icon is as follows:
	/// `(width, height, pixels)`.  You can load icons with aci.
	pub fn new(title: &str, mut icon: afi::Graphic) -> Window {
		icon.bgra();

		let os_window = ::os_window::OSWindow::new(title,
			icon.as_slice());
		let dimensions = (::MWW, ::MWH); // Width & Height
		let input_queue = ::input::InputQueue::new();
		let keyboard = ::Keyboard::new();

		// Make the window visible.
		os_window.show();
		// Update the window.
		os_window.update();

		Window { os_window, dimensions, input_queue, keyboard }
	}

	/// Redraw the window, and get any new user input.
	pub fn update(&mut self) -> () {
		self.os_window.update();
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

	/// Get input, if there's any.
	pub fn input(&mut self) -> Option<::Input> {
		let input = self.input_queue.pop();

		if input == None {
			self.get_events();
		}

		input
	}

	/// Return true if `Window` has been resized on this frame.
	pub fn get_resized(&self) -> bool {
		self.input_queue.get_resized()
	}

	/// Poll for events.
	fn get_events(&mut self) {
		// Get window events, and update keyboard state.
		while self.os_window.poll_event(&mut self.input_queue,
			&mut self.dimensions, &mut self.keyboard) {}

		// Generate keyboard events from keyboard state.
		self.keyboard.add(&mut self.input_queue);

		// If F11 pressed, toggle fullscreen.
		if self.input_queue.get_fullscreen() {
			self.fullscreen();
		}
	}
}
