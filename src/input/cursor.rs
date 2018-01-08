// Aldaron's Window Interface
// Copyright (c) 2017 Jeron Aldaron Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE
//
// src/input/cursor.rs

#[derive(PartialEq)]
#[derive(Copy, Clone)]
pub enum Click {
	Left,
	Middle,
	Right,
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
