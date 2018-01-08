// Aldaron's Window Interface
// Copyright (c) 2017 Jeron Aldaron Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE
//
// src/input/ffi/mod.rs

#[cfg(any(target_os = "macos", target_os = "linux", target_os = "android"))]
mod unix;
#[cfg(any(target_os = "macos", target_os = "linux", target_os = "android"))]
pub use self::unix::Joystick;

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub use self::windows::Joystick;

#[cfg(not(any(target_os = "macos",target_os = "linux",target_os = "windows",target_os = "android")))]
mod emulated;
#[cfg(not(any(target_os = "macos",target_os = "linux",target_os = "windows",target_os = "android")))]
pub use self::emulated::Joystick;
