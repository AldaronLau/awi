// "asi_vulkan" - Aldaron's System Interface - Vulkan
//
// Copyright Jeron A. Lau 2018.
// Distributed under the Boost Software License, Version 1.0.
// (See accompanying file LICENSE_1_0.txt or copy at
// https://www.boost.org/LICENSE_1_0.txt)

use super::null;
use super::mem;

use super::types::*;
use super::Style;
use super::memory::{ Buffer, BufferBuilderType };
use super::Gpu;
use super::Image;
use std::{ rc::Rc };

/// A render-able instance.
pub struct Sprite {
	desc_set: Rc<SpriteContext>,
	// TODO: pub's?
	pub uniform_memory: Buffer,
	pub pipeline: VkPipeline,
	pub pipeline_layout: VkPipelineLayout,
	#[allow(unused)] // To keep in scope, preventing segfault.
	pub texture: Option<Image>,
}

struct SpriteContext {
	desc_set: u64,
	desc_pool: u64,
	vulkan: Gpu,
}

impl Sprite {
	/// Create a new sprite.
	pub unsafe fn new<T>(vulkan: &Gpu, pipeline: &Style,
		buffer_data: T, texture: Option<Image>, tex_count: bool,
		gui: bool)
		 -> Self where T: Clone
	{
	//	let connection = vulkan.get();

		let mut desc_pool = mem::uninitialized();
		let mut desc_set = mem::uninitialized();

		// Descriptor Pool
		(vulkan.get().new_descpool)(
			vulkan.get().device,
			// TODO: based on new_pipeline()
			&VkDescriptorPoolCreateInfo {
				s_type: VkStructureType::DescriptorPoolCreateInfo,
				next: null(),
				flags: 0,
				max_sets: 1,
				pool_size_count: if gui {
					1
				} else if tex_count {
					2
				} else {
					1
				},
				pool_sizes: if gui {
					[VkDescriptorPoolSize { descriptor_type: 
						VkDescriptorType::CombinedImageSampler,
						descriptor_count: 1,
					}].as_ptr()
				} else if tex_count {
					[VkDescriptorPoolSize { descriptor_type: 
						VkDescriptorType::UniformBuffer,
						descriptor_count: 1,
					},
					VkDescriptorPoolSize { descriptor_type: 
						VkDescriptorType::CombinedImageSampler,
						descriptor_count: 1,
					}].as_ptr()
				} else {
					[VkDescriptorPoolSize { descriptor_type: 
						VkDescriptorType::UniformBuffer,
						descriptor_count: 1,
					}].as_ptr()
				},
			},
			null(),
			&mut desc_pool
		).unwrap();

		(vulkan.get().new_descsets)(
			vulkan.get().device,
			&VkDescriptorSetAllocateInfo {
				s_type: VkStructureType::DescriptorSetAllocateInfo,
				next: null(),
				descriptor_pool: desc_pool,
				descriptor_set_count: 1,
				set_layouts: &pipeline.style().2/*descsetlayout*/
			},
			&mut desc_set
		).unwrap();

		// Allocate memory for uniform buffer.
		let uniform_memory = if gui {
			::std::mem::uninitialized()
		} else {
			Buffer::new(vulkan, &[buffer_data],
				BufferBuilderType::Uniform)
		};

		let device = vulkan.get().device;

		txuniform(vulkan, device, desc_set, tex_count, texture.as_ref(),
			if gui { None } else { Some(&uniform_memory) });

		Sprite {
			uniform_memory,
			desc_set: Rc::new(SpriteContext {
				desc_set, desc_pool, vulkan: vulkan.clone(),
			}),
			pipeline: pipeline.style().0/*pipeline*/,
			pipeline_layout: pipeline.style().1/*pipeline_layout*/,
			texture,
		}
	}

	pub/* TODO: (crate)*/ fn handles(&self) -> (u64, u64) {
		(self.desc_set.desc_set, self.desc_set.desc_pool)
	}
}

unsafe fn txuniform(vulkan: &Gpu, device: VkDevice,
	desc_set: VkDescriptorSet, hastex: bool, texture: Option<&Image>,
	memory: Option<&Buffer>)
{
	let mut writer = DescriptorSetWriter::new();

	if let Some(memory) = memory {
		writer = writer.uniform(desc_set, memory);
	}

	if hastex {
		writer = writer.sampler(desc_set, vulkan.get().sampler,
			texture.unwrap().view());
	}

	writer.update_descriptor_sets(vulkan, device);
}

struct DescriptorSetWriter {
	sets: [Set; 255],
	nwrites: u8,
}

impl DescriptorSetWriter {
	/// Create a new DescriptorSetWriter.
	#[inline(always)]
	pub fn new() -> Self {
		Self {
			sets: unsafe { mem::uninitialized() },
			nwrites: 0,
		}
	}

	/// Write a uniform buffer to the descriptor set.
	#[inline(always)]
	pub fn uniform(mut self, desc_set: VkDescriptorSet, memory: &Buffer)
		-> Self
	{
		self.sets[self.nwrites as usize] = Set::Uniform(desc_set,
			memory.buffer());

		self.nwrites += 1;

		self
	}

	/// Write an image sampler to the descriptor set.
	#[inline(always)]
	pub fn sampler(mut self, desc_set: VkDescriptorSet,
		tex_sampler: VkSampler, tex_view: VkImageView) -> Self
	{
		self.sets[self.nwrites as usize] = Set::Sampler(desc_set, tex_sampler, tex_view);

		self.nwrites += 1;

		self
	}

	/// Update the descriptor sets.
	#[inline(always)]
	pub fn update_descriptor_sets(&self, connection: &Gpu,
		device: VkDevice) -> ()
	{
		let connection = connection.get();

		let mut buffer_infos: [VkDescriptorBufferInfo; 255] = unsafe {
			mem::uninitialized()
		};
		let mut image_infos: [VkDescriptorImageInfo; 255] = unsafe {
			mem::uninitialized()
		};
		let mut writes: [VkWriteDescriptorSet; 255] = unsafe {
			mem::uninitialized()
		};

		for i in 0..self.nwrites {
			match self.sets[i as usize] {
				Set::Sampler(desc_set, tex_sampler, tex_view) => {
					image_infos[i as usize] = VkDescriptorImageInfo {
						sampler: tex_sampler,
						image_view: tex_view,
						image_layout: VkImageLayout::General,
					};
					writes[i as usize] = VkWriteDescriptorSet {
						s_type: VkStructureType::WriteDescriptorSet,
						next: null(),
						dst_set: desc_set,
						dst_binding: i as u32,
						descriptor_count: 1, //tex_count,
						descriptor_type: VkDescriptorType::CombinedImageSampler,
						image_info: &image_infos[i as usize],
						buffer_info: null(),
						dst_array_element: 0,
						texel_buffer_view: null(),
					};
				}
				Set::Uniform(desc_set, buffer) => {				
					buffer_infos[i as usize] = VkDescriptorBufferInfo {
						buffer: buffer,
						offset: 0,
						range: !0,
					};
					writes[i as usize] = VkWriteDescriptorSet {
						s_type: VkStructureType::WriteDescriptorSet,
						next: null(),
						dst_set: desc_set,
						dst_binding: i as u32,
						descriptor_count: 1,
						descriptor_type: VkDescriptorType::UniformBuffer,
						buffer_info: &buffer_infos[i as usize],
						dst_array_element: 0,
						texel_buffer_view: null(),
						image_info: null(),
					};
				}
			}
		}

		unsafe {
			(connection.update_descsets)(
				device,
				self.nwrites as u32,
				writes.as_ptr(),
				0,
				null(),
			);
		}
	}
}

enum Set {
	Uniform(VkDescriptorSet, VkBuffer),
	Sampler(VkDescriptorSet, VkSampler, VkImageView),
}

impl Drop for SpriteContext {
	fn drop(&mut self) {
		let vk = self.vulkan.get();

		unsafe {
			(vk.drop_descpool)(vk.device, self.desc_pool, null());
		}
	}
}
