// "asi_vulkan" - Aldaron's System Interface - Vulkan
//
// Copyright Jeron A. Lau 2018.
// Distributed under the Boost Software License, Version 1.0.
// (See accompanying file LICENSE_1_0.txt or copy at
// https://www.boost.org/LICENSE_1_0.txt)

use super::null;
use std::mem;

use super::Gpu;
use super::types::*;
use std::{ rc::Rc };

/// A `VkFence` (TODO)
#[derive(Clone)] pub struct Fence(Rc<FenceContext>);

struct FenceContext {
	fence: u64,
	vulkan: Gpu,
}

impl Fence {
	pub fn new(connection: &Gpu) -> Self {
		Fence(Rc::new(FenceContext {
			fence: unsafe { new(connection) },
			vulkan: connection.clone()
		}))
	}

	pub fn fence(&self) -> u64 {
		self.0.fence
	}
}

pub unsafe fn new(connection: &Gpu) -> u64 {
	let connection = connection.get();

	let mut fence = mem::uninitialized();

	(connection.create_fence)(
		connection.device,
		&VkFenceCreateInfo {
			s_type: VkStructureType::FenceCreateInfo,
			p_next: null(),
			flags: 0,
		},
		null(),
		&mut fence
	).unwrap();

	fence
}

pub unsafe fn drop(connection: &Gpu, fence: u64) {
	let vk = connection.get();

	(vk.destroy_fence)(vk.device, fence, null())
}

pub unsafe fn wait(connection: &Gpu, fence: u64) {
	let vk = connection.get();

	(vk.wait_fence)(vk.device, 1, [fence].as_ptr(), 1, ::std::u64::MAX)
		.unwrap();
}

impl Drop for FenceContext {
	fn drop(&mut self) {
		unsafe { drop(&self.vulkan, self.fence) }
	}
}
