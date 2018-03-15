// os_window/windows/connection_create.rs -- Aldaron's Window Interface
// Copyright (c) 2017-2018  Jeron A. Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE

use ami::Void;

#[link(name = "user32")]
extern "system" {
	fn GetModuleHandleW(a: *const Void) -> *mut Void;
}

pub fn connection_create() -> *mut Void {
	unsafe {
		GetModuleHandleW(null!())
	}
}
