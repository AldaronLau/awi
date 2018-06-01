// "awi" crate - Licensed under the MIT LICENSE
//  * Copyright (c) 2017-2018  Jeron A. Lau <jeron.lau@plopgrizzly.com>

use c_void;
use std::ptr::null;

use winapi::um::libloaderapi::GetModuleHandleW;

pub fn connection_create() -> *mut c_void {
	unsafe {
		GetModuleHandleW(null())
	}
}
