// "awi" crate - Licensed under the MIT LICENSE
//  * Copyright (c) 2017-2018  Jeron A. Lau <jeron.lau@plopgrizzly.com>

use std::ptr::null_mut;

use input;
// use input::keyboard::{ english, FSC, ESC }; TODO

use winapi::um::winuser::{
	WM_LBUTTONDOWN, WM_LBUTTONUP, WM_MBUTTONDOWN, WM_MBUTTONUP, WM_VSCROLL,
	WM_RBUTTONDOWN, WM_RBUTTONUP, WM_CLOSE, WM_MOUSEMOVE, WM_HSCROLL, 
	WM_CHAR, WM_SYSCHAR, VK_RSHIFT,
	PeekMessageW, TranslateMessage, DispatchMessageW, GetCursorPos,
	ScreenToClient, PostQuitMessage, DefWindowProcW,
	MSG,
};
use winapi::ctypes::c_int;
use winapi::shared::windef::{ HWND, POINT };
use winapi::shared::minwindef::{ WPARAM, LPARAM, LRESULT, HIWORD, LOWORD, DWORD };

static mut ADI_WNDPROCMSG : u8 = 0b0000_0000;
static mut AWI_DIMENSIONS: (u32, u32) = (0, 0);

const RESIZED: u8 = 0b1000_0000;
const PAUSED: u8 = 0b0100_0000;
const RESUMED: u8 = 0b0010_0000;

pub extern "system" fn wnd_proc(h_wnd: HWND, u_msg: u32, w_param: WPARAM,
	l_param: LPARAM) -> LRESULT
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

			let h = HIWORD(l_param as DWORD);
			let w = LOWORD(l_param as DWORD);

			unsafe { AWI_DIMENSIONS = (w as u32, h as u32); }

			return 0;
		},
		_ => {},
	}

	unsafe {
		DefWindowProcW(h_wnd, u_msg, w_param, l_param)
	}
}

fn get_mouse(window: HWND, wh: &(u32, u32), is_miw: &mut bool)
	-> (i16, i16, bool)
{
	let mut pos = POINT { x: 0, y: 0 };
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

fn create_key_id(w: WPARAM, l: LPARAM) -> c_int {
	const RCONTROL : c_int = 17 | (0b_1_0001_1101 << 16);
	const RALT: c_int = 18 | (0b_1_0011_1000 << 16);

	let scan = l & 0b00000001_11111111_00000000_00000000;
	
	match (w as c_int) | ( scan as c_int) {
		d @ VK_RSHIFT => d,
		d @ RCONTROL => d,
		d @ RALT => d,
		_ => (w as c_int),
	}
}

pub fn window_poll_event(window: HWND, queue: &mut input::InputQueue,
	miw: &mut bool, wh: &mut (u32, u32), keyboard: &mut ::Keyboard) -> bool
{
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

	let mut msg = MSG { hwnd: null_mut(), message: 0, wParam: 0, lParam: 0,
		time: 0, pt: POINT { x: 0, y: 0 } };
	if unsafe {
		PeekMessageW(&mut msg, null_mut(), 0, 0, 0x0001)
	} == 0 { // no messages available
		return false;
	}
	match msg.message {
		WM_CLOSE => queue.back(),
		WM_MOUSEMOVE => queue.cursor_move(*wh, (x, y)),
		WM_LBUTTONDOWN => queue.left_button_press(*wh, (x, y)),
		WM_LBUTTONUP => queue.left_button_release(*wh, (x, y)),
		WM_MBUTTONDOWN => queue.middle_button_press(*wh, (x, y)),
		WM_MBUTTONUP => queue.middle_button_release(*wh, (x, y)),
		WM_RBUTTONDOWN => queue.right_button_press(*wh, (x, y)),
		WM_RBUTTONUP => queue.right_button_release(*wh, (x, y)),
		0x0100 | 0x0104 => {
			let detail = create_key_id(msg.wParam, msg.lParam);
			
			if let Some(key) = input::key(detail) {
				keyboard.press(key);

				// Required to generate CHAR & SYSCHAR
				unsafe { TranslateMessage(&msg); }
			} else if detail == ::os_window::key::lib::ESCAPE {
				queue.back();
			}
		}
		0x0101 | 0x0105 => {
			let detail = create_key_id(msg.wParam, msg.lParam);

			// A workaround for a bug in windows, when RSHIFT is
			// released while LSHIFT is still pressed, there is no
			// separate release event for LSHIFT.
//			if detail == RSHIFT {
//				keyboard.release(input::key(LSHIFT).unwrap());
//			}
			// And vice versa
//			if detail == LSHIFT {
//				keyboard.release(input::key(RSHIFT).unwrap());
//			}

			if let Some(key) = input::key(detail) {
				keyboard.release(key);
			}

			// Required to generate CHAR & SYSCHAR
			unsafe { TranslateMessage(&msg); }
		}
		WM_CHAR | WM_SYSCHAR => {
			let c = msg.wParam as u16;
			
			queue.text(String::from_utf16(&[c]).unwrap());
		}
		WM_HSCROLL => {
			let a = (((msg.wParam as u32) >> 16) & 0xFFFF)
				as i16;

			if a > 0 {
				queue.scroll(*wh, (x, y),
					(a as f32 / -120.0, 0.0));
			} else {
				queue.scroll(*wh, (x, y),
					(a as f32 / 120.0, 0.0));
			}
		}
		WM_VSCROLL => {
			let a = (((msg.wParam as u32) >> 16) & 0xFFFF)
				as i16;

			if a > 0 {
				queue.scroll(*wh, (x, y),
					(0.0, a as f32 / -120.0));
			} else {
				queue.scroll(*wh, (x, y),
					(0.0, a as f32 / 120.0));
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
