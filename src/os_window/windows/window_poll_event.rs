// os_window/windows/window_poll_event.rs -- Aldaron's Window Interface
// Copyright (c) 2017-2018  Jeron A. Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE

use input;
use libc::c_void;
// use input::keyboard::{ english, FSC, ESC }; TODO
use Key;
use super::types::*;

static mut ADI_WNDPROCMSG : u8 = 0b0000_0000;
static mut AWI_DIMENSIONS: (u32, u32) = (0, 0);

const RESIZED : u8 = 0b1000_0000;
const PAUSED : u8 = 0b0100_0000;
const RESUMED : u8 = 0b0010_0000;

// const KEY_DOWN: u32 = 2;
// const KEY_UP: u32 = 3;
const CURSOR_MOVE: u32 = 0x0200;
// const CURSOR_ENTER: u32 = 7;
// const CURSOR_LEAVE: u32 = 8;
// const GAIN_FOCUS: u32 = 9;
// const LOSE_FOCUS: u32 = 10;
// const WINDOW_RESIZE: u32 = 22;
const WINDOW_CLOSE: u32 = 0x0012;

const LBUTTON_DOWN: u32 = 0x0201;
const LBUTTON_UP: u32 = 0x0202;
const MBUTTON_DOWN: u32 = 0x0207;
const MBUTTON_UP: u32 = 0x0208;
const RBUTTON_DOWN: u32 = 0x0204;
const RBUTTON_UP: u32 = 0x0205;
const SCROLL: u32 = 0x020A;
const CHAR: u32 = 0x0102;
const SYSCHAR: u32 = 0x0106;

const LSHIFT : u32 = 16;
const RSHIFT : u32 = 16 | (0b_0011_0110 << 16);

#[repr(C)]
struct Point {
	x: i32, // long
	y: i32, // long
}

#[repr(C)]
struct Msg {
	hwnd: Hwnd,
	message: u32,
	w_param: Wparam,
	l_param: Lparam,
	time: u32,
	pt: Point,
}

extern "system" {
	fn PeekMessageW(lpMsg: *mut Msg, h_wnd: Hwnd, msg_filter_min: u32,
		msg_filter_max: u32, remove_msg: u32) -> Bool;
	fn TranslateMessage(lpMsg: *const Msg) -> Bool;
	fn DispatchMessageW(lpMsg: *const Msg) -> Lresult;
	fn GetCursorPos(point: *mut Point) -> Bool;
	fn ScreenToClient(h_wnd: Hwnd, point: *mut Point) -> Bool;
	fn PostQuitMessage(exit_code: i32) -> ();
	fn DefWindowProcW(hw: Hwnd, uMsg: u32, wParam: *const c_void,
		lParam: *const c_void) -> Lresult;
}

pub extern "C" fn wnd_proc(h_wnd: Hwnd, u_msg: u32, w_param: *const c_void,
	l_param: *const c_void) -> Lresult
{
	match u_msg {
		0x0007 => unsafe { ADI_WNDPROCMSG |= RESUMED },
		0x0008 => unsafe { ADI_WNDPROCMSG |= PAUSED },
		0x0010 => {
			unsafe { PostQuitMessage(0) }; // Successful exit
			return 1; // TRUE = Don't Close Window Yet
		},
		0x0005 => {
			unsafe { ADI_WNDPROCMSG |= RESIZED };

			let i = unsafe { ::std::mem::transmute::<*const c_void, usize>(l_param) };
			let h = (i & 0xFFFF_0000) / 0x1_0000;
			let w = i & 0x0000_FFFF;

			unsafe { AWI_DIMENSIONS = (w as u32, h as u32); }

			return 0;
		},
		_ => {},
	}

	unsafe {
		DefWindowProcW(h_wnd, u_msg, w_param, l_param)
	}
}

fn get_mouse(window: Hwnd, wh: &(u32, u32), is_miw: &mut bool)
	-> (i16, i16, bool)
{
	let mut pos = Point { x: 0, y: 0 };
	unsafe {
		GetCursorPos(&mut pos);
		ScreenToClient(window, &mut pos);
	}

	let miw = pos.x >= 0 && pos.x as isize <= (*wh).0 as isize && pos.y as isize >= 0
		&& pos.y as isize <= (*wh).1 as isize;

	let miw_changed = if *is_miw != miw {
		*is_miw = miw;
		true
	} else {
		false
	};
	(pos.x as i16, pos.y as i16, miw_changed)
}

fn create_key_id(w: Wparam, l: Lparam) -> u32 {
	const RCONTROL : u32 = 17 | (0b_1_0001_1101 << 16);
	const RALT: u32 = 18 | (0b_1_0011_1000 << 16);

	let scan = l & 0b00000001_11111111_00000000_00000000;
	
	match (w as u32) | ( scan as u32) {
		d @ RSHIFT => d,
		d @ RCONTROL => d,
		d @ RALT => d,
		_ => (w as u32),
	}
}

pub fn window_poll_event(window: Hwnd, queue: &mut input::InputQueue,
	miw: &mut bool, wh: &mut (u32, u32), keyboard: &mut ::Keyboard) -> bool
{
	let mut msg = Msg { hwnd: Hwnd::null(), message: 0, w_param: 0, l_param: 0,
		time: 0, pt: Point { x: 0, y: 0 } };

	if unsafe { ADI_WNDPROCMSG & RESIZED != 0 } {
		println!("RESIZE {:?}", unsafe { AWI_DIMENSIONS });
		queue.resize(wh, unsafe { AWI_DIMENSIONS });
		unsafe { ADI_WNDPROCMSG &= !RESIZED };
		return true;
	}
	
	if unsafe { ADI_WNDPROCMSG & PAUSED != 0 } {
		queue.pause();
		unsafe { ADI_WNDPROCMSG &= !PAUSED };
		return true;
	}
	
	if unsafe { ADI_WNDPROCMSG & RESUMED != 0 } {
		queue.resume();
		unsafe { ADI_WNDPROCMSG &= !RESUMED };
		return true;
	}

	let (x, y, miw_changed) = get_mouse(window, wh, miw);
	if miw_changed {
		if *miw == false {
			queue.cursor_leave();
		}
		return true;
	}

	if unsafe {
		PeekMessageW(&mut msg, Hwnd::null(), 0, 0, 0x0001)
	} == 0 { // no messages available
		return false;
	}
	match msg.message {
		WINDOW_CLOSE => queue.back(),
		CURSOR_MOVE => queue.cursor_move(*wh, (x, y)),
		LBUTTON_DOWN => queue.button_down(*wh, (x, y), ::Click::Left),
		LBUTTON_UP => queue.button_up(*wh, (x, y), ::Click::Left),
		MBUTTON_DOWN => queue.button_down(*wh, (x, y), ::Click::Middle),
		MBUTTON_UP => queue.button_up(*wh, (x, y), ::Click::Middle),
		RBUTTON_DOWN => queue.button_down(*wh, (x, y), ::Click::Right),
		RBUTTON_UP => queue.button_up(*wh, (x, y), ::Click::Right),
		0x0100 | 0x0104 => {
			let detail = create_key_id(msg.w_param, msg.l_param);
			
			if let Some(key) = Key::new(detail) {
				keyboard.press(key);

				// Required to generate CHAR & SYSCHAR
				unsafe { TranslateMessage(&msg); }
			} else if detail == ::os_window::key::lib::ESCAPE {
				queue.back();
			}
		}
		0x0101 | 0x0105 => {
			let detail = create_key_id(msg.w_param, msg.l_param);

			// A workaround for a bug in windows, when RSHIFT is
			// released while LSHIFT is still pressed, there is no
			// separate release event for LSHIFT.
			if detail == RSHIFT {
				keyboard.release(Key::LShift);
			}
			// And vice versa
			if detail == LSHIFT {
				keyboard.release(Key::RShift);
			}

			if let Some(key) = Key::new(detail) {
				keyboard.release(key);
			}

			// Required to generate CHAR & SYSCHAR
			unsafe { TranslateMessage(&msg); }
		}
		CHAR | SYSCHAR => {
			let c = msg.w_param as u16;
			
			queue.text(String::from_utf16(&[c]).unwrap());
		}
		SCROLL => {
			let a = (((msg.w_param as u32) >> 16) & 0xFFFF)
				as i16;

			if a > 0 {
				queue.scroll(*wh, (x, y),
					input::ScrollWheel::Up,
					(a / 120) as usize);
			} else {
				queue.scroll(*wh, (x, y),
					input::ScrollWheel::Down,
					(a / -120) as usize);
			}
		}
		// ignore all other messages
		_ => {
			unsafe {
				TranslateMessage(&msg);
				DispatchMessageW(&msg);
			}
//			println!("Unknown event: {0:x}", x);
		}
	};

	true
}