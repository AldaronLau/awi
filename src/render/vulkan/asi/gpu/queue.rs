// "asi_vulkan" - Aldaron's System Interface - Vulkan
//
// Copyright Jeron A. Lau 2018.
// Distributed under the Boost Software License, Version 1.0.
// (See accompanying file LICENSE_1_0.txt or copy at
// https://www.boost.org/LICENSE_1_0.txt)

use super::super::gpu;
use super::super::types::*;
use c_void;
use std::mem;

pub(super) unsafe fn new(vkd: (VkDevice, unsafe extern "system" fn(
	VkDevice, *const i8) -> *mut c_void), pqi: u32)
	-> Result<VkQueue, String>
{
	// Load function
	type VkGetDeviceQueue = extern "system" fn(device: VkDevice,
		queueFamilyIndex: u32, queueIndex: u32, pQueue: *mut VkQueue)
		-> ();
	let vk_get_device_queue: VkGetDeviceQueue = gpu::vkd_sym(vkd.0, vkd.1,
		b"vkGetDeviceQueue\0")?;
	// Set Data
	let mut queue = mem::uninitialized();
	// Run Function
	vk_get_device_queue(vkd.0, pqi, 0, &mut queue);
	// Return
	Ok(queue)
}
