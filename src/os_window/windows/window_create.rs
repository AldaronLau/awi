// Aldaron's Window Interface
// Copyright (c) 2017 Jeron Aldaron Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE
//
// src/os_window/windows/window_create.rs

use ami::Void;
use super::types::*;

const WS_OVERLAPPEDWINDOW : u32 = 0x00C00000 | 0x00080000 | 0x00040000
	| 0x00010000 | 0x00020000;
const WS_VISIBLE : u32 = 0x10000000;
const WS_SYSMENU : u32 = 0x00080000;
const WS_FLAGS : u32 = WS_OVERLAPPEDWINDOW | WS_VISIBLE | WS_SYSMENU;

#[repr(C)]
#[derive(Copy, Clone)]
struct Rect {
	left: isize,
	top: isize,
	right: isize,
	bottom: isize,
}

extern "system" {
	fn CreateWindowExW(a: u32, class_name: *const [u8;80],
		window_name: *const [u8;80], style: u32, x: i32, y: i32,
		w: i32, h: i32, parent: *const Void, menu: *const Void,
		hInstance: *const Void, param: *const Void) -> Hwnd;
	fn AdjustWindowRect(a: *mut Rect, dwStyle: u32, bMenu: i32) -> i32;
}

pub fn window_create(connection: *const Void, size: (isize, isize), name: [u8; 80]) -> Hwnd {
	let mut wr = Rect { left: 0, top: 0, right: size.0, bottom: size.1 };
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
		null!(),	// handle to parent
		null!(),	// handle to menu
		connection,	// hInstance
		null!())	// no extra parameters
	};
	if window == Hwnd::null() {
		panic!("Couldn't Create a Window!");
	}
	window
}
