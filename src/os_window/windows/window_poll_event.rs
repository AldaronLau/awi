// Aldaron's Window Interface
// Copyright (c) 2017 Plop Grizzly, Jeron Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE
//
// src/os_window/windows/window_poll_event.rs

use input;
use ami::Void;
// use input::keyboard::{ english, FSC, ESC }; TODO
use Key;
use super::types::*;

static mut ADI_WNDPROCMSG : u8 = 0b0000_0000;

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

#[repr(C)]
struct Point {
	x: isize, // long
	y: isize, // long
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

#[repr(C)]
#[derive(Copy, Clone)]
struct Rect {
	left: isize,
	top: isize,
	right: isize,
	bottom: isize,
}

extern "system" {
	fn PeekMessageW(lpMsg: *mut Msg, h_wnd: Hwnd, msg_filter_min: u32,
		msg_filter_max: u32, remove_msg: u32) -> i32;
	fn TranslateMessage(lpMsg: *const Msg) -> i32;
	fn DispatchMessageW(lpMsg: *const Msg) -> usize;
	fn GetCursorPos(point: *mut Point) -> i32;
	fn ScreenToClient(h_wnd: Hwnd, point: *mut Point) -> i32;
	fn GetClientRect(h_wnd: Hwnd, out: *mut Rect) -> i32;
	fn PostQuitMessage(exit_code: i32) -> ();
	fn DefWindowProcW(hw: Hwnd, uMsg: u32, wParam: *const Void,
		lParam: *const Void) -> isize;
}

pub extern "C" fn wnd_proc(h_wnd: Hwnd, u_msg: u32, w_param: *const Void,
	l_param: *const Void) -> isize
{
	match u_msg {
		0x0007 => unsafe { ADI_WNDPROCMSG |= RESUMED },
		0x0008 => unsafe { ADI_WNDPROCMSG |= PAUSED },
		0x0010 => {
			unsafe { PostQuitMessage(0) }; // Successful exit
			return 1; // TRUE = Don't Close Window Yet
		},
		0x0024 => {
			unsafe { ADI_WNDPROCMSG |= RESIZED };
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

	let miw = pos.x >= 0 && pos.x <= (*wh).0 as isize && pos.y >= 0
		&& pos.y <= (*wh).1 as isize;

	let miw_changed = if *is_miw != miw {
		*is_miw = miw;
		true
	} else {
		false
	};
	(pos.x as i16, pos.y as i16, miw_changed)
}

pub fn window_poll_event(window: Hwnd, queue: &mut input::InputQueue,
	miw: &mut bool, wh: &mut (u32, u32), keyboard: &mut ::Keyboard) -> bool
{
	let mut msg = Msg { hwnd: Hwnd::null(), message: 0, w_param: 0, l_param: 0,
		time: 0, pt: Point { x: 0, y: 0 } };

	if unsafe { ADI_WNDPROCMSG & RESIZED != 0 } {
		let clia = {
			let mut clia = Rect { left: 0, right: 0, top: 0, bottom: 0 };
			unsafe {
				GetClientRect(window, &mut clia);
			}
			println!("LT: {} {}", clia.left, clia.top);
			(clia.right as u32, clia.bottom as u32)
		};

//		if should_resize(wh, clia) {
			queue.resize(wh, clia);
//		}
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
			// TODO
//			let scan = ((msg.l_param
//				& 0b00000001_11111111_00000000_00000000) >> 16)
//					as u16;
			if msg.l_param & 0b01000000_00000000_00000000_00000000
				!= 0
			{
				// TODO
				/*
				let chr = english(msg.w_param as u16, scan);

				match chr {
					// These keys shouldn't repeat.
					Key::Char(ESC) | Key::Char(FSC) |
						Key::Insert | Key::Compose |
						Key::NumLock | Key::Shift(_) |
						Key::Ctrl(_) | Key::Alt(_)
					=> { } // ignore
					_ => input.push(Input::KeyRepeat(chr))
				}*/
			} else if let Some(key) = Key::new(msg.w_param as u32) {
				keyboard.press(key);
			}
		}
		0x0101 | 0x0105 => {
			// TODO
//			let scan = ((msg.l_param
//				& 0b00000001_11111111_00000000_00000000) >> 16);
//			let chr = english(msg.w_param as u16, scan);
			
			if let Some(key) = Key::new(msg.w_param as u32) {
				keyboard.press(key);
			}
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