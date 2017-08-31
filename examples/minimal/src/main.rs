// Aldaron's Window Interface
// Copyright (c) 2017 Plop Grizzly, Jeron Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE
//
// examples/minimal/src/main.rs

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
