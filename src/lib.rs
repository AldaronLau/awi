// Copyright Jeron A. Lau 2017-2018.
// Dual-licensed under either the MIT License or the Boost Software License,
// Version 1.0.  (See accompanying file LICENSE_1_0.txt or copy at
// https://www.boost.org/LICENSE_1_0.txt)

//! Aldaron's Window Interface is a library developed by Plop Grizzly for
//! creating a window and handling it's input.

#![warn(missing_docs)]
#![doc(html_logo_url = "https://plopgrizzly.com/images/awi.png",
       html_favicon_url = "https://plopgrizzly.com/images/awi.png")]

#[macro_use] extern crate ami;
extern crate barg;
#[macro_use] extern crate approx;
extern crate arrayvec;
extern crate ordered_float;
extern crate stick;
pub extern crate afi;
#[cfg(target_os="windows")] extern crate winapi;
#[cfg(not(target_arch="wasm32"))] #[macro_use] extern crate dl_api;
#[cfg(target_arch="wasm32")] #[macro_use] extern crate stdweb;
#[cfg(target_arch="wasm32")] #[macro_use] extern crate stdweb_derive;

pub mod screen;

mod window_connection;
pub(crate) mod input;
#[cfg(not(target_arch="wasm32"))] pub(crate) mod window;
#[cfg(not(target_arch="wasm32"))] pub(crate) mod window_ops;
pub mod render;

/* 1. Windows */ #[cfg(target_os = "windows")] pub(crate) mod os { mod windows; pub use self::windows::*; }
/* 2. Linux / BSD */ #[cfg(any(target_os = "linux", target_os = "freebsd", target_os = "dragonfly", target_os = "bitrig", target_os = "openbsd", target_os = "netbsd"))] pub(crate) mod os { mod linux; pub use self::linux::*; }
/* 3. Raspberry Pi (feature) */ #[cfg(target_os = "rpi")] pub(crate) mod os { mod rpi; pub use self::rpi::*; }
/* 4. Deskron (feature) */ #[cfg(target_os = "deskron")] pub(crate) mod os { mod deskron; pub use self::deskron::*; }
/* 5. Android */ #[cfg(target_os = "android")] pub(crate) mod os { mod android; pub use self::android::*; }
/* 6. MacOS / iOS */ #[cfg(any(target_os = "macos", target_os = "ios"))] pub(crate) mod os { mod apple; pub use self::apple::*; }
/* 7. Web */ #[cfg(target_arch = "wasm32")] pub(crate) mod os { mod wasm32; pub use self::wasm32::*; }
/* 8. Nintendo Switch (Custom target_os) */ #[cfg(target_os = "switch")] pub(crate) mod os { mod switch; pub use self::switch::*; }
/* 9. Redox */ #[cfg(target_os = "redox")] pub(crate) mod os { mod redox; pub use self::redox::*; }
/* 10. XBox One (Custom target_os) */ #[cfg(target_os = "xbox")] pub(crate) mod os { mod xbox; pub use self::xbox::*; }

/// Compatibility with different platforms, languages, C API
// pub mod c_api;

pub(crate) use std::os::raw::c_void;
pub(crate) use input::keyboard::Keyboard;

pub use input::Event;
#[cfg(not(target_arch="wasm32"))] pub(crate) use window_connection::WindowConnection;
#[cfg(not(target_arch="wasm32"))] pub(crate) use window::Window;

pub use ami::*;
