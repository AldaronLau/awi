// "awi" - Aldaron's Window Interface
//
// Copyright Jeron A. Lau 2017-2018.
// Distributed under the Boost Software License, Version 1.0.
// (See accompanying file LICENSE_1_0.txt or copy at
// https://www.boost.org/LICENSE_1_0.txt)

extern crate awi;
extern crate aci_png;

use awi::Window;

pub fn main() -> () {
	let mut window = Window::new(
		"awi example",
		&aci_png::decode(include_bytes!("../res/icon.png")).unwrap(),
		None
	);

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
