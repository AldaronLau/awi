// "asi_vulkan" - Aldaron's System Interface - Vulkan
//
// Copyright Jeron A. Lau 2018.
// Distributed under the Boost Software License, Version 1.0.
// (See accompanying file LICENSE_1_0.txt or copy at
// https://www.boost.org/LICENSE_1_0.txt)

use super::null;
use super::mem;

use super::Gpu;
use super::VK_SAMPLE_COUNT;
use super::ShaderModule;
use super::types::*;
use std::{ rc::Rc };

pub struct Style(Rc<StyleContext>);

struct StyleContext {
	pipeline: u64,
	pipeline_layout: u64,
	descsetlayout: u64,
	vulkan: Gpu,
}

impl Style {
	pub fn new(connection: &mut Gpu, render_pass: VkRenderPass,
		vertex: &ShaderModule, fragment: &ShaderModule, ntextures: u32,
		nvbuffers: u32, alpha: bool) -> Self
	{
		new_pipeline(connection, render_pass, vertex, fragment,
			ntextures, nvbuffers, alpha)
	}

	pub (crate) fn style(&self) -> (u64, u64, u64) {
		(self.0 .pipeline, self.0 .pipeline_layout, self.0 .descsetlayout)
	}
}

pub fn new_pipeline(vulkan: &mut Gpu, render_pass: VkRenderPass,
	vertex: &ShaderModule, fragment: &ShaderModule, ntextures: u32,
	nvbuffers: u32, alpha: bool) -> Style
{ unsafe {
	let connection = vulkan.get();

	let mut pipeline = mem::uninitialized();
	let mut pipeline_layout = mem::uninitialized();
	let mut descsetlayout = mem::uninitialized();

	// depth/stencil config:
	const NO_OP_STENCIL_STATE: VkStencilOpState = VkStencilOpState {
		fail_op: VkStencilOp::Keep,
		pass_op: VkStencilOp::Keep,
		depth_fail_op: VkStencilOp::Keep,
		compare_op: VkCompareOp::Always,
		compare_mask: 0,
		write_mask: 0,
		reference: 0,
	};

	(connection.new_descset_layout)(
		connection.device,
		&VkDescriptorSetLayoutCreateInfo {
			s_type: VkStructureType::DescriptorSetLayoutCreateInfo,
			next: null(),
			flags: 0,
			binding_count: if ntextures == ::std::u32::MAX {
				1
			} else {
				1 + ntextures
			},
			// TODO: consolidate
			bindings: if ntextures == 0 {
				[VkDescriptorSetLayoutBinding {
					binding: 0,
					descriptor_type: VkDescriptorType::UniformBuffer,
					descriptor_count: 1,
					stage_flags: VkShaderStage::VertexAndFragment,
					immutable_samplers: null(),
				}].as_ptr()
			} else if ntextures == ::std::u32::MAX {
				[VkDescriptorSetLayoutBinding {
					binding: 0,
					descriptor_type: VkDescriptorType::CombinedImageSampler,
					descriptor_count: 1, // Texture Count
					stage_flags: VkShaderStage::Fragment,
					immutable_samplers: null(),
				}].as_ptr()
			} else {
				[VkDescriptorSetLayoutBinding {
					binding: 0,
					descriptor_type: VkDescriptorType::UniformBuffer,
					descriptor_count: 1,
					stage_flags: VkShaderStage::VertexAndFragment,
					immutable_samplers: null(),
				},
				VkDescriptorSetLayoutBinding {
					binding: 1,
					descriptor_type: VkDescriptorType::CombinedImageSampler,
					descriptor_count: 1, // Texture Count
					stage_flags: VkShaderStage::Fragment,
					immutable_samplers: null(),
				}].as_ptr()
			},
		},
		null(),
		&mut descsetlayout
	).unwrap();

	// pipeline layout:
	(connection.new_pipeline_layout)(
		connection.device,
		&VkPipelineLayoutCreateInfo {
			s_type: VkStructureType::PipelineLayoutCreateInfo,
			next: null(),
			flags: 0,
			set_layout_count: 1,
			set_layouts: [descsetlayout].as_ptr(),
			push_constant_range_count: 0,
			push_constant_ranges: null(),
		},
		null(),
		&mut pipeline_layout
	).unwrap();

	// setup shader stages:
	(connection.new_pipeline)(
		connection.device,
		mem::zeroed(),
		1,
		&VkGraphicsPipelineCreateInfo {
			s_type: VkStructureType::GraphicsPipelineCreateInfo,
			next: null(),
			flags: 0,
			stage_count: 2,
			stages: [
				VkPipelineShaderStageCreateInfo {
					s_type: VkStructureType::PipelineShaderStageCreateInfo,
					next: null(),
					flags: 0,
					stage: VkShaderStage::Vertex,
					module: vertex.0,
					name: b"main\0".as_ptr(), // shader main function name
					specialization_info: null(),
				},
				VkPipelineShaderStageCreateInfo {
					s_type: VkStructureType::PipelineShaderStageCreateInfo,
					next: null(),
					flags: 0,
					stage: VkShaderStage::Fragment,
					module: fragment.0,
					name: b"main\0".as_ptr(), // shader main function name
					specialization_info: null(),
				},
			].as_ptr(),
			vertex_input_state: &VkPipelineVertexInputStateCreateInfo {
				s_type: VkStructureType::PipelineVertexInputStateCreateInfo,
				next: null(),
				flags: 0,
				vertex_binding_description_count: nvbuffers,
				vertex_binding_descriptions: [
					// Vertices
					VkVertexInputBindingDescription {
						binding: 0,
						stride: (mem::size_of::<f32>() * 4) as u32,
						input_rate: VkVertexInputRate::Vertex,
					},
					// Texture Coordinates
					VkVertexInputBindingDescription {
						binding: 1,
						stride: (mem::size_of::<f32>() * 4) as u32,
						input_rate: VkVertexInputRate::Vertex,
					},
					// Color
					VkVertexInputBindingDescription {
						binding: 2,
						stride: (mem::size_of::<f32>() * 4) as u32,
						input_rate: VkVertexInputRate::Vertex,
					},
				].as_ptr(),
				vertex_attribute_description_count: nvbuffers,
				vertex_attribute_descriptions: [
					VkVertexInputAttributeDescription {
						location: 0,
						binding: 0,
						format: VkFormat::R32g32b32a32Sfloat,
						offset: 0,
					},
					VkVertexInputAttributeDescription {
						location: 1,
						binding: 1,
						format: VkFormat::R32g32b32a32Sfloat,
						offset: 0,
					},
					VkVertexInputAttributeDescription {
						location: 2,
						binding: 2,
						format: VkFormat::R32g32b32a32Sfloat,
						offset: 0,
					},
				].as_ptr(),
			},
			input_assembly_state: &VkPipelineInputAssemblyStateCreateInfo {
				s_type: VkStructureType::PipelineInputAssemblyStateCreateInfo,
				next: null(),
				flags: 0,
				topology: VkPrimitiveTopology::TriangleFan,
				primitive_restart_enable: 0,
			},
			tessellation_state: null(),
			viewport_state: &VkPipelineViewportStateCreateInfo {
				s_type: VkStructureType::PipelineViewportStateCreateInfo,
				next: null(),
				flags: 0,
				viewport_count: 1,
				viewports: null(),
				scissor_count: 1,
				scissors: null(),
			},
			rasterization_state: &VkPipelineRasterizationStateCreateInfo {
				s_type: VkStructureType::PipelineRasterizationStateCreateInfo,
				next: null(),
				flags: 0,
				depth_clamp_enable: 0,
				rasterizer_discard_enable: 0,
				polygon_mode: VkPolygonMode::Fill,
				cull_mode: VkCullMode::Back,
				front_face: VkFrontFace::CounterClockwise,
				depth_bias_enable: 0,
				depth_bias_constant_factor: 0.0,
				depth_bias_clamp: 0.0,
				depth_bias_slope_factor: 0.0,
				line_width: 1.0,
			},
			multisample_state: &VkPipelineMultisampleStateCreateInfo {
				s_type: VkStructureType::PipelineMultisampleStateCreateInfo,
				next: null(),
				flags: 0,
				rasterization_samples: VK_SAMPLE_COUNT,
				sample_shading_enable: 0,
				min_sample_shading: 0.0,
				sample_mask: null(),
				alpha_to_coverage_enable: 0,
				alpha_to_one_enable: 0,
			},
			depth_stencil_state: &VkPipelineDepthStencilStateCreateInfo {
				s_type: VkStructureType::PipelineDepthStencilStateCreateInfo,
				next: null(),
				flags: 0,
				depth_test_enable: 1,
				depth_write_enable: 1,
				depth_compare_op: VkCompareOp::LessOrEqual,
				depth_bounds_test_enable: 0, // 
				stencil_test_enable: 0,
				front: NO_OP_STENCIL_STATE,
				back: NO_OP_STENCIL_STATE,
				min_depth_bounds: 0.0, // unused
				max_depth_bounds: 0.0, // unused
			},
			color_blend_state: &VkPipelineColorBlendStateCreateInfo {
				s_type: VkStructureType::PipelineColorBlendStateCreateInfo,
				next: null(),
				flags: 0,
				logic_op_enable: 0,
				logic_op: VkLogicOp::Clear,
				attachment_count: 1,
				attachments: &VkPipelineColorBlendAttachmentState {
					blend_enable: if alpha { 1 } else { 0 },
					src_color_blend_factor: VkBlendFactor::SrcAlpha,
					dst_color_blend_factor: VkBlendFactor::OneMinusSrcAlpha,
					color_blend_op: VkBlendOp::Add,
					src_alpha_blend_factor: VkBlendFactor::SrcAlpha,
					dst_alpha_blend_factor: VkBlendFactor::One,
					alpha_blend_op: VkBlendOp::Add,
					color_write_mask:
						if alpha { 0b1111 } // RGBA
						else { 0b111 }, // RGB
				},
				blend_constants: [0.0, 0.0, 0.0, 0.0],
			},
			dynamic_state: &VkPipelineDynamicStateCreateInfo {
				s_type: VkStructureType::PipelineDynamicStateCreateInfo,
				next: null(),
				flags: 0,
				dynamic_state_count: 2,
				dynamic_states: [
					VkDynamicState::Viewport, VkDynamicState::Scissor
				].as_ptr(),
			},
			layout: pipeline_layout,
			render_pass: render_pass,
			subpass: 0,
			base_pipeline_handle: mem::zeroed(), // NULL TODO: ?
			base_pipeline_index: 0,
		},
		null(),
		&mut pipeline
	).unwrap();

	Style(Rc::new(StyleContext {
		pipeline, pipeline_layout, descsetlayout, vulkan: vulkan.clone()
	}))
}}

impl Drop for StyleContext {
	fn drop(&mut self) {
		let vk = self.vulkan.get();

		unsafe {
			(vk.drop_pipeline)(vk.device, self.pipeline, null());
			(vk.drop_pipeline_layout)(vk.device,
				self.pipeline_layout, null());
			(vk.drop_descset_layout)(vk.device, self.descsetlayout,
				null());
		}
	}
}
