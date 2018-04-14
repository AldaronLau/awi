// os_window/windows/connection_create.rs -- Aldaron's Window Interface
// Copyright (c) 2017-2018  Jeron A. Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE

use libc::c_void;
use std::ptr::null;

#[link(name = "user32")]
extern "system" {
	fn GetModuleHandleW(a: *const c_void) -> *mut c_void;
}

pub fn connection_create() -> *mut c_void {
	unsafe {
		GetModuleHandleW(null())
	}
}
