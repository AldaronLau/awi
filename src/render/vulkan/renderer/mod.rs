// Copyright Jeron A. Lau 2018.
// Dual-licensed under either the MIT License or the Boost Software License,
// Version 1.0.  (See accompanying file LICENSE_1_0.txt or copy at
// https://www.boost.org/LICENSE_1_0.txt)

use std::cell::UnsafeCell;

use std::{ mem };
use std::time::Instant;

use super::base::*;

mod ffi;

use super::asi::types::*;
use super::asi::Image;
use super::asi::Style;
use super::asi::Buffer;

// TODO
use super::asi::TransformUniform;
use super::asi::Sprite;
use super::asi::Gpu;

use super::ShapeHandle;

use Matrix;

#[derive(Clone)] #[repr(C)] struct TransformFullUniform {
	mat4: [f32; 16],
}

#[derive(Clone)] #[repr(C)] struct TransformAndFadeUniform {
	mat4: [f32; 16],
	fade: f32,
}

#[derive(Clone)] #[repr(C)] struct TransformAndColorUniform {
	mat4: [f32; 16],
	vec4: [f32; 4],
}

pub struct Vw {
	connection: Gpu,
	present_images: [VkImage; 2], // 2 for double-buffering
	frame_buffers: [VkFramebuffer; 2], // 2 for double-buffering
	image_count: u32, // 1 (single-buffering) or 2 (double-buffering)
	present_image_views: [VkImageView; 2], // 2 for double-buffering
	ms_image: Image,
	depth_image: Image,
	render_pass: VkRenderPass,
}

/// A texture on the GPU.
pub struct Texture {
	mappable_image: Image,
	image: Option<Image>,
//	view: VkImageView,
	pub(super) w: u16,
	pub(super) h: u16,
	pitch: u32,
	staged: bool,
}

pub struct Shape {
	num_buffers: usize,
	buffers: [VkBuffer; 3],
	instance: Sprite,
	fans: Vec<(u32, u32)>,
	transform: Matrix, // Transformation matrix.
}

impl super::base::Point for Shape {
	fn point(&self) -> Vector {
		// Position vector at origin * object transform.
		self.transform * (vector!(), 1f32)
	}
}

pub struct Model {
	shape: super::asi::Buffer,
	vertex_count: u32,
	fans: Vec<(u32, u32)>,
}

pub struct TexCoords {
	vertex_buffer: Buffer,
	vertex_count: u32,
}

pub struct Gradient {
	vertex_buffer: Buffer,
	vertex_count: u32,
}

fn swapchain_resize(connection: &Gpu, image_count: &mut u32,
	present_images: &mut [VkImage; 2],
	present_image_views: &mut [VkImageView; 2],
	frame_buffers: &mut [VkFramebuffer; 2])
	-> (Image, Image, VkRenderPass)
{
	unsafe {
		let submit_fence;
		let depth_image;
		let ms_image;
		let render_pass;

		// Link swapchain to vulkan instance.
		super::asi::create_swapchain(
			connection,
			image_count,
			&mut present_images[0]
		);

		// Link Image Views for each framebuffer
		submit_fence = super::asi::create_image_view(
			connection,
			*image_count,
			present_images,
			present_image_views,
		);

		// Link Depth Buffer to swapchain
		depth_image = super::asi::create_depth_buffer(
			connection,
			&submit_fence,
		);

		// Create multisampling buffer
		ms_image = super::asi::create_ms_buffer(
			connection,
		);

		// Link Render Pass to swapchain
		render_pass = super::asi::create_render_pass(
			connection,
		);

		// Link Framebuffers to swapchain
		super::asi::create_framebuffers(
			connection,
			*image_count,
			render_pass,
			present_image_views,
			&ms_image,
			&depth_image,
			frame_buffers,
		);

		(depth_image, ms_image, render_pass)
	}
}

fn swapchain_delete(vw: &mut Vw) {
	unsafe {
		super::asi::destroy_swapchain(
			&vw.connection,
			&vw.frame_buffers,
			&vw.present_image_views,
			vw.render_pass,
			vw.image_count,
		);
	}
}

fn new_texture(vw: &Vw, width: u16, height: u16) -> Texture {
//	let mut format_props = unsafe { mem::uninitialized() };
	let staged = !vw.connection.sampled();

	let mappable_image = super::asi::Image::new(
		&vw.connection, width as u32, height as u32,
		VkFormat::R8g8b8a8Srgb, // Because VkColorSpace is always Srgb
		VkImageTiling::Linear,
		if staged { VkImageUsage::TransferSrcBit }
		else { VkImageUsage::SampledBit },
		VkImageLayout::Preinitialized,
		0x00000006 /* visible|coherent */,
		VkSampleCount::Sc1
	);

	let layout = unsafe {
		super::asi::subres_layout(&vw.connection, &mappable_image)
	};

	let pitch = layout.row_pitch;

	let image = if staged {
		Some(super::asi::Image::new(
			&vw.connection, width as u32, height as u32,
			VkFormat::R8g8b8a8Unorm,
			VkImageTiling::Optimal,
			VkImageUsage::TransferDstAndUsage,
			VkImageLayout::Undefined, 0,
			VkSampleCount::Sc1))
	} else {
		None
	};

	Texture {
		staged, mappable_image,	image, pitch: pitch as u32,
		w: width, h: height,
	}
}

fn set_texture(vw: &Vw, texture: &mut Texture, writer: &Fn(u16, u16) -> [u8; 4])
{
	ffi::copy_memory_pitched(&vw.connection,
		texture.image
			.as_ref()
			.unwrap_or(&texture.mappable_image)
			.memory(),
		writer, texture.w as usize, texture.h as usize,
		texture.pitch as usize);

	if texture.staged {
		// Use optimal tiled image - create from linear tiled image

		// Copy data from linear image to optimal image.
		unsafe {
			super::asi::copy_image(&vw.connection,
				&texture.mappable_image,
				texture.image.as_ref().unwrap(),
				texture.w, texture.h
			);
		}
	} else {
		// Use a linear tiled image for the texture, is supported
		texture.image = None;
	}
}

impl Vw {
	pub(crate) fn new(rgb: Vector) -> Result<(Vw, ::Window), String> {
		let (mut connection, window) = super::asi::Gpu::new(rgb)?;

		// END BLOCK 2
		let mut image_count = unsafe {
			super::asi::get_buffering(&mut connection)
		};

		// Prepare Swapchain
		let mut present_images: [VkImage; 2] = [unsafe { mem::zeroed() }; 2];
		let mut present_image_views = [unsafe { mem::zeroed() }; 2];
		let mut frame_buffers: [VkFramebuffer; 2]
			= [unsafe { mem::uninitialized() }; 2];

		let (depth_image, ms_image, render_pass)
			= swapchain_resize(&connection,
				&mut image_count,
				&mut present_images,
				&mut present_image_views, &mut frame_buffers);

		let vw = Vw {
			connection,
			present_images, frame_buffers,
			image_count,
			present_image_views, ms_image, depth_image, render_pass,
		};

		Ok((vw, window))
	}
}

fn draw_shape(connection: &Gpu, shape: &Shape) {
	unsafe {
		// TODO: reduce calls to these functions (for speed).
		super::asi::cmd_bind_vb(connection,
			&shape.buffers[..shape.num_buffers]);
		super::asi::cmd_bind_pipeline(connection,
			shape.instance.pipeline);
		super::asi::cmd_bind_descsets(connection,
			shape.instance.pipeline_layout,
			shape.instance.handles().0/*desc_set*/);

		for i in shape.fans.iter() {
			super::asi::cmd_draw(connection, i.1,
				1, i.0, 0);
		}
	}
}

pub struct Renderer {
	earlier: Instant,
	vw: Vw,
	ar: f32,
	opaque_ind: Vec<u32>,
	alpha_ind: Vec<u32>,
	opaque_vec: UnsafeCell<Vec<Shape>>,
	alpha_vec: UnsafeCell<Vec<Shape>>,
	gui: Shape,
	models: Vec<Model>,
	texcoords: Vec<TexCoords>,
	gradients: Vec<Gradient>,
	textures: Vec<Texture>,
	gui_texture: UnsafeCell<Texture>,
	style_solid: Style,
	style_nasolid: Style,
	style_texture: Style,
	style_natexture: Style,
	style_gradient: Style,
	style_nagradient: Style,
	style_faded: Style,
	style_tinted: Style,
	style_natinted: Style,
	style_complex: Style,
	style_nacomplex: Style,
	style_gui: Style,
	projection: Matrix,
	clear_color: (f32, f32, f32),
	xyz: Vector,
	rotate_xyz: Vector,
}

impl Renderer {
	pub(crate) fn draw(&self, wh: (u16, u16),
		writer: &Fn(u16, u16) -> [u8; 4])
	{
/*		if wh != (unsafe {(*self.gui_texture.get()).w},
			unsafe {(*self.gui_texture.get()).h})
		{
			unsafe {
				*self.gui_texture.get() =
					new_texture(&self.vw, wh.0, wh.1);
			}
		}*/

		set_texture(&self.vw, unsafe { &mut *self.gui_texture.get() },
			writer);
	}

	pub(crate) fn new(rgb: Vector) -> Result<(Renderer, ::Window), String> {
		let (mut vw, window) = Vw::new(rgb)?;

		let solid_vert = super::asi::ShaderModule::new(
			&mut vw.connection, include_bytes!(
			"../shaders/res/solid-vert.spv"));
		let solid_frag = super::asi::ShaderModule::new(
			&mut vw.connection, include_bytes!(
			"../shaders/res/solid-frag.spv"));
		let texture_vert = super::asi::ShaderModule::new(
			&mut vw.connection, include_bytes!(
			"../shaders/res/texture-vert.spv"));
		let texture_frag = super::asi::ShaderModule::new(
			&mut vw.connection, include_bytes!(
			"../shaders/res/texture-frag.spv"));
		let gradient_vert = super::asi::ShaderModule::new(
			&mut vw.connection, include_bytes!(
			"../shaders/res/gradient-vert.spv"));
		let gradient_frag = super::asi::ShaderModule::new(
			&mut vw.connection, include_bytes!(
			"../shaders/res/gradient-frag.spv"));
		let faded_vert = super::asi::ShaderModule::new(
			&mut vw.connection, include_bytes!(
			"../shaders/res/faded-vert.spv"));
		let faded_frag = super::asi::ShaderModule::new(
			&mut vw.connection, include_bytes!(
			"../shaders/res/faded-frag.spv"));
		let tinted_vert = super::asi::ShaderModule::new(
			&mut vw.connection, include_bytes!(
			"../shaders/res/gradient-vert.spv"));
		let tinted_frag = super::asi::ShaderModule::new(
			&mut vw.connection, include_bytes!(
			"../shaders/res/gradient-frag.spv"));
		let complex_vert = super::asi::ShaderModule::new(
			&mut vw.connection, include_bytes!(
			"../shaders/res/gradient-vert.spv"));
		let complex_frag = super::asi::ShaderModule::new(
			&mut vw.connection, include_bytes!(
			"../shaders/res/gradient-frag.spv"));
		let gui_frag = super::asi::ShaderModule::new(
			&mut vw.connection, include_bytes!(
			"../shaders/res/gui-frag.spv"));
		let gui_vert = super::asi::ShaderModule::new(
			&mut vw.connection, include_bytes!(
			"../shaders/res/gui-vert.spv"));
		let style_solid = Style::new(&mut vw.connection, vw.render_pass,
			&solid_vert, &solid_frag, 0, 1, true);
		let style_nasolid = Style::new(&mut vw.connection,
			vw.render_pass,	&solid_vert, &solid_frag, 0, 1, false);
		let style_texture = Style::new(&mut vw.connection,
			vw.render_pass,	&texture_vert, &texture_frag, 1, 2,
			true);
		let style_natexture = Style::new(&mut vw.connection,
			vw.render_pass,	&texture_vert, &texture_frag, 1, 2,
			false);
		let style_gradient = Style::new(&mut vw.connection,
			vw.render_pass,	&gradient_vert, &gradient_frag, 0, 2,
			true);
		let style_nagradient = Style::new(&mut vw.connection,
			vw.render_pass,	&gradient_vert, &gradient_frag, 0, 2,
			false);
		let style_faded = Style::new(&mut vw.connection, vw.render_pass,
			&faded_vert, &faded_frag, 1, 2, true);
		let style_tinted = Style::new(&mut vw.connection,
			vw.render_pass,	&tinted_vert, &tinted_frag, 1, 2, true);
		let style_natinted = Style::new(&mut vw.connection,
			vw.render_pass, &tinted_vert, &tinted_frag, 1, 2,
			false);
		let style_complex = Style::new(&mut vw.connection,
			vw.render_pass, &complex_vert, &complex_frag, 1, 3,
			true);
		let style_nacomplex = Style::new(&mut vw.connection,
			vw.render_pass, &complex_vert, &complex_frag, 1, 3,
			false);
		let style_gui = Style::new(&mut vw.connection,
			vw.render_pass, &gui_vert, &gui_frag, ::std::u32::MAX,
			2, true);

		let ar = vw.connection.ar();
		let projection = super::base::projection(ar, 0.5 * PI);

		// Add GUI
		let wh = window.wh();
		let w = wh.0 as usize;
		let h = wh.1 as usize;
		let mut gui_texture = new_texture(&mut vw, wh.0, wh.1);
		set_texture(&mut vw, &mut gui_texture, &|_x, _y| { [0; 4] });

		let instance = unsafe {
			Sprite::new(
				&vw.connection,
				&style_gui,
				(), // unused
				Some(gui_texture.image.as_ref()
					.unwrap_or(&gui_texture
						.mappable_image).clone()),
				true, // 1 texure
				true, // gui
			)
		};

		let shape = unsafe {
			super::asi::new_buffer(&vw.connection,
				&[
					-1.0, -1.0, 0.0, 1.0,
					-1.0, 1.0, 0.0, 1.0,
					1.0, 1.0, 0.0, 1.0,
					1.0, -1.0, 0.0, 1.0,
				])
		};

		let texcoords = unsafe {
			super::asi::new_buffer(&vw.connection,
				&[
					0.0, 0.0, 0.0, 1.0,
					0.0, 1.0, 0.0, 1.0,
					1.0, 1.0, 0.0, 1.0,
					1.0, 0.0, 0.0, 1.0,
				])
		};

		let gui = Shape {
			instance,
			num_buffers: 2,
			buffers: [
				shape.buffer(),
				texcoords.buffer(),
				unsafe { mem::uninitialized() }
			],
			fans: vec![(0, 4)], // make a rectangle (4 vertices)
			transform: unsafe { ::std::mem::uninitialized() },
		};

		::std::mem::forget(shape);
		::std::mem::forget(texcoords);

		let mut renderer = Renderer {
			earlier: Instant::now(),
			vw, ar, projection,
			alpha_ind: Vec::new(),
			opaque_ind: Vec::new(),
			alpha_vec: UnsafeCell::new(Vec::new()),
			opaque_vec: UnsafeCell::new(Vec::new()),
			gui,
			gradients: Vec::new(),
			models: Vec::new(),
			texcoords: Vec::new(),
			textures: Vec::new(),
			gui_texture: UnsafeCell::new(gui_texture),
			style_solid, style_nasolid,
			style_texture, style_natexture,
			style_gradient, style_nagradient,
			style_faded,
			style_tinted, style_natinted,
			style_complex, style_nacomplex, style_gui,
			clear_color: (rgb.x, rgb.y, rgb.z),
			xyz: vector!(0.0, 0.0, 0.0),
			rotate_xyz: vector!(0.0, 0.0, 0.0),
		};

		Ok((renderer, window))
	}

	pub fn bg_color(&mut self, rgb: Vector) {
		self.clear_color = (rgb.x, rgb.y, rgb.z);
		self.vw.connection.color(rgb);
	}

	pub fn update(&mut self) -> f32 {
		let rendering_complete_sem = unsafe {
			super::asi::new_semaphore(&self.vw.connection)
		};

		let fence = unsafe { super::asi::fence::new(&self.vw.connection) };
		let next_image_index = unsafe {
			super::asi::get_next_image(&self.vw.connection, fence)
		};

		unsafe {
			super::asi::draw_begin(&self.vw.connection,
				self.vw.render_pass,
				self.vw.present_images[next_image_index as usize],
				self.vw.frame_buffers[next_image_index as usize]
			);
		}

		// sort nearest
		super::base::zsort(&mut self.opaque_ind, unsafe{::std::mem::transmute::<_, &mut Vec<Shape>>(self.opaque_vec.get())},
			true, self.xyz);
		for shape in self.opaque_ind.iter() {
			let shape = &unsafe{::std::mem::transmute::<_, &mut Vec<Shape>>(self.opaque_vec.get())}[*shape as usize];
			draw_shape(&self.vw.connection, shape);
		}

		// sort farthest
		super::base::zsort(&mut self.alpha_ind, unsafe{::std::mem::transmute::<_, &mut Vec<Shape>>(self.alpha_vec.get())},
			false, self.xyz);
		for shape in self.alpha_ind.iter() {
			let shape = &unsafe{::std::mem::transmute::<_, &mut Vec<Shape>>(self.alpha_vec.get())}[*shape as usize];
			draw_shape(&self.vw.connection, shape);
		}

		draw_shape(&self.vw.connection, &self.gui);

		unsafe {
			super::asi::end_render_pass(&self.vw.connection);

			super::asi::pipeline_barrier(&self.vw.connection,
				self.vw.present_images[next_image_index as usize]);

			super::asi::end_cmdbuff(&self.vw.connection);
		}

		unsafe { // Drop fence when it's done use
			let fence = super::asi::Fence::new(&self.vw.connection);

			super::asi::queue_submit(&self.vw.connection,
				&fence,
				VkPipelineStage::BottomOfPipe,
				Some(rendering_complete_sem));
				
			super::asi::wait_fence(&self.vw.connection, &fence);
		}

		unsafe {
			// Actually present the image to the screen.
			super::asi::queue_present(&self.vw.connection,
				next_image_index);

			super::asi::fence::wait(&self.vw.connection, fence);
			super::asi::fence::drop(&self.vw.connection, fence);

			super::asi::drop_semaphore(&self.vw.connection,
				rendering_complete_sem);

			super::asi::wait_idle(&self.vw.connection);
		}

		// Get the time step for the next frame.
		let new = Instant::now();
		let r = new.duration_since(self.earlier).subsec_nanos() as f32
			/ 1_000_000_000.0;
		self.earlier = new;
		r
	}

	pub fn resize(&mut self, size: (u16, u16)) {
		swapchain_delete(&mut self.vw);
		let (depth_image, ms_image, render_pass)
			= swapchain_resize(&self.vw.connection,
				&mut self.vw.image_count,
				&mut self.vw.present_images,
				&mut self.vw.present_image_views,
				&mut self.vw.frame_buffers);

		self.ar = size.0 as f32 / size.1 as f32;
		self.vw.depth_image = depth_image;
		self.vw.ms_image = ms_image;
		self.vw.render_pass = render_pass;

		self.projection = super::base::projection(self.ar, 0.5 * PI);

		println!("YAA");
	}

	pub fn texture(&mut self, width: u16, height: u16, rgba: &[u8])
		-> usize
	{
		let mut texture = new_texture(&mut self.vw, width, height);

		set_texture(&mut self.vw, &mut texture, &|x, y| {
			let r = rgba[(width as usize * y as usize + x as usize) * 4 + 0];
			let g = rgba[(width as usize * y as usize + x as usize) * 4 + 1];
			let b = rgba[(width as usize * y as usize + x as usize) * 4 + 2];
			let a = rgba[(width as usize * y as usize + x as usize) * 4 + 3];

			[r, g, b, a]
		});

		let a = self.textures.len();
		self.textures.push(texture);
		a
	}

	pub fn set_texture(&mut self, texture: usize, rgba: &[u8]) {
		let width = self.textures[texture].w;
		set_texture(&mut self.vw, &mut self.textures[texture], &|x, y| {
			let r = rgba[(width as usize * y as usize + x as usize) * 4 + 0];
			let g = rgba[(width as usize * y as usize + x as usize) * 4 + 1];
			let b = rgba[(width as usize * y as usize + x as usize) * 4 + 2];
			let a = rgba[(width as usize * y as usize + x as usize) * 4 + 3];

			[r, g, b, a]
		});
	}

	pub fn resize_texture(&mut self, texture_id: usize, width: u16,
		height: u16, rgba: &[u8])
	{
		println!("RESIZE TX");
		let mut texture = new_texture(&mut self.vw, width, height);
		set_texture(&mut self.vw, &mut texture, &|x, y| {
			let r = rgba[(width as usize * y as usize + x as usize) * 4 + 0];
			let g = rgba[(width as usize * y as usize + x as usize) * 4 + 1];
			let b = rgba[(width as usize * y as usize + x as usize) * 4 + 2];
			let a = rgba[(width as usize * y as usize + x as usize) * 4 + 3];

			[r, g, b, a]
		});
		self.textures[texture_id] = texture;
	}

	/// Push a model (collection of vertices) into graphics memory.
	pub fn model(&mut self, vertices: &[f32], fans: Vec<(u32, u32)>)
		-> usize
	{
		let shape = unsafe {
			super::asi::new_buffer(&self.vw.connection,
				vertices)
		};

		let a = self.models.len();

		self.models.push(Model {
			shape,
			vertex_count: vertices.len() as u32 / 4,
			fans,
		});

		a
	}

	/// Push texture coordinates (collection of vertices) into graphics
	/// memory.
	pub fn texcoords(&mut self, texcoords: &[(f32, f32)]) -> usize {
		let mut buffer = Vec::with_capacity(texcoords.len() * 4);

		for i in texcoords {
			buffer.push(i.0);
			buffer.push(i.1);
			buffer.push(1.0);
			buffer.push(1.0);
		}

		let vertex_buffer = unsafe {
			super::asi::new_buffer(
				&self.vw.connection,
				buffer.as_slice(),
			)
		};

		let a = self.texcoords.len();

		self.texcoords.push(TexCoords {
			vertex_buffer,
			vertex_count: texcoords.len() as u32,
		});

		a
	}

	/// Push colors per vertex into graphics memory.
	pub fn colors(&mut self, colors: &[f32]) -> usize {
		let vertex_buffer = unsafe {
			super::asi::new_buffer(
				&self.vw.connection,
				colors,
			)
		};

		let a = self.gradients.len();

		self.gradients.push(Gradient {
			vertex_buffer,
			vertex_count: colors.len() as u32 / 4,
		});

		a
	}

	pub(crate) fn textured(&mut self, model: usize, mat: Matrix,
		texture: usize, texcoords: usize, alpha: bool,
		fog: bool, camera: bool) -> ShapeHandle
	{
		if self.models[model].vertex_count
			!= self.texcoords[texcoords].vertex_count
		{
			panic!("TexCoord length doesn't match vertex length");
		}

		// Add an instance
		let instance = unsafe {
			Sprite::new(
				&self.vw.connection,
				if alpha {
					&self.style_texture
				} else {
					&self.style_natexture
				},
				TransformFullUniform {
					mat4: (self.projection * mat).into(),
				},
				Some(self.textures[texture].image.as_ref()
					.unwrap_or(&self.textures[texture]
						.mappable_image).clone()),
				true, // 1 texure
				false, // not gui
			)
		};

		let shape = Shape {
			instance,
			num_buffers: 2,
			buffers: [
				self.models[model].shape.buffer(),
				self.texcoords[texcoords].vertex_buffer.buffer(),
				unsafe { mem::uninitialized() }
			],
			fans: self.models[model].fans.clone(),
			transform: mat,
		};

		if alpha {
			let index = unsafe{::std::mem::transmute::<_, &mut Vec<Shape>>(self.alpha_vec.get())}.len() as u32;
			unsafe{::std::mem::transmute::<_, &mut Vec<Shape>>(self.alpha_vec.get())}.push(shape);
			self.alpha_ind.push(index);
			ShapeHandle::Alpha(index)
		} else {
			let index = unsafe{::std::mem::transmute::<_, &mut Vec<Shape>>(self.opaque_vec.get())}.len() as u32;
			unsafe{::std::mem::transmute::<_, &mut Vec<Shape>>(self.opaque_vec.get())}.push(shape);
			self.opaque_ind.push(index);
			ShapeHandle::Opaque(index)
		}
	}

	pub(crate) fn solid(&mut self, model: usize, mat: Matrix,
		color: [f32; 4], alpha: bool, fog: bool, camera: bool)
		-> ShapeHandle
	{
		// Add an instance
		let instance = unsafe {
			Sprite::new(
				&self.vw.connection,
				if alpha {
					&self.style_solid
				} else {
					&self.style_nasolid
				},
				TransformAndColorUniform {
					vec4: color,
					mat4: (self.projection * mat).into(),
				},
				None,
				false, // no texure
				false, // not gui
			)
		};

		let shape = Shape {
			instance,
			num_buffers: 1,
			buffers: [
				self.models[model].shape.buffer(),
				unsafe { mem::uninitialized() },
				unsafe { mem::uninitialized() }
			],
			fans: self.models[model].fans.clone(),
			transform: mat,
		};

		if alpha {
			let index = unsafe{::std::mem::transmute::<_, &mut Vec<Shape>>(self.alpha_vec.get())}.len() as u32;
			unsafe{::std::mem::transmute::<_, &mut Vec<Shape>>(self.alpha_vec.get())}.push(shape);
			self.alpha_ind.push(index);
			ShapeHandle::Alpha(index)
		} else {
			let index = unsafe{::std::mem::transmute::<_, &mut Vec<Shape>>(self.opaque_vec.get())}.len() as u32;
			unsafe{::std::mem::transmute::<_, &mut Vec<Shape>>(self.opaque_vec.get())}.push(shape);
			self.opaque_ind.push(index);
			ShapeHandle::Opaque(index)
		}
	}

	pub(crate) fn gradient(&mut self, model: usize, mat: Matrix,
		colors: usize, alpha: bool, fog: bool, camera: bool)
		-> ShapeHandle
	{
		if self.models[model].vertex_count
			!= self.gradients[colors].vertex_count
		{
			panic!("TexCoord length doesn't match gradient length");
		}

		// Add an instance
		let instance = unsafe {
			Sprite::new(
				&self.vw.connection,
				if alpha {
					&self.style_gradient
				} else {
					&self.style_nagradient
				},
				TransformFullUniform {
					mat4: (self.projection * mat).into(),
				},
				None,
				false, // no texure
				false, // not gui
			)
		};

		let shape = Shape {
			instance,
			num_buffers: 2,
			buffers: [
				self.models[model].shape.buffer(),
				self.gradients[colors].vertex_buffer.buffer(),
				unsafe { mem::uninitialized() }
			],
			fans: self.models[model].fans.clone(),
			transform: mat,
		};

		if alpha {
			let index = unsafe{::std::mem::transmute::<_, &mut Vec<Shape>>(self.alpha_vec.get())}.len() as u32;
			unsafe{::std::mem::transmute::<_, &mut Vec<Shape>>(self.alpha_vec.get())}.push(shape);
			self.alpha_ind.push(index);
			ShapeHandle::Alpha(index)
		} else {
			let index = unsafe{::std::mem::transmute::<_, &mut Vec<Shape>>(self.opaque_vec.get())}.len() as u32;
			unsafe{::std::mem::transmute::<_, &mut Vec<Shape>>(self.opaque_vec.get())}.push(shape);
			self.opaque_ind.push(index);
			ShapeHandle::Opaque(index)
		}
	}

	pub(crate) fn faded(&mut self, model: usize, mat: Matrix,
		texture: usize, texcoords: usize, fade_factor: f32, fog: bool,
		camera: bool) -> ShapeHandle
	{
		if self.models[model].vertex_count
			!= self.texcoords[texcoords].vertex_count
		{
			panic!("TexCoord length doesn't match vertex length");
		}

		// Add an instance
		let instance = unsafe {
			Sprite::new(
				&self.vw.connection,
				&self.style_faded,
				TransformAndFadeUniform {
					mat4: (self.projection * mat).into(),
					fade: fade_factor,
				},
				Some(self.textures[texture].image.as_ref()
					.unwrap_or(&self.textures[texture]
						.mappable_image).clone()),
				true, // 1 texure
				false, // not gui
			)
		};

		let shape = Shape {
			instance,
			num_buffers: 2,
			buffers: [
				self.models[model].shape.buffer(),
				self.texcoords[texcoords].vertex_buffer.buffer(),
				unsafe { mem::uninitialized() }
			],
			fans: self.models[model].fans.clone(),
			transform: mat,
		};

		let index = unsafe{::std::mem::transmute::<_, &mut Vec<Shape>>(self.alpha_vec.get())}.len() as u32;
		unsafe{::std::mem::transmute::<_, &mut Vec<Shape>>(self.alpha_vec.get())}.push(shape);
		self.alpha_ind.push(index);
		ShapeHandle::Alpha(index)
	}

	pub(crate) fn tinted(&mut self, model: usize, mat: Matrix,
		texture: usize, texcoords: usize, color: [f32; 4],
		alpha: bool, fog: bool, camera: bool)
		-> ShapeHandle
	{
		if self.models[model].vertex_count
			!= self.texcoords[texcoords].vertex_count
		{
			panic!("TexCoord length doesn't match vertex length");
		}

		// Add an instance
		let instance = unsafe {
			Sprite::new(
				&self.vw.connection,
				if alpha {
					&self.style_tinted
				} else {
					&self.style_natinted
				},
				TransformAndColorUniform {
					mat4: (self.projection * mat).into(),
					vec4: color,
				},
				Some(self.textures[texture].image.as_ref()
					.unwrap_or(&self.textures[texture]
						.mappable_image).clone()),
				true, // 1 texure
				false, // not gui
			)
		};

		let shape = Shape {
			instance,
			num_buffers: 2,
			buffers: [
				self.models[model].shape.buffer(),
				self.texcoords[texcoords].vertex_buffer.buffer(),
				unsafe { mem::uninitialized() }
			],
			fans: self.models[model].fans.clone(),
			transform: mat,
		};

		if alpha {
			let index = unsafe{::std::mem::transmute::<_, &mut Vec<Shape>>(self.alpha_vec.get())}.len() as u32;
			unsafe{::std::mem::transmute::<_, &mut Vec<Shape>>(self.alpha_vec.get())}.push(shape);
			self.alpha_ind.push(index);
			ShapeHandle::Alpha(index)
		} else {
			let index = unsafe{::std::mem::transmute::<_, &mut Vec<Shape>>(self.opaque_vec.get())}.len() as u32;
			unsafe{::std::mem::transmute::<_, &mut Vec<Shape>>(self.opaque_vec.get())}.push(shape);
			self.opaque_ind.push(index);
			ShapeHandle::Opaque(index)
		}
	}

	pub(crate) fn complex(&mut self, model: usize, mat: Matrix,
		texture: usize, texcoords: usize, colors: usize, alpha: bool,
		fog: bool, camera: bool) -> ShapeHandle
	{
		if self.models[model].vertex_count
			!= self.texcoords[texcoords].vertex_count ||
			self.models[model].vertex_count
			!= self.gradients[colors].vertex_count
		{
			panic!("TexCoord length doesn't match vertex length");
		}

		// Add an instance
		let instance = unsafe {
			Sprite::new(
				&self.vw.connection,
				if alpha {
					&self.style_complex
				} else {
					&self.style_nacomplex
				},
				TransformFullUniform {
					mat4: (self.projection * mat).into(),
				},
				Some(self.textures[texture].image.as_ref()
					.unwrap_or(&self.textures[texture]
						.mappable_image).clone()),
				true, // 1 texure
				false, // not gui
			)
		};

		let shape = Shape {
			instance,
			num_buffers: 3,
			buffers: [
				self.models[model].shape.buffer(),
				self.texcoords[texcoords].vertex_buffer.buffer(),
				self.gradients[colors].vertex_buffer.buffer(),
			],
			fans: self.models[model].fans.clone(),
			transform: mat,
		};

		if alpha {
			let index = unsafe{::std::mem::transmute::<_, &mut Vec<Shape>>(self.alpha_vec.get())}.len() as u32;
			unsafe{::std::mem::transmute::<_, &mut Vec<Shape>>(self.alpha_vec.get())}.push(shape);
			self.alpha_ind.push(index);
			ShapeHandle::Alpha(index)
		} else {
			let index = unsafe{::std::mem::transmute::<_, &mut Vec<Shape>>(self.opaque_vec.get())}.len() as u32;
			unsafe{::std::mem::transmute::<_, &mut Vec<Shape>>(self.opaque_vec.get())}.push(shape);
			self.opaque_ind.push(index);
			ShapeHandle::Opaque(index)
		}
	}

	pub(crate) fn drop_shape(&mut self, shape: ShapeHandle) {
		match shape {
			ShapeHandle::Opaque(x) => {
				let index = self.opaque_ind.iter()
					.position(|y| *y == x).unwrap();
				self.opaque_ind.remove(index);
			}
			ShapeHandle::Alpha(x) => {
				let index = self.alpha_ind.iter()
					.position(|y| *y == x).unwrap();
				self.alpha_ind.remove(index);
			}
		}
	}

	pub(crate) fn transform(&self, shape: &ShapeHandle, mat: Matrix) {
		let uniform = TransformUniform {
			mat4: (self.projection * mat).into(),
		};

		match shape {
			ShapeHandle::Opaque(x) => {
				let x = *x as usize; // for indexing
				(unsafe{::std::mem::transmute::<_, &mut Vec<Shape>>(self.opaque_vec.get())})[x].transform = mat;
				ffi::copy_memory(&self.vw.connection,
					unsafe{::std::mem::transmute::<_, &mut Vec<Shape>>(self.opaque_vec.get())}[x].instance.uniform_memory.memory(),
					&uniform);
			}
			ShapeHandle::Alpha(x) => {
				let x = *x as usize; // for indexing
				(unsafe{::std::mem::transmute::<_, &mut Vec<Shape>>(self.alpha_vec.get())})[x].transform = mat;
				ffi::copy_memory(&self.vw.connection,
					unsafe{::std::mem::transmute::<_, &mut Vec<Shape>>(self.alpha_vec.get())}[x].instance.uniform_memory.memory(),
					&uniform);
			}
		}
	}
}

impl Drop for Renderer {
	fn drop(&mut self) -> () {
		swapchain_delete(&mut self.vw);
	}
}
