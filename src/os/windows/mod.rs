// Copyright Jeron A. Lau 2017-2018.
// Dual-licensed under either the MIT License or the Boost Software License,
// Version 1.0.  (See accompanying file LICENSE_1_0.txt or copy at
// https://www.boost.org/LICENSE_1_0.txt)

mod class_create;
mod connection_create;
mod string; // for UTF-16 conversions
mod window_create;
mod window_fullscreen;
mod window_poll_event;

use winapi::ctypes::c_int;
use input::keyboard;
use c_void;
use input::InputQueue;

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
		wnd_proc: extern "system" fn(
			a: HWND, b: u32, c: WPARAM, d: LPARAM)
			-> LRESULT)
		-> Class
	{
		Class {
			name: class_create::class_create(connection.native,
				name, wnd_proc)
		}
	}
}
struct NativeWindow { native: HWND }
impl NativeWindow {
	fn create(connection: &Connection, class: Class) -> NativeWindow {
		let c = connection.native;
		let name = class.name;

		NativeWindow { native: window_create::window_create(c, name) }
	}
}

pub struct Window {
	window: NativeWindow,
	connection: Connection,
	miw: bool, // Mouse In Window
	restore_size: (i32, i32, i32, i32),
	fullscreen: bool,
	restore_style: LONG,
	wh: (u16, u16),
}
impl Window {
	pub fn new(_title: &str, _icon: &::afi::Video, _v: Option<i32>) -> Self
	{
		let connection = Connection::create();
		let class = Class::create(&connection, _title,
			window_poll_event::wnd_proc);
		let window = NativeWindow::create(&connection,
			class);

		Window { connection: connection, window: window, miw: true,
			restore_size: (0, 0, 0, 0),
			fullscreen: false, restore_style: 0, wh: (640, 360),
		}
	}

/*	fn fullscreen(&mut self) {
		window_fullscreen::window_fullscreen(self.window.native,
			&mut self.fullscreen, &mut self.restore_size,
			&mut self.restore_style);
	}*/

	pub fn poll_event(&mut self, input: &mut InputQueue,
		keyboard: &mut ::Keyboard) -> bool
	{
		let miw = &mut self.miw;
		let window = self.window.native;

		window_poll_event::window_poll_event(window, input, miw,
			keyboard, &mut self.wh)
	}

	pub fn get_connection(&self) -> ::WindowConnection {
		::WindowConnection::Windows(self.connection.native as *mut c_void,
			self.window.native as *mut c_void)
	}
	
	pub fn wh(&self) -> (u16, u16) {
		self.wh
	}
}

// Keycode translator
fn key(physical_key: c_int) -> Option<u8> {
	const RCTRL: c_int = (17 | (0b_1_0001_1101 << 16));
	const RSHIFT: c_int = (16 | (0b_0011_0110 << 16));
	const ALT_GR: c_int = (18 | (0b_1_0011_1000 << 16));

	Some(match physical_key {
		192 => keyboard::EXT_BACKTICK,
		107 => keyboard::EXT_PLUS,
		106 => keyboard::EXT_ASTERISK,
		111 | 191 => keyboard::SLASH,
		13 => keyboard::ENTER,
		49 | 97 => keyboard::NUM1,
		50 | 98 => keyboard::NUM2,
		51 | 99 => keyboard::NUM3,
		52 | 100 => keyboard::NUM4,
		53 | 101 => keyboard::NUM5,
		54 | 102 => keyboard::NUM6,
		55 | 103 => keyboard::NUM7,
		56 | 104 => keyboard::NUM8,
		57 | 105 => keyboard::NUM9,
		48 | 96 => keyboard::NUM0,
		190 | 110 => keyboard::PERIOD,
		189 | 109 => keyboard::MINUS,
		187 => keyboard::EQUAL_SIGN,
		8 => keyboard::BACKSPACE,
		9 => keyboard::TAB,
		65 => keyboard::A,
		66 => keyboard::B,
		67 => keyboard::C,
		68 => keyboard::D,
		69 => keyboard::E,
		70 => keyboard::F,
		71 => keyboard::G,
		72 => keyboard::H,
		73 => keyboard::I,
		74 => keyboard::J,
		75 => keyboard::K,
		76 => keyboard::L,
		77 => keyboard::M,
		78 => keyboard::N,
		79 => keyboard::O,
		80 => keyboard::P,
		81 => keyboard::Q,
		82 => keyboard::R,
		83 => keyboard::S,
		84 => keyboard::T,
		85 => keyboard::U,
		86 => keyboard::V,
		87 => keyboard::W,
		88 => keyboard::X,
		89 => keyboard::Y,
		90 => keyboard::Z,
		219 => keyboard::BRACKET_OPEN,
		221 => keyboard::BRACKET_CLOSE,
		17 => keyboard::LCTRL,
		RCTRL => keyboard::RCTRL,
		16 => keyboard::LSHIFT,
		RSHIFT => keyboard::RSHIFT,
		18 => keyboard::ALT,
		ALT_GR => keyboard::EXT_ALT_GR,
		20 => keyboard::COMPOSE,
		186 => keyboard::SEMICOLON,
		222 => keyboard::APOSTROPHE,
		220 => keyboard::BACKSLASH,
		188 => keyboard::COMMA,
		32 => keyboard::SPACE,
		144 => keyboard::EXT_NUM_LOCK,
		36 => keyboard::EXT_HOME,
		35 => keyboard::EXT_END,
		33 => keyboard::EXT_PAGE_UP,
		34 => keyboard::EXT_PAGE_DOWN,
		45 => keyboard::EXT_INSERT,
		46 => keyboard::EXT_DELETE,
		38 => keyboard::UP,
		37 => keyboard::LEFT,
		39 => keyboard::RIGHT,
		40 => keyboard::DOWN,
		_ => return None,
	} )
}