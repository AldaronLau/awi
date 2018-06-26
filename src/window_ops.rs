// "awi" - Aldaron's Window Interface
//
// Copyright Jeron A. Lau 2017-2018.
// Distributed under the Boost Software License, Version 1.0.
// (See accompanying file LICENSE_1_0.txt or copy at
// https://www.boost.org/LICENSE_1_0.txt)

/// Native window operations for implementing new platforms.
pub trait WindowOps {
	/// Create the window.
	fn new(title: &str, icon: (u32, u32, &[u32]), v: Option<i32>) -> Self;
	/// Show the window.
	fn show(&self) -> ();
	/// Re-draw the window.
	fn update(&self) -> ();
	/// Poll for events, returns true if there's more.  Adds 1+ to input.
	fn poll_event(&mut self, input: &mut ::input::InputQueue,
		wh: &mut(u32,u32), keyboard: &mut ::Keyboard) -> bool;
	/// Toggle fullscreen.
	fn fullscreen(&mut self) -> ();
	/// Get connection details
	fn get_connection(&self) -> ::WindowConnection;
}
