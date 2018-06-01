// "awi" crate - Licensed under the MIT LICENSE
//  * Copyright (c) 2017-2018  Jeron A. Lau <jeron.lau@plopgrizzly.com>

use c_void;
use super::{ string };
use super::types::*;
use std::ptr::null;

use std::mem;

use winapi::um::winuser::{
	RegisterClassExW, WNDCLASSEXW, LoadCursorW, CreateIcon,
};
use winapi::um::wingdi::{
	GetStockObject,
};

pub fn class_create(hi: *const c_void, title: &str, icon: (u32, u32, &[u32]),
	wnd_proc: extern "C" fn(a: Hwnd, b: u32, c: *const c_void,
		d: *const c_void) -> Lresult)
	-> [u8; 80]
{
	let mut name : [u8; 80] = [0u8; 80];
	let nam = string::native(title);

	for i in 0..nam.len() {
		name[i] = nam[i];
	}

	let (w, h, pixels) = icon;

	let mut and : Vec<u32> = Vec::new();
	let mut xor : Vec<u32> = Vec::new();

	let w = w as usize;
	let h = h as usize;

	for i in 0usize..w {
		for j in 0usize..h {
			// TODO
			// Xor
			xor.push(pixels[(j + (h * i))]);
			// And
			and.push(0xFF_FF_FF_FF);
		}
	}

	let new_icon = unsafe {
		CreateIcon(hi, w as i32, h as i32, 1, 32, &and[0], &xor[0])
	};
	
	let window_class = WNDCLASSEXW {
		cbSize: mem::size_of::<WndClassEx>() as UINT,
		style: 0x0002 | 0x0001,
		lpfnWndProc: wnd_proc,
		cbClsExtra: 0,
		cbWndExtra: 0,
		hInstance: hi,
		hIcon: new_icon,
		hCursor: unsafe { LoadCursorW(null(), 32512),
		hbrBackground: unsafe { GetStockObject(0) },
		lpszMenuName: 0,
		lpszClassName: &name,
		hIconSm: new_icon,
	};
	
	if unsafe { RegisterClassExW(&window_class) } == 0 {
		panic!("Failed to register windows class.");
	}
	
	name
}
