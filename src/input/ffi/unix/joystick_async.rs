// Aldaron's Window Interface
// Copyright (c) 2017 Plop Grizzly, Jeron Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE
//
// src/input/ffi/unix/joystick_async.rs

extern {
	fn fcntl(fd: i32, cmd: i32, v: i32) -> i32;
}

pub fn joystick_async(fd: i32) -> () {
	let error = unsafe {
		fcntl(fd, 0x4, 0x800)
	} == -1;

	if error {
		panic!("Joystick unplugged 2!");
	}
}
