// "asi_vulkan" - Aldaron's System Interface - Vulkan
//
// Copyright Jeron A. Lau 2018.
// Distributed under the Boost Software License, Version 1.0.
// (See accompanying file LICENSE_1_0.txt or copy at
// https://www.boost.org/LICENSE_1_0.txt)

use super::super::c_void;
use super::super::null;
use std::{ mem, ptr };

use super::super::VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT;
use super::super::VK_MEMORY_PROPERTY_HOST_COHERENT_BIT;
use super::super::Gpu;
use super::super::types::*;
use std::{ rc::Rc };

pub enum BufferBuilderType {
	Uniform,
	Vertex,
}

/// A buffer in GPU memory.
#[derive(Clone)] pub struct Buffer(Rc<BufferContext>);

struct BufferContext {
	buffer: u64,
	memory: u64,
	vulkan: Gpu,
}

impl Buffer {
	/// Create a new buffer on the GPU.
	#[inline(always)]
	pub fn new<T: Clone>(vulkan: &Gpu, data: &[T], bbt: BufferBuilderType)
		-> Buffer
	{
		let mut buffer = unsafe { mem::uninitialized() };
		let mut memory = unsafe { mem::uninitialized() };
		let mut mem_reqs = unsafe { mem::uninitialized() };
		unsafe {
			(vulkan.get().new_buffer)(
				vulkan.get().device,
				&VkBufferCreateInfo {
					s_type: VkStructureType::BufferCreateInfo,
					next: ptr::null(),
					flags: 0,
					size: (mem::size_of::<T>() * data.len())
						as u64,
					usage: match bbt {
					  BufferBuilderType::Uniform =>
					    VkBufferUsage::UniformBufferBit,
					  BufferBuilderType::Vertex =>
					    VkBufferUsage::VertexIndexBufferBit,
					},
					sharing_mode: VkSharingMode::Exclusive,
					queue_family_index_count: 0,
					queue_family_indices: ptr::null(),
				},
				ptr::null(),
				&mut buffer
			).unwrap();
		}
		// memory requirements
		unsafe {
			(vulkan.get().get_bufmemreq)(
				vulkan.get().device,
				buffer,
				&mut mem_reqs
			);
		}
		// memory
		unsafe {
			let memory_type_index = super::super::get_memory_type(
				vulkan,
				mem_reqs.memory_type_bits,
				VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT |
				VK_MEMORY_PROPERTY_HOST_COHERENT_BIT
			);

			(vulkan.get().mem_allocate)(
				vulkan.get().device,
				&VkMemoryAllocateInfo {
					s_type: VkStructureType::MemoryAllocateInfo,
					next: ptr::null(),
					allocation_size: mem_reqs.size,
					memory_type_index,
				},
				ptr::null(),
				&mut memory
			).unwrap();
			(vulkan.get().bind_buffer_mem)(
				vulkan.get().device,
				buffer,
				memory,
				0
			).unwrap();
		}

		let buffer = Buffer(Rc::new(BufferContext {
			vulkan: vulkan.clone(), buffer, memory 
		}));

		buffer.update(data, vulkan);

		buffer
	}

	pub fn memory(&self) -> u64 {
		self.0.memory
	}

	pub fn buffer(&self) -> u64 {
		self.0.buffer
	}

	/// Update the contents of the memory.
	#[inline(always)] pub fn update<T: Clone>(&self, data: &[T],
		vulkan: &Gpu)
	{
		let c = vulkan.get();

		let mut mapped: *mut T = unsafe { mem::uninitialized() };

		unsafe {
			(c.mapmem)(c.device, self.memory(), 0, !0, 0,
				&mut mapped as *mut *mut _ as *mut *mut c_void)
				.unwrap();
		}

		if mapped.is_null() {
			panic!("Couldn't Map Buffer Memory?  Unknown cause.");
		}

		unsafe {
			for i in 0..data.len() {
				*mapped.offset(i as isize) = data[i].clone();
			}
			(c.unmap)(c.device, self.memory());
		}
	}
}

impl Drop for BufferContext {
	fn drop(&mut self) {
		let vulkan = self.vulkan.get();

		unsafe {
			(vulkan.drop_buffer)(vulkan.device,self.buffer,null());
			(vulkan.drop_memory)(vulkan.device,self.memory,null());
		}
	}
}
