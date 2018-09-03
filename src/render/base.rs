// Copyright Jeron A. Lau 2018.
// Dual-licensed under either the MIT License or the Boost Software License,
// Version 1.0.  (See accompanying file LICENSE_1_0.txt or copy at
// https://www.boost.org/LICENSE_1_0.txt)

//! This library is the base library for implementations of the adi_gpu api.
//! If you would like to make your own implementation of the api, you can use
//! this library as a dependency.

extern crate ami;

use std::cmp::Ordering;

pub use	afi;
pub use afi::VFrame;
pub use Event;
pub use self::ami::*;
pub use std::f32::consts::PI;

/// A trait for a `Display`
pub trait Display {
	/// Set the background color for the `Display`.
	///
	/// * `color`: The background color for the display.
	fn color(&mut self, color: (u8, u8, u8)) -> ();

	/// Get input, if there's any.
	fn input(&mut self) -> Option<Event>;

	/// Update the `Display`.
	fn update(&mut self) -> f32;

	/// Create a new `Model` for this `Display`.
	fn model(&mut self, vertices: &[f32], fans: Vec<(u32, u32)>) -> Model;

	/// Create a new `Texture` for this `Display`.
	fn texture(&mut self, wh: (u16,u16), graphic: &VFrame) -> Texture;

	/// Create a new `Gradient` for this `Display`.
	fn gradient(&mut self, colors: &[f32]) -> Gradient;

	/// Create new `TexCoords` for this `Display`.
	fn texcoords(&mut self, texcoords: &[(f32, f32)]) -> TexCoords;

	/// Set the pixels for a `Texture`.
	fn set_texture(&mut self, texture: &mut Texture, wh: (u16,u16),
		graphic: &VFrame) -> ();

	/// Create a new shape with a solid color.
	fn shape_solid(&mut self, model: &Model, transform: Matrix,
		color: [f32; 4], blending: bool, fog: bool, camera: bool)
		-> Shape;

	/// Create a new shape shaded by a gradient (1 color per vertex).
	fn shape_gradient(&mut self, model: &Model, transform: Matrix,
		gradient: Gradient, blending: bool, fog: bool, camera: bool)
		-> Shape;

	/// Create a new shape shaded by a texture using texture coordinates.
	///
	/// Texture Coordinates follow this format (X, Y, UNUSED(1.0), ALPHA)
	fn shape_texture(&mut self, model: &Model, transform: Matrix,
		texture: &Texture, tc: TexCoords, blending: bool,
		fog: bool, camera: bool) -> Shape;

	/// Create a new shape shaded by a texture using texture coordinates
	/// and alpha.
	///
	/// Texture Coordinates follow this format (X, Y, UNUSED(1.0), ALPHA)
	fn shape_faded(&mut self, model: &Model, transform: Matrix,
		texture: &Texture, tc: TexCoords, alpha: f32,
		fog: bool, camera: bool) -> Shape;

	/// Create a new shape shaded by a texture using texture coordinates
	/// and tint.
	///
	/// Texture Coordinates follow this format (X, Y, UNUSED(1.0), ALPHA)
	fn shape_tinted(&mut self, model: &Model, transform: Matrix,
		texture: &Texture, tc: TexCoords, tint: [f32; 4],
		blending: bool, fog: bool, camera: bool) -> Shape;

	/// Create a new shape shaded by a texture using texture coordinates
	/// and tint per vertex.
	///
	/// Texture Coordinates follow this format (X, Y, UNUSED(1.0), ALPHA)
	fn shape_complex(&mut self, model: &Model, transform: Matrix,
		texture: &Texture, tc: TexCoords,
		gradient: Gradient, blending: bool,
		fog: bool, camera: bool) -> Shape;

	/// Drop a shape (don't draw it anymore).
	fn drop_shape(&mut self, shape: &Shape);

	/// Transform the shape.
	fn transform(&self, shape: &Shape, transform: Matrix);

	/// Resize the display.
	fn resize(&mut self, wh: (u16, u16)) -> ();

	/// Get the width and height of the window, as a tuple.
	fn wh(&self) -> (u16, u16);
}

/// Handle for shape.
#[derive(Clone)]
pub(crate) enum ShapeHandle {
	Alpha(u32),
	Opaque(u32),
}

/// A renderable object that exists on the `Display`.
pub struct Shape(ShapeHandle);

/// A list of vertices that make a shape.
#[derive(Copy, Clone)]
pub struct Model(pub usize); // TODO: unsafe

/// A list of colors to be paired with vertices.
#[derive(Copy, Clone)]
pub struct Gradient(pub usize); // TODO: unsafe

/// A list of texture coordinates to be paired with vertices.
#[derive(Copy, Clone)]
pub struct TexCoords(pub usize); // TODO: unsafe

/// A Texture
pub struct Texture(pub usize, pub u16, pub u16); // TODO: unsafe

/// Create a new shape
pub(crate) fn new_shape(i: ShapeHandle) -> Shape {
	Shape(i)
}

/// Get the index of a shape
pub(crate) fn get_shape(s: &Shape) -> ShapeHandle {
	s.0.clone()
}

/// Generate a projection matrix.
pub fn projection(ratiox: f32, fovy: f32) -> Matrix {
	matrix!()
		// TODO: is this the right order to multiply stuff?
		.m(Matrix::new(
			1.0, 0.0, 0.0, 0.0,
			0.0, 1.0, 0.0, 0.0,
			0.0, 0.0, -1.0, 0.0,
			0.0, 0.0, 0.0, 1.0,
		))
		.m(Matrix::finite_perspective_projection(
			fovy,
			ratiox,
			0.1, // Near
			100.0, // Far
		))
}

pub trait Point {
	fn point(&self) -> Vector;
}

/// Sort by distance.  nr => true if Near Sort, nr => false if Far Sort
pub fn zsort<T: Point>(sorted: &mut Vec<u32>, points: &Vec<T>, nr: bool,
	position: Vector)
{
	sorted.sort_unstable_by(|a, b| {
		let p1 = points[*a as usize].point() - position;
		let p2 = points[*b as usize].point() - position;

		if p1.length() > p2.length() {
			if nr { Ordering::Greater } else { Ordering::Less }
		} else if p1.length() < p2.length() {
			if nr { Ordering::Less } else { Ordering::Greater }
		} else {
			Ordering::Equal
		}
	});
}
