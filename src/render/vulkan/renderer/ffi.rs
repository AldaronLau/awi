// Copyright Jeron A. Lau 2018.
// Dual-licensed under either the MIT License or the Boost Software License,
// Version 1.0.  (See accompanying file LICENSE_1_0.txt or copy at
// https://www.boost.org/LICENSE_1_0.txt)

// TODO: absorb into ffi, only once internal todo is resolved.

use super::super::asi;
use super::super::asi::types::*;
use super::super::asi::Gpu;

pub fn copy_memory<T>(connection: &Gpu, vk_memory: VkDeviceMemory,
	data: &T) where T: Clone
{
	let mapped : *mut T = unsafe {
		asi::map_memory(connection, vk_memory, !0)
	};

	if mapped.is_null() {
		panic!("Couldn't Map Buffer Memory?  Unknown cause.");
	}

	unsafe {
		*mapped = data.clone();
		asi::unmap_memory(connection, vk_memory);
	}
}

pub fn copy_memory_pitched(connection: &Gpu, vk_memory: VkDeviceMemory,
	writer: &Fn(u16, u16) -> [u8; 4], width: usize, height: usize,
	pitch: usize)
{
	let mapped : *mut u8 = unsafe {
		asi::map_memory(connection, vk_memory, !0)
	};

	if mapped.is_null() {
		panic!("Couldn't Map Buffer Memory?  Unknown cause.");
	}

	for i in 0..height {
		for j in 0..width {
			let pixel = writer(j as u16, i as u16);

			for k in 0..4 {
				unsafe {
					*(mapped.offset((i * pitch + j * 4 + k)
							as isize))
						= pixel[k];
				}
			}
		}
	}

	unsafe {
		asi::unmap_memory(connection, vk_memory);
	}
}
