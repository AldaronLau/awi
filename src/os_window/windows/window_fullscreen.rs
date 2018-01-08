// Aldaron's Window Interface
// Copyright (c) 2017 Jeron Aldaron Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE
//
// src/os_window/windows/window_fullscreen.rs

use super::types::*;

#[repr(C)]
#[derive(Copy, Clone)]
struct Rect {
	left: isize,
	top: isize,
	right: isize,
	bottom: isize,
}

const WS_VISIBLE : u32 = 0x10000000;

extern "system" {
	fn GetWindowRect(hw: Hwnd, out: *mut Rect) -> i32;
	//	fn SetWindowLongPtrW(hw: *const Void, n_index: i32, new: usize) -> usize; // 64-bit
	fn GetWindowLongW(hw: Hwnd, n_index: i32) -> usize;
	fn SetWindowLongW(hw: Hwnd, n_index: i32, new: usize) -> usize;
	fn SetWindowPos(hw: Hwnd, insert_after: Hwnd, x: i32,
		y: i32, w: i32, h: i32, flags: u32) -> i32;
	fn GetSystemMetrics(index: i32) -> i32;
}

pub fn window_fullscreen(window: Hwnd, state: &mut bool,
	size: &mut (i32, i32, i32, i32), style: &mut usize)
{
	let flags = 0x0040 | 0x0020;

	if *state {
		unsafe {
			SetWindowLongW(window, -16, *style);
			SetWindowPos(window, Hwnd::notopmost(), size.0, size.1,
				size.2, size.3, flags);
		}
	} else {
		let mut rc = Rect { left: 0, top: 0, bottom: 0, right: 0 };
		let w = unsafe { GetSystemMetrics(0) };
		let h = unsafe { GetSystemMetrics(1) };
		println!("w {} h {}", w, h);
		unsafe {
			*style = GetWindowLongW(window, -16);
			
			GetWindowRect(window, &mut rc);

			SetWindowLongW(window, -16, WS_VISIBLE as usize);
			SetWindowPos(window, Hwnd::topmost(), 0, 0, w, h, flags);
		}

		let sx = rc.left as i32;
		let sy = rc.top as i32;
		let sw = (rc.right - rc.left) as i32;
		let sh = (rc.bottom - rc.top) as i32;
		*size = (sx, sy, sw, sh);
	}
	*state = !*state;
}
