// Aldaron's Window Interface
// Copyright (c) 2017 Plop Grizzly, Jeron Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE
//
// src/os_window/windows/connection_create.rs

use ami::void_pointer::*;

extern "system" {
	fn GetModuleHandleW(a: VoidPointer) -> VoidPointer;
}

pub fn connection_create() -> VoidPointer {
	unsafe {
		GetModuleHandleW(NULL)
	}
}
