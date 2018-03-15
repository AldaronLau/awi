// input/cursor.rs -- Aldaron's Window Interface
// Copyright (c) 2017-2018  Jeron A. Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE

/// A Mouse Click or Touch (for touchscreens)
#[derive(PartialEq)]
#[derive(Copy, Clone)]
pub enum Click {
	/// Left Click
	Left,
	/// Middle Click
	Middle,
	/// Right Click (or CTRL-Click)
	Right,
	/// Touch (on a touchscreen)
	Touch,
}

impl ::std::fmt::Display for Click {
	fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
		use Click::*;

		// TODO: Write in language of the user.
		write!(f, "{}", match *self {
			Left => "Left Click",
			Middle => "Middle Click",
			Right => "Right Click",
			Touch => "Touch",
		})
	}
}
