// Copyright Jeron A. Lau 2017-2018.
// Dual-licensed under either the MIT License or the Boost Software License,
// Version 1.0.  (See accompanying file LICENSE_1_0.txt or copy at
// https://www.boost.org/LICENSE_1_0.txt)

//! Screen Module (Computer Monitor / Phone Screen / etc.)

#[cfg(not(target_arch="wasm32"))]
use render::{Display, new_display};
#[cfg(not(target_arch="wasm32"))]
pub use render::{Shape, Gradient, Model, Texture, TexCoords};

use render::{Event};
use afi::{VFrame, PathOp};

use Matrix;
use Vector;

#[cfg(target_arch="wasm32")] mod win {mod wasm32; pub use self::wasm32::*;}

#[cfg(target_arch="wasm32")]
use self::win::{Display};
#[cfg(target_arch="wasm32")]
pub use self::win::{Shape, Gradient, Model, Texture, TexCoords};

/// A Window to the Screen.
pub struct Screen<Ctx> where Ctx: Default {
	// The platform-dependant implementation.
	#[cfg(not(target_arch="wasm32"))]
	display: Box<Display>,
	#[cfg(target_arch="wasm32")]
	display: Display,

	/* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * */

	// For Vector Graphics Rendering.
	vframe: VFrame,
	// program context.
	pub ctx: Ctx,
	// current function pointer.
	run: fn(&mut Screen<Ctx>, Event, f32),
	running: bool,
}

/// An error in the connection to the screen.
#[derive(Debug)]
pub enum ScreenError {
	
}

impl<Ctx> Screen<Ctx> where Ctx: Default {
	/// Start the program.
	pub fn start(run: fn(&mut Screen<Ctx>, Event, f32))
		-> Result<(), ScreenError>
	{
		let mut screen = Screen::new(run);
		let mut dt = 0.0;

		while screen.running {
			while let Some(input) = screen.display.input() {
				(screen.run)(&mut screen, input, dt);
			}

			(screen.run)(&mut screen, Event::Timestep, dt);
			dt = screen.display.update();
		}

		Ok(())
	}

	/// Open a new Window to the Screen.
	fn new(run: fn(&mut Screen<Ctx>, Event, f32)) -> Self {
		let mut screen = Screen {
			ctx: Ctx::default(),
			vframe: VFrame(vec![]),

			#[cfg(not(target_arch="wasm32"))]
			display: new_display().unwrap(),
			#[cfg(target_arch="wasm32")]
			display: Display::new(),

			run,
			running: true,
		};

		let wh = screen.display.wh();
		screen.vframe.0.resize((wh.0 as usize * wh.1 as usize) * 4, 0);

		screen
	}

	/// Stop the program.
	pub fn stop(&mut self) {
		::std::process::exit(0);
	}

	/// Switch the run function
	pub fn switch(&mut self, run: fn(&mut Screen<Ctx>, Event, f32)) {
		self.run = run;
	}

	/// Update the clear color of the Window.
	pub fn clear(&mut self, color: (u8, u8, u8)) {
		self.display.color(color)
	}

	/// Upload a model to the GPU.
	pub fn model(&mut self, vertices: &[f32], fans: Vec<(u32, u32)>)
		-> Model
	{
		self.display.model(vertices, fans)
	}

	/// Upload a texture to the GPU.
	pub fn texture(&mut self, wh: (u16, u16), graphic: &VFrame) -> Texture {
		self.display.texture(wh, graphic)
	}

	/// Create gradient object.
	pub fn gradient(&mut self, colors: &[f32]) -> Gradient {
		self.display.gradient(colors)
	}

	/// Create texture coordinate object.
	pub fn texcoords(&mut self, texcoords: &[(f32, f32)]) -> TexCoords {
		self.display.texcoords(texcoords)
	}

	/// Set the pixels of a texture to something other than the original.
	pub fn set_texture(&mut self, texture: &mut Texture, wh: (u16, u16),
		graphic: &VFrame)
	{
		self.display.set_texture(texture, wh, graphic)
	}

	/// Make a shape with solid color.
	pub fn shape_solid(&mut self, model: &Model, matrix: Matrix,
		color: [f32; 4], blending: bool, fog: bool, camera: bool)
		-> Shape	
	{
		self.display.shape_solid(model, matrix, color, blending, fog,
			camera)
	}

	/// Make a shape with gradient
	pub fn shape_gradient(&mut self, model: &Model, matrix: Matrix,
		gradient: Gradient, blending: bool, fog: bool, camera: bool)
		-> Shape
	{
		self.display.shape_gradient(model, matrix, gradient,
			blending, fog, camera)
	}

	/// Make a shape will solid texture.
	pub fn shape_texture(&mut self, model: &Model, matrix: Matrix,
		texture: &Texture, tc: TexCoords, blending: bool, fog: bool, 
		camera: bool) -> Shape
	{
		self.display.shape_texture(model, matrix, texture, tc,
			blending, fog, camera)
	}

	/// Make a shape will texture and transparency
	pub fn shape_faded(&mut self, model: &Model, matrix: Matrix,
		texture: &Texture, tc: TexCoords, alpha: f32, fog: bool,
		camera: bool) -> Shape
	{
		self.display.shape_faded(model, matrix, texture, tc, alpha,
			fog, camera)
	}

	/// Make a shape with texture, and tint (color)
	pub fn shape_tinted(&mut self, model: &Model, matrix: Matrix, 
		texture: &Texture, tc: TexCoords, tint: [f32; 4],
		blending: bool, fog: bool, camera: bool) -> Shape
	{
		self.display.shape_tinted(model, matrix, texture, tc, tint,
			blending, fog, camera)
	}

	/// Make a shape with texture, and gradent
	pub fn shape_complex(&mut self, model: &Model, matrix: Matrix, 
		texture: &Texture, tc: TexCoords, gradient: Gradient,
		blending: bool, fog: bool, camera: bool) -> Shape
	{
		self.display.shape_complex(model, matrix, texture, tc,
			gradient, blending, fog, camera)
	}

	/// Stop drawing a shape.
	pub fn drop_shape(&mut self, shape: &Shape) {
		self.display.drop_shape(shape)
	}

	/// Apply a matrix transform to a shape.
	pub fn transform(&self, shape: &Shape, matrix: Matrix) {
		self.display.transform(shape, matrix)
	}

	/// Call this function when you get a resize event.
	pub fn resize(&mut self, wh: (u16, u16)) {
		self.vframe.0.resize((wh.0 as usize * wh.1 as usize) * 4, 0);
		self.display.resize(wh);
	}

	/// Get the width and height of the window.
	pub fn wh(&self) -> (u16, u16) {
		self.display.wh()
	}

	/// Update 2D overlay with writer function.
	pub fn draw(&self, writer: &Fn(u16, u16) -> [u8; 4]) {
		self.display.draw(writer)
	}
}
