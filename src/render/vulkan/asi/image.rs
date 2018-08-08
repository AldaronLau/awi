// "asi_vulkan" - Aldaron's System Interface - Vulkan
//
// Copyright Jeron A. Lau 2018.
// Distributed under the Boost Software License, Version 1.0.
// (See accompanying file LICENSE_1_0.txt or copy at
// https://www.boost.org/LICENSE_1_0.txt)

use std::mem;
use std::ptr::{ null };

use super::Gpu;
use super::types::*;
use super::get_memory_type;
use std::{ rc::Rc };

/// An Image
#[derive(Clone)] pub struct Image(Rc<ImageContext>);

struct ImageContext {
	image: u64,
	memory: u64,
	view: u64,
	vulkan: Gpu,
}

impl Image {
	/// Create a new image.
	#[inline(always)] pub fn new(vulkan: &Gpu, width: u32, height: u32,
		format: VkFormat, tiling: VkImageTiling, usage: VkImageUsage,
		initial_layout: VkImageLayout, reqs_mask: VkFlags,
		samples: VkSampleCount) -> Image
	{ unsafe {
		let mut image = mem::uninitialized();
		let mut memory = mem::uninitialized();
		let mut memory_reqs = mem::uninitialized();

		(vulkan.get().create_image)(
			vulkan.get().device,
			&VkImageCreateInfo {
				s_type: VkStructureType::ImageCreateInfo,
				p_next: null(),
				flags: 0,
				image_type: VkImageType::Dim2d,
				format: format.clone(),
				extent: VkExtent3D {
					width,
					height,
					depth: 1,
				},
				mip_levels: 1,
				array_layers: 1,
				samples,
				tiling,
				usage,
				sharing_mode: VkSharingMode::Exclusive,
				queue_family_index_count: 0,
				p_queue_family_indices: null(),
				initial_layout,
			},
			null(),
			&mut image
		).unwrap();

		(vulkan.get().get_imgmemreq)(vulkan.get().device, image,
			&mut memory_reqs);

		let memory_type_index = get_memory_type(
			vulkan,
			memory_reqs.memory_type_bits,
			reqs_mask
		);

		(vulkan.get().mem_allocate)(
			vulkan.get().device,
			&VkMemoryAllocateInfo {
				s_type: VkStructureType::MemoryAllocateInfo,
				next: null(),
				allocation_size: memory_reqs.size,
				memory_type_index,
			},
			null(),
			&mut memory
		).unwrap();

		(vulkan.get().bind_imgmem)(vulkan.get().device, image,
			memory, 0).unwrap();

		let view = super::create_img_view(vulkan, image,
			format.clone(),
			usage != VkImageUsage::DepthStencilAttachmentBit
		);

		Image(Rc::new(ImageContext {
			vulkan: vulkan.clone(), image, memory, view
		}))
	} }

	pub (crate) fn image(&self) -> (u64, u64, u64) {
		(self.0.image, self.0.memory, self.0.view)
	}

	/// Get the memory handle for this image.
	pub fn memory(&self) -> u64 {
		self.image().1
	}

	/// Get the image view for this image
	pub fn view(&self) -> u64 {
		self.image().2
	}
}

impl Drop for ImageContext {
	fn drop(&mut self) {
		let vk = self.vulkan.get();

		unsafe {
			(vk.drop_image)(vk.device, self.image, null());
			(vk.drop_memory)(vk.device, self.memory, null());
			(vk.drop_imgview)(vk.device, self.view, null());
		}
	}
}
