// Aldaron's Window Interface
// Copyright (c) 2017 Jeron Aldaron Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE
//
// src/window_connection/mod.rs

use libc::c_void;

/// Connection is listed first, then window.
#[derive(Clone)]
pub enum WindowConnection {
	/// XCB Window Handles
	Xcb(*mut c_void, u32),
	/// Wayland Window Handles
	Wayland,
	/// DirectFB Window Handles
	DirectFB,
	/// Windows Window Handles
	Windows(*mut c_void, *mut c_void),
	/// Android Window Handles
	Android,
	/// IOS Window Handles
	IOS,
	/// Aldaron's OS Window Handles
	AldaronsOS,
	/// Arduino Window Handles
	Arduino,
	/// Switch Window Handles
	Switch,
	/// Web Window Handles
	Web,
	/// No OS Window Handles
	NoOS,
}
