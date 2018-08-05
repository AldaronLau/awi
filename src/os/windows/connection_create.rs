// Copyright Jeron A. Lau 2017-2018.
// Dual-licensed under either the MIT License or the Boost Software License,
// Version 1.0.  (See accompanying file LICENSE_1_0.txt or copy at
// https://www.boost.org/LICENSE_1_0.txt)

use std::ptr::null;

use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::shared::minwindef::HINSTANCE;

pub fn connection_create() -> HINSTANCE {
	unsafe {
		GetModuleHandleW(null())
	}
}
