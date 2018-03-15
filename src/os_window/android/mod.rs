// os_window/android/mod.rs -- Aldaron's Window Interface
// Copyright (c) 2017-2018  Jeron A. Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE

mod input;
mod glue;

pub use self::input::key;
pub use self::glue::gsp_main;

pub struct AndroidWindow {
}

impl ::WindowOps for AndroidWindow {
	fn create(title: &str, icon: (u32, u32, &[u8])) -> Self {
		AndroidWindow { }
	}

	fn show(&self) -> () {

	}

	fn update(&self) -> () {

	}

	fn poll_event(&self, input: &mut ::input::InputQueue, wh: &mut(u32,u32))
		-> bool
	{
		false
	}

	fn fullscreen(&self) -> () {

	}

	fn get_connection(&self) -> ::WindowConnection {
		::WindowConnection::Android
	}
}
