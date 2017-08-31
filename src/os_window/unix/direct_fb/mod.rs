// Aldaron's Window Interface
// Copyright (c) 2017 Plop Grizzly, Jeron Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE
//
// src/os_window/unix/direct_fb/mod.rs

use Input;

pub struct FBWindow {
	// TODO
}

impl FBWindow {
	pub fn poll_event(&self, input: &mut Vec<Input>, wh: &mut (u32, u32))
		-> bool
	{
		// TODO: this breaks
		false
	}
}
