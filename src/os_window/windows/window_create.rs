// "awi" crate - Licensed under the MIT LICENSE
//  * Copyright (c) 2017-2018  Jeron A. Lau <jeron.lau@plopgrizzly.com>

use c_void;
use super::types::*;
use std::ptr::null;

use winapi::um::winuser::{
	WS_OVERLAPPEDWINDOW, WS_VISIBLE, WS_SYSMENU,
	CreateWindowExW, AdjustWindowRect, RECT
};

const WS_FLAGS : u32 = WS_OVERLAPPEDWINDOW | WS_VISIBLE | WS_SYSMENU;

pub fn window_create(connection: *const c_void, size: (isize, isize),
	name: [u8; 80]) -> Hwnd
{
	let mut wr = RECT { left: 0, top: 0, right: size.0, bottom: size.1 };
	unsafe {
		AdjustWindowRect(&mut wr, WS_OVERLAPPEDWINDOW, 0)
	};
	let width = wr.right - wr.left;
	let height = wr.bottom - wr.top;

	let window = unsafe { CreateWindowExW(0,
		&name,		// class name
		&name,		// app name
		WS_FLAGS,	// window style
		0, 0,		// x/y coords
		width as i32,	// width
		height as i32,	// height
		null(),	// handle to parent
		null(),	// handle to menu
		connection,	// hInstance
		null())	// no extra parameters
	};
	if window == Hwnd::null() {
		panic!("Couldn't Create a Window!");
	}
	window
}
