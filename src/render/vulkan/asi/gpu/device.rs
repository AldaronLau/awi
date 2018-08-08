// "asi_vulkan" - Aldaron's System Interface - Vulkan
//
// Copyright Jeron A. Lau 2018.
// Distributed under the Boost Software License, Version 1.0.
// (See accompanying file LICENSE_1_0.txt or copy at
// https://www.boost.org/LICENSE_1_0.txt)

use super::super::gpu;
use super::super::types::*;
use super::VulkanApi;
use c_void;
use std::{ mem, ptr::{ null, null_mut } };

pub(super) unsafe fn get_gpu(vk: VkInstance, lib: &VulkanApi,
	surface: VkSurfaceKHR)
	-> Result<(VkPhysicalDevice, u32, bool, VkFormat), String>
{
	#[repr(C)]
	struct VkQueueFamilyProperties {
		queue_flags: u32,
		queue_count: u32,
		timestamp_valid_bits: u32,
		min_image_transfer_granularity: VkExtent3D,
	}

	// Load Function
	type ListGpus = unsafe extern "system" fn(VkInstance, *mut u32,
		*mut VkPhysicalDevice) -> VkResult;
	let vk_list_gpus: ListGpus = gpu::vk_sym(vk, lib,
		b"vkEnumeratePhysicalDevices\0").unwrap();

	// Set Data
	let mut num_gpus = 0;

	// Run Function
	vk_list_gpus(vk, &mut num_gpus, null_mut()).unwrap();

	// Set Data
	let mut gpus = vec![mem::uninitialized(); num_gpus as usize];

	// Run function
	vk_list_gpus(vk, &mut num_gpus, gpus.as_mut_ptr()).unwrap();

	// Load functions
	type GetGpuQueueFamProps = unsafe extern "system" fn(VkPhysicalDevice,
		*mut u32, *mut VkQueueFamilyProperties) -> ();
	type GetGpuSurfaceSupport = unsafe extern "system" fn(VkPhysicalDevice,
		u32, VkSurfaceKHR, *mut u32) -> VkResult;
	type GetGpuProps = unsafe extern "system" fn(VkPhysicalDevice, VkFormat,
		*mut VkFormatProperties) -> ();
	type GetGpuSurfaceFormats = unsafe extern "system" fn(VkPhysicalDevice,
		VkSurfaceKHR, *mut u32, *mut VkSurfaceFormatKHR) -> VkResult;

	let vk_get_props: GetGpuQueueFamProps = gpu::vk_sym(vk, lib,
		b"vkGetPhysicalDeviceQueueFamilyProperties\0")?;
	let vk_get_support: GetGpuSurfaceSupport = gpu::vk_sym(vk, lib,
		b"vkGetPhysicalDeviceSurfaceSupportKHR\0")?;
	let vk_gpu_props: GetGpuProps = gpu::vk_sym(vk, lib,
		b"vkGetPhysicalDeviceFormatProperties\0")?;
	let vk_gpu_surface_formats: GetGpuSurfaceFormats = gpu::vk_sym(vk, lib,
		b"vkGetPhysicalDeviceSurfaceFormatsKHR\0")?;

	// Process Data
	for i in 0..(num_gpus as usize) {
		let mut num_queue_families = 0;

		vk_get_props(gpus[i], &mut num_queue_families, null_mut());

		let queue_families_size = num_queue_families as usize;

		let mut properties = Vec::with_capacity(queue_families_size);

		properties.set_len(queue_families_size);

		vk_get_props(gpus[i], &mut num_queue_families,
			properties.as_mut_ptr());

		for j in 0..queue_families_size {
			let k = j as u32;
			let mut supports_present = 0;

			vk_get_support(gpus[i], k, surface,
				&mut supports_present).unwrap();

			if supports_present != 0 &&
				(properties[j].queue_flags & 0x00000001) != 0
			{
				// Get format
				let mut nformats = 1;
				let mut format = mem::uninitialized();
				vk_gpu_surface_formats(gpus[i], surface,
					&mut nformats, &mut format).unwrap();
				let format = format.format;

				// 
				let mut props = mem::uninitialized();

				vk_gpu_props(gpus[i], format.clone(), &mut props);

				return Ok((gpus[i], k,
					props.linear_tiling_features
						& 0x00000001 /* sampled image */
						!= 0,
					format
				));
			}
		}
	}

	Err("Couldn't Create Gpu.".to_string())
}

pub(super) unsafe fn create_device(vk: VkInstance, lib: &VulkanApi,
	gpu: VkPhysicalDevice, pqi: u32) -> VkDevice
{
	let mut device = mem::uninitialized();

	#[derive(Debug)] #[repr(C)]
	struct VkDeviceQueueCreateInfo {
		s_type: VkStructureType,
		p_next: *mut c_void,
		flags: u32,
		queue_family_index: u32,
		queue_count: u32,
		p_queue_priorities: *const f32,
	}

	#[derive(Debug)] #[repr(C)]
	struct VkDeviceCreateInfo {
		s_type: VkStructureType,
		p_next: *mut c_void,
		flags: u32,
		queue_create_info_count: u32,
		p_queue_create_infos: *const VkDeviceQueueCreateInfo,
		enabled_layer_count: u32,
		enabled_layer_names: *const *const u8,
		enabled_extension_count: u32,
		enabled_extension_names: *const *const u8,
		enabled_features: *mut c_void,
	}

	// Load function
	type VkCreateDevice = extern "system" fn(
		physicalDevice: VkPhysicalDevice,
		pCreateInfo: *const VkDeviceCreateInfo,
		pAllocator: *mut c_void,
		pDevice: *mut VkDevice) -> VkResult;
	let vk_create_device: VkCreateDevice = gpu::vk_sym(vk, lib,
		b"vkCreateDevice\0").unwrap();

	let ext = b"VK_KHR_swapchain\0";

	vk_create_device(gpu, &VkDeviceCreateInfo {
		s_type: VkStructureType::DeviceCreateInfo,
		p_next: null_mut(),
		flags: 0,
		queue_create_info_count: 1,
		p_queue_create_infos: [VkDeviceQueueCreateInfo {
			s_type: VkStructureType::DeviceQueueCreateInfo,
			p_next: null_mut(),
			flags: 0,
			queue_family_index: pqi,
			queue_count: 1,
			p_queue_priorities: &1.0,
		}].as_ptr(),
		enabled_layer_count: 0,
		enabled_layer_names: null(),
		enabled_extension_count: 1,
		enabled_extension_names: [ext.as_ptr()].as_ptr(),
		enabled_features: null_mut(),
	}, null_mut(), &mut device).unwrap();

	device
}
