// Aldaron's Window Interface
// Copyright (c) 2017 Plop Grizzly, Jeron Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE
//
// src/os_window/windows/types.rs

use ami::Void;

#[cfg(target_pointer_width = "32")]
type LongPtr = isize;
#[cfg(target_pointer_width = "64")]
type LongPtr = i64;

#[cfg(target_pointer_width = "32")]
type UintPtr = u32;
#[cfg(target_pointer_width = "64")]
type UintPtr = u64;

type Pvoid = *const Void;
type Handle = Pvoid;

pub type Lparam = LongPtr;
pub type Wparam = UintPtr;

#[repr(C)] #[derive(Copy, Clone, PartialEq)] pub struct Hwnd(Handle);

impl Hwnd {
	pub fn null() -> Hwnd { Hwnd(null!()) }
	pub fn bottom() -> Hwnd { unsafe { Hwnd(null!().offset(1)) } }
	pub fn notopmost() -> Hwnd { unsafe { Hwnd(null!().offset(-2)) } }
	pub fn top() -> Hwnd { Hwnd(null!()) }
	pub fn topmost() -> Hwnd { unsafe { Hwnd(null!().offset(-1)) } }
}