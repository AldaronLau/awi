// "awi" crate - Licensed under the MIT LICENSE
//  * Copyright (c) 2017-2018  Jeron A. Lau <jeron.lau@plopgrizzly.com>

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
