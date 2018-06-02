// "awi" crate - Licensed under the MIT LICENSE
//  * Copyright (c) 2017-2018  Jeron A. Lau <jeron.lau@plopgrizzly.com>

use std::ptr::null;

use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::shared::minwindef::HINSTANCE;

pub fn connection_create() -> HINSTANCE {
	unsafe {
		GetModuleHandleW(null())
	}
}
