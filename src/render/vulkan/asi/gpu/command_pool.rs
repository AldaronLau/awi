// "asi_vulkan" - Aldaron's System Interface - Vulkan
//
// Copyright Jeron A. Lau 2018.
// Distributed under the Boost Software License, Version 1.0.
// (See accompanying file LICENSE_1_0.txt or copy at
// https://www.boost.org/LICENSE_1_0.txt)

use super::super::gpu;
use super::super::types::*;
use c_void;
use std::{ mem, ptr::null_mut };

pub(super) unsafe fn new(vkd: (VkDevice,
	unsafe extern "system" fn( VkDevice, *const i8) -> *mut c_void),
	pqi: u32)
	-> Result<(VkCommandBuffer, u64), String>
{
	#[repr(C)]
	enum VkCommandBufferLevel {
		Primary = 0,
	}

	#[repr(C)]
	struct VkCommandPoolCreateInfo {
		s_type: VkStructureType,
		p_next: *mut c_void,
		flags: u32,
		queue_family_index: u32,
	}

	#[repr(C)]
	struct VkCommandBufferAllocateInfo {
		s_type: VkStructureType,
		p_next: *mut c_void,
		command_pool: u64,
		level: VkCommandBufferLevel,
		command_buffer_count: u32,
	}

	// Load function
	type VkCreateCommandPool = extern "system" fn(device: VkDevice,
		pCreateInfo: *const VkCommandPoolCreateInfo,
		pAllocator: *mut c_void, pCommandPool: *mut u64) -> VkResult;
	let vk_create_command_pool: VkCreateCommandPool = gpu::vkd_sym(
		vkd.0, vkd.1, b"vkCreateCommandPool\0")?;

	// Set Data
	let mut command_pool = 0;
	let mut command_buffer = mem::uninitialized();

	let create_info = VkCommandPoolCreateInfo {
		s_type: VkStructureType::CommandPoolCreateInfo,
		p_next: null_mut(),
		flags: 0x00000002, // Reset Command Buffer
		queue_family_index: pqi,
	};

	// Run Function
	vk_create_command_pool(vkd.0, &create_info, null_mut(),
		&mut command_pool).unwrap();

	// Load Function
	type VkAllocateCommandBuffers = extern "system" fn(device: VkDevice,
		ai: *const VkCommandBufferAllocateInfo,
		cmd_buffs: *mut VkCommandBuffer) -> VkResult;
	let vk_allocate_command_buffers: VkAllocateCommandBuffers =
		gpu::vkd_sym(vkd.0, vkd.1, b"vkAllocateCommandBuffers\0")?;

	// Set Data
	let allocate_info = VkCommandBufferAllocateInfo {
		s_type: VkStructureType::CommandBufferAllocateInfo,
		p_next: null_mut(),
		command_pool: command_pool,
		level: VkCommandBufferLevel::Primary,
		command_buffer_count: 1,
	};

	// Run Function
	vk_allocate_command_buffers(vkd.0, &allocate_info, &mut command_buffer)
		.unwrap();

	// Return
	Ok((command_buffer, command_pool))
}
