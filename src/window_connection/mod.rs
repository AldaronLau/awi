// lib/window_connection/mod.rs
// Graphical Software Packager
// Copyright 2017 (c) Aldaron's Tech
// Copyright 2017 (c) Jeron Lau
// Licensed under the MIT LICENSE

use ami::Void;

/// Connection is listed first, then window.
#[derive(Clone)]
pub enum WindowConnection {
	Xcb(*mut Void, u32),
	Wayland,
	DirectFB,
	Windows,
	Android,
	IOS,
	AldaronsOS,
	Arduino,
	Switch,
	Web,
	NoOS,
}
