// Copyright Jeron A. Lau 2017 - 2018.
// Dual-licensed under either the MIT License or the Boost Software License,
// Version 1.0.  (See accompanying file LICENSE_1_0.txt or copy at
// https://www.boost.org/LICENSE_1_0.txt)

//! Interface with the GPU to render graphics or do fast calculations.

mod base;

pub use self::base::*;

#[cfg(any(
	target_os="macos", target_os="android", target_os="linux",
	target_os="windows", target_os="nintendo_switch"
))] mod vulkan;

#[cfg(any(
	target_os="android", target_os="linux", target_os="windows",
	target_os="web"
))] mod opengl;

/// Create a new Vulkan / OpenGL Display.
pub fn new_display() -> Result<Box<Display>, String> {
	let mut err = "".to_string();

	// Try Vulkan first.
	#[cfg(any(
		target_os="macos", target_os="android", target_os="linux",
		target_os="windows", target_os="nintendo_switch"
	))]
	{
		match vulkan::new() {
			Ok(vulkan) => return Ok(vulkan),
			Err(vulkan) => err.push_str(&vulkan),
		}
		err.push('\n');
	}

	// Fallback on OpenGL/OpenGLES
	#[cfg(any(
		target_os="android", target_os="linux", target_os="windows",
	))]
	{
		match opengl::new() {
			Ok(opengl) => return Ok(opengl),
			Err(opengl) => err.push_str(opengl),
		}
		err.push('\n');
	}

	// Give up
	err.push_str("No more backend options");
	Err(err)
}
