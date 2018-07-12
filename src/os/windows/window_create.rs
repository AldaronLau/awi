// "awi" - Aldaron's Window Interface
//
// Copyright Jeron A. Lau 2017-2018.
// Distributed under the Boost Software License, Version 1.0.
// (See accompanying file LICENSE_1_0.txt or copy at
// https://www.boost.org/LICENSE_1_0.txt)

use std::ptr::null_mut;

use winapi::um::winuser::{
	WS_OVERLAPPEDWINDOW, WS_VISIBLE, WS_SYSMENU,
	CreateWindowExW, AdjustWindowRect
};
use winapi::shared::windef::{ HWND, RECT };
use winapi::shared::minwindef::HINSTANCE;
use winapi::um::winnt::LONG;

const WS_FLAGS : u32 = WS_OVERLAPPEDWINDOW | WS_VISIBLE | WS_SYSMENU;

pub fn window_create(connection: HINSTANCE, size: (LONG, LONG),
	name: [u8; 80]) -> HWND
{
	let mut wr = RECT { left: 0, top: 0, right: size.0, bottom: size.1 };
	unsafe {
		AdjustWindowRect(&mut wr, WS_OVERLAPPEDWINDOW, 0)
	};
	let width = wr.right - wr.left;
	let height = wr.bottom - wr.top;

	let window = unsafe { CreateWindowExW(0,
		&name as *const _ as *const _,		// class name TODO: should be utf16?
		&name as *const _ as *const _,		// app name TODO: should be utf16?
		WS_FLAGS,	// window style
		0, 0,		// x/y coords
		width as i32,	// width
		height as i32,	// height
		null_mut(),	// handle to parent
		null_mut(),	// handle to menu
		connection,	// hInstance
		null_mut())	// no extra parameters
	};
	if window.is_null() {
		panic!("Couldn't Create a Window!");
	}
	window
}
