// Aldaron's Window Interface
// Copyright (c) 2017 Plop Grizzly, Jeron Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE
//
// src/os_window/windows/types.rs

use ami::Void;

#[cfg(target_pointer_width = "64")]
type LongPtr = i64;
#[cfg(target_pointer_width = "32")]
type LongPtr = i32;

#[cfg(target_pointer_width = "32")]
type UintPtr = u32;
#[cfg(target_pointer_width = "64")]
type UintPtr = u64;

// Long is always 32 bits on Windows.
pub type Long = i32;
pub type Bool = i32;

type Pvoid = *mut Void;
type Handle = Pvoid;

pub type Lparam = LongPtr;
pub type Wparam = UintPtr;

pub type Lresult = LongPtr;

#[repr(C)] #[derive(Copy, Clone, PartialEq)] pub struct Hwnd(Handle);

#[repr(C)] #[derive(Copy, Clone)] pub struct Rect {
	pub left: Long,
	pub top: Long,
	pub right: Long,
	pub bottom: Long,
}

#[allow(dead_code)]
impl Hwnd {
	pub fn null() -> Hwnd { Hwnd(null_mut!()) }
	pub fn bottom() -> Hwnd { unsafe { Hwnd(null_mut!().offset(1)) } }
	pub fn notopmost() -> Hwnd { unsafe { Hwnd(null_mut!().offset(-2)) } }
	pub fn top() -> Hwnd { Hwnd(null_mut!()) }
	pub fn topmost() -> Hwnd { unsafe { Hwnd(null_mut!().offset(-1)) } }
	pub fn to_ptr(&self) -> *mut Void { self.0 }
}