// "awi" - Aldaron's Window Interface
//
// Copyright Jeron A. Lau 2017-2018.
// Distributed under the Boost Software License, Version 1.0.
// (See accompanying file LICENSE_1_0.txt or copy at
// https://www.boost.org/LICENSE_1_0.txt)

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub use self::windows::{ WindowsWindow as OSWindow, key };

#[cfg(target_os = "android")]
mod android;
#[cfg(target_os = "android")]
pub use self::android::{ AndroidWindow as OSWindow, key, gsp_main };

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
pub use self::macos::{ MacosWindow as OSWindow, key };

#[cfg(target_os = "ios")]
mod ios;
#[cfg(target_os = "ios")]
pub use self::macos::{ IosWindow as OSWindow, key };

#[cfg(any(target_os = "linux", target_os = "macos", target_os = "freebsd",
	target_os = "dragonfly", target_os = "bitrig", target_os = "openbsd",
	target_os = "netbsd"))]
mod unix;
#[cfg(any(target_os = "linux", target_os = "macos", target_os = "freebsd",
	target_os = "dragonfly", target_os = "bitrig", target_os = "openbsd",
	target_os = "netbsd"))]
pub use self::unix::{ UnixWindow as OSWindow, key };

// Platforms that don't have standard libary support.

#[cfg(target_os = "aldarons_os")]
mod aldarons_os;
#[cfg(target_os = "aldarons_os")]
pub use self::aldarons_os::{ AldaronsWindow as OSWindow, key };

#[cfg(target_os = "nintendo_switch")]
mod nintendo_switch;
#[cfg(target_os = "nintendo_switch")]
pub use self::nintendo_switch::{ SwitchWindow as OSWindow, key };

#[cfg(target_os = "web_assembly")]
mod web_assembly;
#[cfg(target_os = "web_assembly")]
pub use self::web_assembly::{ WebWindow as OSWindow, key };
