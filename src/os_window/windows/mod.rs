// "awi" crate - Licensed under the MIT LICENSE
//  * Copyright (c) 2017-2018  Jeron A. Lau <jeron.lau@plopgrizzly.com>

mod input;
mod class_create;
mod connection_create;
mod string; // for UTF-16 conversions
mod window_create;
mod window_fullscreen;
mod window_poll_event;

use c_void;
use input::InputQueue;

pub use self::input::key;

use winapi::shared::windef::HWND;
use winapi::um::winnt::LONG;
use winapi::shared::minwindef::{ WPARAM, LPARAM, LRESULT, HINSTANCE };

struct Connection { native: HINSTANCE }
impl Connection {
	fn create() -> Connection {
		Connection { native: connection_create::connection_create() }
	}
}
struct Class { name: [u8; 80] }
impl Class {
	fn create(connection: &Connection, name: &str,
		image: (u32, u32, &[u32]), wnd_proc: extern "system" fn(
			a: HWND, b: u32, c: WPARAM, d: LPARAM)
			-> LRESULT)
		-> Class
	{
		Class {
			name: class_create::class_create(connection.native,
				name, image, wnd_proc)
		}
	}
}
struct Window { native: HWND }
impl Window {
	fn create(connection: &Connection, size: (LONG, LONG), class: Class) -> Window {
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
	restore_style: LONG,
}
impl ::WindowOps for WindowsWindow {
	fn new(title: &str, icon: (u32, u32, &[u32]), _v: Option<i32>)
		-> WindowsWindow
	{
		let connection = Connection::create();
		let class = Class::create(&connection, title, icon,
			window_poll_event::wnd_proc);
		let window = Window::create(&connection,
			(::MWW as LONG, ::MWH as LONG), class);

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
		::WindowConnection::Windows(self.connection.native as *mut c_void,
			self.window.native as *mut c_void)
	}
}
