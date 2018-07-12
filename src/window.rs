// "awi" - Aldaron's Window Interface
//
// Copyright Jeron A. Lau 2017-2018.
// Distributed under the Boost Software License, Version 1.0.
// (See accompanying file LICENSE_1_0.txt or copy at
// https://www.boost.org/LICENSE_1_0.txt)

use os;
use afi;

/// A window on Windows, Android, IOS, Wayland, XWindows, Direct to Display,
/// Aldaron's OS, Arduino, Nintendo Switch, A Web Page, or No OS.
pub struct Window {
	os_window: os::Window,
	input_queue: ::input::InputQueue,
	keyboard: ::Keyboard,
	reset: bool,
	cm: ::stick::ControllerManager,
}

impl Window {
	/// Create a window, using `title` as the title, and `icon` as the
	/// window icon.  The format of icon is as follows:
	/// `(width, height, pixels)`.  You can load icons with aci.  `v` should
	/// be either `None` or `Some(visual_id from EGL)`.
	pub fn new(title: &str, icon: &afi::Graphic, v: Option<i32>) -> Window {
		let os_window = os::Window::new(title, icon.into_bgra(), v);
		let input_queue = ::input::InputQueue::new();
		let keyboard = ::Keyboard::new();
		let reset = false;
		let cm = ::stick::ControllerManager::new(vec![]);

		Window { os_window, input_queue, keyboard, reset, cm }
	}

	/// Get the type of connection, plus native window and connection
	/// handles to pass to ffi.  See `WindowConnection` for more details.
	pub fn get_connection(&self) -> ::WindowConnection {
		self.os_window.get_connection()
	}

	/// Get the width and height of the window, as a tuple.
	pub fn wh(&self) -> (u16, u16) {
		self.os_window.wh()
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

		// New Frame
		self.reset = true;
		self.get_events();
		self.update()
	}

	/// Poll for events.
	fn get_events(&mut self) {
		// Get window events, and update keyboard state.
		while self.os_window.poll_event(&mut self.input_queue,
			&mut self.keyboard) {}

		// Generate keyboard events from keyboard state.
		self.keyboard.add(&mut self.input_queue);

		// Generate controller events from stick
		self.input_queue.stick(&mut self.cm);
	}
}
