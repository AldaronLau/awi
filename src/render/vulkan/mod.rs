// Copyright Jeron A. Lau 2018.
// Dual-licensed under either the MIT License or the Boost Software License,
// Version 1.0.  (See accompanying file LICENSE_1_0.txt or copy at
// https://www.boost.org/LICENSE_1_0.txt)

//! Vulkan implementation for adi_gpu.

// #![no_std]

extern crate libc;

/// Transform represents a transformation matrix.
pub(crate) mod renderer;
mod asi;

pub use self::base::Shape;
pub use self::base::Gradient;
pub use self::base::Model;
pub use self::base::TexCoords;
pub use self::base::Texture;

use super::base;
use super::base::*;

/// To render anything with adi_gpu, you have to make a `Display`
pub struct Display {
	window: ::Window,
	renderer: renderer::Renderer,
}

pub fn new() -> Result<Box<Display>, String> {
	let (renderer, window) = renderer::Renderer::new(
		vec3!(0.0, 0.0, 0.0)
	)?;

	Ok(Box::new(Display { window, renderer }))
}

impl base::Display for Display {
	fn color(&mut self, color: (u8, u8, u8)) {
		self.renderer.bg_color(vec3!(color.0 as f32 / 255.0,
			color.1 as f32 / 255.0, color.2 as f32 / 255.0));
	}

	fn input(&mut self) -> Option<base::Event> {
		self.window.update()
	}

	fn update(&mut self) -> f32 {
		self.renderer.update()
	}

	fn camera(&mut self, xyz: Vec3, rotate_xyz: Vec3) {
		self.renderer.set_camera(xyz, rotate_xyz);
		self.renderer.camera();
	}

	fn model(&mut self, vertices: &[f32], fans: Vec<(u32, u32)>) -> Model {
		Model(self.renderer.model(vertices, fans))
	}

	fn fog(&mut self, fog: Option<(f32, f32)>) -> () {
		if let Some(fog) = fog {
			self.renderer.fog(fog);
		} else {
			self.renderer.fog((::std::f32::MAX, 0.0));
		}
	}

	fn texture(&mut self, wh: (u16,u16), graphic: &VFrame) -> Texture {
		let (w, h) = wh;
		let pixels = graphic.0.as_slice();

		Texture(self.renderer.texture(w, h, pixels), wh.0, wh.1)
	}

	fn gradient(&mut self, colors: &[f32]) -> Gradient {
		Gradient(self.renderer.colors(colors))
	}

	fn texcoords(&mut self, texcoords: &[(f32, f32)]) -> TexCoords {
		TexCoords(self.renderer.texcoords(texcoords))
	}

	fn set_texture(&mut self, texture: &mut Texture, wh: (u16,u16),
		graphic: &VFrame)
	{
		if texture.1 == wh.0 && texture.2 == wh.1 {
			self.renderer.set_texture(texture.0,
				graphic.0.as_slice());
		} else {
			// resize
			self.renderer.resize_texture(texture.0, wh.0, wh.1,
				graphic.0.as_slice());
		}
	}

	#[inline(always)]
	fn shape_solid(&mut self, model: &Model, transform: Transform,
		color: [f32; 4], blending: bool, fog: bool,
		camera: bool) -> Shape
	{
		base::new_shape(self.renderer.solid(model.0, transform, color,
			blending, fog, camera))
	}

	#[inline(always)]
	fn shape_gradient(&mut self, model: &Model, transform: Transform,
		colors: Gradient, blending: bool, fog: bool,
		camera: bool) -> Shape
	{
		base::new_shape(self.renderer.gradient(model.0, transform,
			colors.0, blending, fog, camera))
	}

	#[inline(always)]
	fn shape_texture(&mut self, model: &Model, transform: Transform,
		texture: &Texture, tc: TexCoords, blending: bool,
		fog: bool, camera: bool) -> Shape
	{
		base::new_shape(self.renderer.textured(model.0, transform,
			texture.0, tc.0, blending, fog, camera))
	}

	#[inline(always)]
	fn shape_faded(&mut self, model: &Model, transform: Transform,
		texture: &Texture, tc: TexCoords, alpha: f32,
		fog: bool, camera: bool) -> Shape
	{
		base::new_shape(self.renderer.faded(model.0, transform,
			texture.0, tc.0, alpha, fog, camera))
	}

	#[inline(always)]
	fn shape_tinted(&mut self, model: &Model, transform: Transform,
		texture: &Texture, tc: TexCoords, tint: [f32; 4], blending: bool,
		fog: bool, camera: bool) -> Shape
	{
		base::new_shape(self.renderer.tinted(model.0, transform,
			texture.0, tc.0, tint, blending, fog, camera))
	}

	#[inline(always)]
	fn shape_complex(&mut self, model: &Model, transform: Transform,
		texture: &Texture, tc: TexCoords, tints: Gradient,
		blending: bool, fog: bool, camera: bool) -> Shape
	{
		base::new_shape(self.renderer.complex(model.0, transform,
			texture.0, tc.0, tints.0, blending, fog, camera))
	}

	#[inline(always)]
	fn drop_shape(&mut self, shape: &Shape) {
		self.renderer.drop_shape(get_shape(&shape));
	}

	fn transform(&mut self, shape: &Shape, transform: Transform) {
		self.renderer.transform(&base::get_shape(shape), transform);
	}

	fn resize(&mut self, wh: (u16, u16)) -> () {
		self.renderer.resize(wh);
	}

	fn wh(&self) -> (u16, u16) {
		self.window.wh()
	}
}
