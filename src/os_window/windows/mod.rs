// Aldaron's Window Interface
// Copyright (c) 2017 Plop Grizzly, Jeron Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE
//
// src/os_window/windows/mod.rs

mod types;
mod input;
mod class_create;
mod connection_create;
mod string; // for UTF-16 conversions
mod window_create;
mod window_fullscreen;
mod window_poll_event;

use ami::Void;
use input::InputQueue;
use self::types::*;

pub use self::input::key;

struct Connection { native: *mut Void }
impl Connection {
	fn create() -> Connection {
		Connection { native: connection_create::connection_create() }
	}
}
struct Class { name: [u8; 80] }
impl Class {
	fn create(connection: &Connection, name: &str,
		image: (u32, u32, &[u32]), wnd_proc: extern "C" fn(
			a: Hwnd, b: u32, c: *const Void, d: *const Void)
			-> Lresult)
		-> Class
	{
		Class {
			name: class_create::class_create(connection.native,
				name, image, wnd_proc)
		}
	}
}
struct Window { native: Hwnd }
impl Window {
	fn create(connection: &Connection, size: (isize, isize), class: Class) -> Window {
		let c = connection.native;
		let name = class.name;

		Window { native: window_create::window_create(c, size, name) }
	}
}

pub struct WindowsWindow {
	window: Window,
	connection: Connection,
	miw: bool, // Mouse In Window
	restore_size: (i32, i32, i32, i32),
	fullscreen: bool,
	restore_style: usize,
}
impl ::WindowOps for WindowsWindow {
	fn new(title: &str, icon: (u32, u32, &[u32])) -> WindowsWindow {
		let connection = Connection::create();
		let class = Class::create(&connection, title, icon,
			window_poll_event::wnd_proc);
		let window = Window::create(&connection,
			(::MWW as isize, ::MWH as isize), class);

		WindowsWindow { connection: connection, window: window, miw: true,
			restore_size: (0, 0, 0, 0),
			fullscreen: false, restore_style: 0,
		}
	}
	
	fn show(&self) -> () {
	}

	fn fullscreen(&mut self) {
		window_fullscreen::window_fullscreen(self.window.native,
			&mut self.fullscreen, &mut self.restore_size,
			&mut self.restore_style);
	}

	fn poll_event(&mut self, input: &mut InputQueue, wh: &mut (u32, u32),
		keyboard: &mut ::Keyboard) -> bool
	{
		let miw = &mut self.miw;
		let window = self.window.native;

		window_poll_event::window_poll_event(window, input, miw, wh,
			keyboard)
	}

	fn update(&self) {
	}

	fn get_connection(&self) -> ::WindowConnection {
		::WindowConnection::Windows(self.connection.native,
			self.window.native.to_ptr())
	}
}