// main.rs -- Aldaron's Window Interface
// Copyright (c) 2017-2018  Jeron A. Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE

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
