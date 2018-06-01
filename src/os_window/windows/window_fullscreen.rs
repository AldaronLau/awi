// "awi" crate - Licensed under the MIT LICENSE
//  * Copyright (c) 2017-2018  Jeron A. Lau <jeron.lau@plopgrizzly.com>

use std::ptr::null_mut;
use super::types::*;

use winapi::um::winuser::{
	RECT, GetWindowRect, GetWindowLongW, SetWindowLongW, SetWindowPos, 
	GetSystemMetrics, WS_VISIBLE, HWND_NOTOPMOST
};
use winapi::shared::windef::HWND;

pub fn window_fullscreen(window: HWND, state: &mut bool,
	size: &mut (i32, i32, i32, i32), style: &mut usize)
{
	let flags = 0x0040 | 0x0020;

	if *state {
		unsafe {
			SetWindowLongW(window, -16, *style);
			SetWindowPos(window, HWND_NOTOPMOST, size.0, size.1,
				size.2, size.3, flags);
		}
	} else {
		let mut rc = RECT { left: 0, top: 0, bottom: 0, right: 0 };
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
