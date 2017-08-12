// mod.rs
// Window Demo
// Copyright 2017 (c) Aldaron's Tech
// Copyright 2017 (c) Jeron Lau
// Licensed under the MIT LICENSE

#[macro_use]
extern crate window;
extern crate aci_png;

use window::{ Window, InputQueue };

pub fn main() -> () {
	let mut window = connect!();
	let mut queue = InputQueue::new();

	'mainloop: loop {
		window.update(&mut queue);

		for input in queue.iter() {
			use window::Input::*;
			use window::Msg::*;

			match *input {
				Msg(Quit) | Msg(Back) => break 'mainloop,
	//			Input::Redraw => redraw(&mut context),
				_ => {},
			}
		}
	}
}
