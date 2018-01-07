// Aldaron's Window Interface
// Copyright (c) 2017 Plop Grizzly, Jeron Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE
//
// src/os_window/windows/class_create.rs

use ami::Void;
use super::{ string };
use super::types::*;

use std::mem;

#[repr(C)]
struct WndClassEx {
	cb_size: u32,
	style: u32,
	lpfn_wnd_proc: extern "C" fn(a: Hwnd, b: u32, c: *const Void,
		d: *const Void) -> Lresult,
	cb_cls_extra: i32,
	cb_wnd_extra: i32,
	h_instance: *const Void,
	h_icon: *const Void,
	h_cursor: *const Void,
	hbr_background: *const Void,
	lpsz_menu_name: usize, // Char *
	lpsz_class_name: *const [u8;80],
	h_icon_sm: *const Void,
}

#[link(name = "gdi32")]
extern "system" {
	fn CreateIcon(hi: *const Void, w: i32, h: i32, planes: u8,
		bitspixel: u8, and: *const u32, xor: *const u32) -> *const Void;
	fn LoadCursorW(hi: *const Void, cursorName: usize) -> *const Void;
	fn GetStockObject(fnObject: i32) -> *const Void;
	fn RegisterClassExW(a: *const WndClassEx) -> u16;
}

pub fn class_create(hi: *const Void, title: &str, icon: (u32, u32, &[u32]),
	wnd_proc: extern "C" fn(a: Hwnd, b: u32, c: *const Void,
		d: *const Void) -> Lresult)
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
		h_cursor: unsafe { LoadCursorW(null!(), 32512) },
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
