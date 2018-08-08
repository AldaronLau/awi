// "asi_vulkan" - Aldaron's System Interface - Vulkan
//
// Copyright Jeron A. Lau 2018.
// Distributed under the Boost Software License, Version 1.0.
// (See accompanying file LICENSE_1_0.txt or copy at
// https://www.boost.org/LICENSE_1_0.txt)

use super::Gpu;

mod buffer;

pub use self::buffer::{ Buffer, BufferBuilderType };

// TODO: is needed?  Probably just use buffer instead.
pub struct Memory<T> where T: Clone {
	pub data: T,
	pub buffer: buffer::Buffer,
}

impl<T> Memory<T> where T: Clone {
	/// Allocate memory in a GPU buffer.
	#[inline(always)]
	pub fn new(vulkan: &Gpu, data: T) -> Memory<T> {
//		let c = vulkan.0.data();

		let buffer = buffer::Buffer::new(vulkan,
			&[data.clone()],
			buffer::BufferBuilderType::Uniform);

		Memory { data: data.clone(), buffer }
	}

	/// Update the contents of the memory.
	#[inline(always)]
	pub fn update(&self, vulkan: &Gpu) {
		self.buffer.update(&[self.data.clone()], vulkan);
	}
}
