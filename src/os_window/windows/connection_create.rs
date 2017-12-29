// Aldaron's Window Interface
// Copyright (c) 2017 Plop Grizzly, Jeron Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE
//
// src/os_window/windows/connection_create.rs

use ami::Void;

#[link(name = "user32")]
extern "system" {
	fn GetModuleHandleW(a: *const Void) -> *const Void;
}

pub fn connection_create() -> *const Void {
	unsafe {
		GetModuleHandleW(null!())
	}
}
