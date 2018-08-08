// "asi_vulkan" - Aldaron's System Interface - Vulkan
//
// Copyright Jeron A. Lau 2018.
// Distributed under the Boost Software License, Version 1.0.
// (See accompanying file LICENSE_1_0.txt or copy at
// https://www.boost.org/LICENSE_1_0.txt)

use super::super::gpu;
use super::super::types::*;
use c_void;
use std::{ mem, ptr::null };

pub(super) unsafe fn new(vkd: (VkDevice, unsafe extern "system" fn(
	VkDevice, *const i8) -> *mut c_void))
	-> Result<VkSampler, String>
{
	// Load function
	type VkCreateSampler = extern "system" fn(VkDevice,
		*const VkSamplerCreateInfo, *const c_void, *mut VkSampler)
		-> VkResult;
	let new_sampler: VkCreateSampler = gpu::vkd_sym(vkd.0, vkd.1,
		b"vkCreateSampler\0")?;
	// Creat the sampler & Return
	let mut sampler = mem::uninitialized();
	new_sampler(
		vkd.0,
		&VkSamplerCreateInfo {
			s_type: VkStructureType::SamplerCreateInfo,
			next: null(),
			flags: 0,
			mag_filter: VkFilter::Linear,
			min_filter: VkFilter::Linear,
			mipmap_mode: VkSamplerMipmapMode::Linear,
			address_mode_u: VkSamplerAddressMode::Repeat,
			address_mode_v: VkSamplerAddressMode::Repeat,
			address_mode_w: VkSamplerAddressMode::Repeat,
			mip_lod_bias: 0.0,
			anisotropy_enable: 0,
			max_anisotropy: 1.0,
			compare_enable: 0,
			compare_op: VkCompareOp::Never,
			min_lod: 0.0,
			max_lod: 0.0,
			border_color: VkBorderColor::FloatOpaqueWhite,
			unnormalized_coordinates: 0,
		},
		null(),
		&mut sampler
	).unwrap();
	Ok(sampler)
}
