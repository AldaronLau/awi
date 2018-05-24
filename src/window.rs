// window.rs -- Aldaron's Window Interface
// Copyright (c) 2017-2018  Jeron A. Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE

use afi;

use WindowOps;

/// A window on Windows, Android, IOS, Wayland, XWindows, Direct to Display,
/// Aldaron's OS, Arduino, Nintendo Switch, A Web Page, or No OS.
pub struct Window {
	os_window: ::os_window::OSWindow,
	input_queue: ::input::InputQueue,
	dimensions: (u32, u32),
	keyboard: ::Keyboard,
	reset: bool,
	cm: ::stick::ControllerManager,
}

impl Window {
	/// Create a window, using `title` as the title, and `icon` as the
	/// window icon.  The format of icon is as follows:
	/// `(width, height, pixels)`.  You can load icons with aci.  `v` should
	/// be either `None` or `Some(visual_id from EGL)`.
	pub fn new(title: &str, icon: &afi::Graphic, v: Option<i32>)
		-> Window
	{
		let mut icon = (*icon).clone();

		icon.bgra();

		let os_window = ::os_window::OSWindow::new(title,
			icon.as_slice(), v);
		let dimensions = (::MWW, ::MWH); // Width & Height
		let input_queue = ::input::InputQueue::new();
		let keyboard = ::Keyboard::new();
		let reset = false;
		let cm = ::stick::ControllerManager::new(vec![]);

		// Make the window visible.
		os_window.show();
		// Update the window.
		os_window.update();

		Window {os_window, dimensions, input_queue, keyboard, reset, cm}
	}

	/// Toggle whether the window is fullscreen.
	pub fn fullscreen(&mut self) {
		self.os_window.fullscreen();
	}

	/// Get the type of connection, plus native window and connection
	/// handles to pass to ffi.  See `WindowConnection` for more details.
	pub fn get_connection(&self) -> ::WindowConnection {
		self.os_window.get_connection()
	}

	/// Get the width and height of the window, as a tuple.
	pub fn wh(&self) -> (u32, u32) {
		self.dimensions
	}

	/// Poll window input, return `None` when finished.  After returning
	/// `None`, the next call will update the window.
	pub fn update(&mut self) -> Option<::Input> {
		// First, update & get events
		// Next, cycle them
		// Then, Return None when through event loop.
		if let Some(input) = self.input_queue.pop() {
			return Some(input);
		} else if self.reset {
			self.reset = false;
			return None;
		}

		self.reset = true;
		self.os_window.update();
		self.get_events();

		self.update()
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

		// Generate controller events from stick
		self.input_queue.stick(&mut self.cm);
	}
}
