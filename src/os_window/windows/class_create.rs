// "awi" crate - Licensed under the MIT LICENSE
//  * Copyright (c) 2017-2018  Jeron A. Lau <jeron.lau@plopgrizzly.com>

use libc::c_void;
use super::{ string };
use super::types::*;
use std::ptr::null;

use std::mem;

#[repr(C)]
struct WndClassEx {
	cb_size: u32,
	style: u32,
	lpfn_wnd_proc: extern "C" fn(a: Hwnd, b: u32, c: *const c_void,
		d: *const c_void) -> Lresult,
	cb_cls_extra: i32,
	cb_wnd_extra: i32,
	h_instance: *const c_void,
	h_icon: *const c_void,
	h_cursor: *const c_void,
	hbr_background: *const c_void,
	lpsz_menu_name: usize, // Char *
	lpsz_class_name: *const [u8;80],
	h_icon_sm: *const c_void,
}

#[link(name = "gdi32")]
extern "system" {
	fn CreateIcon(hi: *const c_void, w: i32, h: i32, planes: u8,
		bitspixel: u8, and: *const u32, xor: *const u32)
		-> *const c_void;
	fn LoadCursorW(hi: *const c_void, cursorName: usize) -> *const c_void;
	fn GetStockObject(fnObject: i32) -> *const c_void;
	fn RegisterClassExW(a: *const WndClassEx) -> u16;
}

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
	
	let window_class = WndClassEx {
		cb_size: mem::size_of::<WndClassEx>() as u32,
		style: 0x0002 | 0x0001,
		lpfn_wnd_proc: wnd_proc,
		cb_cls_extra: 0,
		cb_wnd_extra: 0,
		h_instance: hi,
		h_icon: new_icon,
		h_cursor: unsafe { LoadCursorW(null(), 32512) },
		hbr_background: unsafe { GetStockObject(0) },
		lpsz_menu_name: 0,
		lpsz_class_name: &name,
		h_icon_sm: new_icon,
	};
	
	if unsafe { RegisterClassExW(&window_class) } == 0 {
		panic!("Failed to register windows class.");
	}
	
	name
}
