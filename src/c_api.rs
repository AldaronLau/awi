// Copyright Jeron A. Lau 2017-2018.
// Dual-licensed under either the MIT License or the Boost Software License,
// Version 1.0.  (See accompanying file LICENSE_1_0.txt or copy at
// https://www.boost.org/LICENSE_1_0.txt)

use c_void;
use std::{ ptr::null_mut, mem::transmute };

/// An input event.
#[repr(C)]
pub union AwiInput {
	pub none: (),
}

pub(crate) struct AwiCApi {
	awi_input: unsafe extern "C" fn(*mut c_void, *mut AwiInput) -> (),
	awi_update: unsafe extern "C" fn(*mut c_void) -> (),
	awi_free: unsafe extern "C" fn(*mut c_void) -> (),
}

pub(crate) static mut AWI_INIT: unsafe extern "C" fn(*mut *mut c_void, u32)
	-> AwiCApi = ::os::awi_init;
pub(crate) static mut AWI_INPUT: *mut c_void = null_mut();
pub(crate) static mut AWI_UPDATE: *mut c_void = null_mut();
pub(crate) static mut AWI_FREE: *mut c_void = null_mut();

/// Create a new window.
/// ```c
/// #include "awi.h"
///
/// void* window;
/// awi_init(&window, AWI_GRAPHICS | AWI_AUDIO);
///
/// uint16_t width, height;
/// awi_wh(window, &width, &height);
///
/// AwiInput input;
/// awi_input(window, &input);
///
/// awi_update(window);
///
/// awi_free(window);
/// ```
#[no_mangle]
pub unsafe extern "C" fn awi_init(window: *mut *mut c_void, flags: u32) -> () {
	// Create the window
	let awi_c_api = AWI_INIT(window, flags);

	// Load the functions
	AWI_INPUT = transmute(awi_c_api.awi_input);
	AWI_UPDATE = transmute(awi_c_api.awi_update);
	AWI_FREE = transmute(awi_c_api.awi_free);
}

/// Get Input
#[no_mangle]
pub unsafe extern "C" fn awi_input(window: *mut c_void,
	input: *mut AwiInput) -> ()
{
	let awi_input: unsafe extern "C" fn(*mut c_void, *mut AwiInput)
		-> () = transmute(AWI_INPUT);

	awi_input(window, input)
}

/// Update window.
#[no_mangle]
pub unsafe extern "C" fn awi_update(window: *mut c_void) -> () {
	let awi_update: unsafe extern "C" fn(*mut c_void) -> ()
		= transmute(AWI_UPDATE);

	awi_update(window)
}

/// Close the window.
#[no_mangle]
pub unsafe extern "C" fn awi_free(window: *mut c_void) -> () {
	let awi_free: unsafe extern "C" fn(*mut c_void) -> ()
		= transmute(AWI_FREE);

	awi_free(window)
}
