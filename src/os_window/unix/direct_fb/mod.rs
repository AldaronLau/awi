// os_window/unix/direct_fb/mod.rs -- Aldaron's Window Interface
// Copyright (c) 2017-2018  Jeron A. Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE

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
