// Copyright Jeron A. Lau 2017-2018.
// Dual-licensed under either the MIT License or the Boost Software License,
// Version 1.0.  (See accompanying file LICENSE_1_0.txt or copy at
// https://www.boost.org/LICENSE_1_0.txt)

use super::{ string };
use std::ptr::{ null, null_mut };

use std::mem;

use winapi::um::winuser::{
	RegisterClassExW, WNDCLASSEXW, LoadCursorW, CreateIcon, IDC_ARROW
};
use winapi::um::wingdi::{
	GetStockObject,
};
use winapi::shared::windef::HWND;
use winapi::shared::minwindef::{ WPARAM, LPARAM, LRESULT, HINSTANCE, UINT };

pub fn class_create(hi: HINSTANCE, title: &str,
	wnd_proc: extern "system" fn(a: HWND, b: u32, c: WPARAM,
		d: LPARAM) -> LRESULT)
	-> [u8; 80]
{
	let mut name : [u8; 80] = [0u8; 80];
	let nam = string::native(title);

	for i in 0..nam.len() {
		name[i] = nam[i];
	}

	let (w, h, pixels) = (1, 1, &[0xFF_FF_FF_FF]);

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
		CreateIcon(hi, w as i32, h as i32, 1, 32,
			&and[0] as *const _ as *const _,
			&xor[0] as *const _ as *const _)
	};
	
	let window_class = WNDCLASSEXW {
		cbSize: mem::size_of::<WNDCLASSEXW>() as UINT,
		style: 0x0002 | 0x0001,
		lpfnWndProc: Some(wnd_proc),
		cbClsExtra: 0,
		cbWndExtra: 0,
		hInstance: hi,
		hIcon: new_icon,
		hCursor: unsafe { LoadCursorW(null_mut(), IDC_ARROW) }, // TODO: use undeprecated function?
		hbrBackground: unsafe { GetStockObject(0) } as *mut _,
		lpszMenuName: null(),
		lpszClassName: &name as *const _ as *const _,
		hIconSm: new_icon,
	};
	
	if unsafe { RegisterClassExW(&window_class) } == 0 {
		panic!("Failed to register windows class.");
	}
	
	name
}
