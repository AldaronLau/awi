// "awi" - Aldaron's Window Interface
//
// Copyright Jeron A. Lau 2017-2018.
// Distributed under the Boost Software License, Version 1.0.
// (See accompanying file LICENSE_1_0.txt or copy at
// https://www.boost.org/LICENSE_1_0.txt)

use winapi::um::winuser::{
	GetWindowRect, GetWindowLongW, SetWindowLongW, SetWindowPos, 
	GetSystemMetrics, WS_VISIBLE, HWND_NOTOPMOST, HWND_TOPMOST,
};
use winapi::shared::windef::{ HWND, RECT };
use winapi::um::winnt::LONG;

pub fn window_fullscreen(window: HWND, state: &mut bool,
	size: &mut (i32, i32, i32, i32), style: &mut LONG)
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

			SetWindowLongW(window, -16, WS_VISIBLE as LONG);
			SetWindowPos(window, HWND_TOPMOST, 0, 0, w, h, flags);
		}

		let sx = rc.left as i32;
		let sy = rc.top as i32;
		let sw = (rc.right - rc.left) as i32;
		let sh = (rc.bottom - rc.top) as i32;
		*size = (sx, sy, sw, sh);
	}
	*state = !*state;
}
