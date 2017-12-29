// Aldaron's Window Interface
// Copyright (c) 2017 Plop Grizzly, Jeron Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE
//
// src/window_connection/mod.rs

use ami::Void;

/// Connection is listed first, then window.
#[derive(Clone)]
pub enum WindowConnection {
	Xcb(*mut Void, u32),
	Wayland,
	DirectFB,
	Windows(*const Void),
	Android,
	IOS,
	AldaronsOS,
	Arduino,
	Switch,
	Web,
	NoOS,
}
