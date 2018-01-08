// Aldaron's Window Interface
// Copyright (c) 2017 Jeron Aldaron Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE
//
// examples/minimal/src/main.rs

#[macro_use]
extern crate awi;
extern crate aci_png;

use awi::Window;

pub fn main() -> () {
	let mut window = connect!();

	'mainloop: loop {
		while let Some(input) = window.input() {
			use awi::Input::*;
			use awi::Msg::*;

			match input {
				Msg(Quit) | Msg(Back) => break 'mainloop,
	//			Input::Redraw => redraw(&mut context),
				a => println!("{}", a),
			}
		}

		window.update();
	}
}
