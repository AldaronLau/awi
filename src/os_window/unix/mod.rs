// "awi" crate - Licensed under the MIT LICENSE
//  * Copyright (c) 2017-2018  Jeron A. Lau <jeron.lau@plopgrizzly.com>

mod input;

mod xcb;
// mod wayland; // TODO: link to in runtime only if it's installed.
// mod direct_fb;

pub use self::input::key;

pub enum UnixWindow {
//	Wayland(),
	Xcb(xcb::XcbWindow),
//	DirectFb(direct_fb::FBWindow),
}

impl ::WindowOps for UnixWindow {
	fn new(title: &str, icon: (u32, u32, &[u32]), v: Option<i32>) -> Self {
		let xcb = xcb::XcbWindow::new(title, icon, v);

		if xcb.failed() {
			println!("WARNING: Either XCB is uninstalled or no X.");
		} else {
			return UnixWindow::Xcb(xcb);
		}

		println!("WARNING: No wayland support yet.");
		println!("WARNING: No directfb support yet.");
		panic!("None of the unix backends [xcb,] found!");
	}

	fn show(&self) -> () {
		match *self {
//			UnixWindow::Wayland(w) => w.show(),
			UnixWindow::Xcb(ref w) => w.show(),
//			UnixWindow::DirectFb(ref w) => w.show(),
		}
	}

	fn update(&self) -> () {
		match *self {
//			UnixWindow::Wayland(w) => w.update(),
			UnixWindow::Xcb(ref w) => w.update(),
//			UnixWindow::DirectFb(ref w) => w.update(),
		}
	}

	fn poll_event(&mut self, input: &mut ::input::InputQueue,
		wh: &mut(u32,u32), keyboard: &mut ::Keyboard) -> bool
	{
		let r = match *self {
//			UnixWindow::Wayland(w) => w.poll_event(input, wh),
			UnixWindow::Xcb(ref mut w) => w.poll_event(input, wh, keyboard),
//			UnixWindow::DirectFb(ref w) => w.poll_event(input, wh),
		};

		r
	}

	fn fullscreen(&mut self) -> () {
		match *self {
//			UnixWindow::Wayland(w) => w.fullscreen(),
			UnixWindow::Xcb(ref mut w) => w.fullscreen(),
//			UnixWindow::DirectFb(ref w) => w.fullscreen(),
		}
	}

	fn get_connection(&self) -> ::WindowConnection {
		match *self {
//			UnixWindow::Wayland(w) => w.get_connection(),
			UnixWindow::Xcb(ref w) => w.get_connection(),
//			UnixWindow::DirectFb(ref w) => w.get_connection(),
		}
	}
}
