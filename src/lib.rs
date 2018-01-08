// Aldaron's Window Interface
// Copyright (c) 2017 Jeron Aldaron Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE
//
// src/lib.rs

//! Aldaron's Window Interface is a library developed by Plop Grizzly for
//! creating a window and handling it's input.

#![warn(missing_docs)]
#![doc(html_logo_url = "http://plopgrizzly.com/awi/icon.png",
	html_favicon_url = "http://plopgrizzly.com/awi/icon.svg",
	html_root_url = "http://plopgrizzly.com/awi/")]

pub(crate) extern crate afi;
pub(crate) extern crate afi_docf;
pub(crate) extern crate aci_png;
pub(crate) extern crate libc;
#[macro_use] pub(crate) extern crate ami;

pub(crate) mod input;
pub(crate) mod os_window;
pub(crate) mod window_connection;
pub(crate) mod window;
pub(crate) mod window_ops;

pub use input::Input;
pub use input::Key;
pub use input::Click;
pub use input::Joystick;
pub use input::Button;
pub use input::Msg;
pub use window_connection::WindowConnection;
pub use window::Window;
pub use window_ops::WindowOps;
pub use afi_docf::{ Emphasis, Align, FontColor };

pub(crate) use input::keyboard::Keyboard;

// Default Width and Height for a window.
pub(crate) const MWW : u32 = 640;
pub(crate) const MWH : u32 = 360;

// Main
/*#[cfg(target_os = "android")]
#[allow(unused)]
#[no_mangle]
pub extern "C" fn gsp_main(activity: *mut ANativeActivity) -> () {
	println!("Got Start");
}*/

/*#[cfg(not(target_os = "android"))]
#[no_mangle]
pub extern "C" fn gsp_main() -> () {
	println!("Got Start");
}*/
