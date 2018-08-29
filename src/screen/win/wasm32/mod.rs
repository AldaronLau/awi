// Copyright Jeron A. Lau 2017-2018.
// Dual-licensed under either the MIT License or the Boost Software License,
// Version 1.0.  (See accompanying file LICENSE_1_0.txt or copy at
// https://www.boost.org/LICENSE_1_0.txt)

include!("../common.rs");

mod webgl;

use screen::Transform;
use input::Input;
use afi::VFrame;

use stdweb::unstable::TryInto;
use stdweb::web::{
	IEventTarget,
	IHtmlElement,
	IParentNode,
	document,
	window,
};

use stdweb::web::event::{
	ResizeEvent,
};

use stdweb::web::html_element::CanvasElement;
use self::webgl::{
	WebGLRenderingContext as Gl,
};

use std::sync::{Mutex, Barrier};

macro_rules! enclose {
	( ($( $x:ident ),*) $y:expr ) => {
		{
			$(let $x = $x.clone();)*
			$y
		}
	};
}

static mut WIDTH: u16 = 0;
static mut HEIGHT: u16 = 0;

fn resize(canvas: &CanvasElement) {
	canvas.set_width(canvas.offset_width() as u32);
	canvas.set_height(canvas.offset_height() as u32);
	unsafe {
		WIDTH = canvas.offset_width() as u16;
		HEIGHT = canvas.offset_height() as u16;
	}
}

pub(crate) struct Display {
	canvas: CanvasElement,
	context: Gl,
}

pub struct Model;
pub struct Texture;
pub struct TexCoords;
pub struct Shape;
pub struct Gradient;

impl Display {
	pub(crate) fn new() -> Self {
		// Connect to the Javascript Canvas.
		let canvas: CanvasElement = document()
			.query_selector("#canvas")
			.unwrap()
			.unwrap()
			.try_into()
			.unwrap();
		let context: Gl = canvas.get_context().unwrap();

		// Default background color.
		context.clear_color(0.0, 0.5, 0.0, 1.0);

		// Set Canvas Size.
		resize(&canvas);

		// Get Resize Events.
		window().add_event_listener( enclose!( (canvas) move |_: ResizeEvent| {
			resize(&canvas);
		}));

		// Return Display.
		Display {
			context, canvas,
		}
	}

	pub(crate) fn clear(&mut self, color: (u8, u8, u8)) {
		let [r, g, b, a] = to_f32_rgba([color.0, color.1, color.2]);

		self.context.clear_color(r, g, b, a);
	}

	pub(crate) fn fog(&mut self, fog: Option<(f32, f32)>) {
		// TODO
	}

	pub(crate) fn input(&mut self) -> Option<Input> {
		// TODO: No Input
		None
	}

	pub(crate) fn update(&mut self) -> f32 {
		// Clear Screen
		self.context.clear(Gl::COLOR_BUFFER_BIT);

		// Go through draw commands

		// Update
		let time = Mutex::new(0.0);
		let barrier = Barrier::new(2);
		window().request_animation_frame(move |t| {
			*(time.lock().unwrap()) = t;
			barrier.wait();
		});
		barrier.wait();
		*time.lock().unwrap() as f32
	}

	pub(crate) fn camera(&mut self, position: Vec3, rotation: Vec3) {
		// TODO
	}

	pub fn model(&mut self, vertices: &[f32], fans: Vec<(u32, u32)>)
		-> Model
	{
		// TODO
		Model
	}

	pub fn texture(&mut self, wh: (u16, u16), graphic: &VFrame) -> Texture {
		// TODO
		Texture
	}

	pub fn gradient(&mut self, colors: &[f32]) -> Gradient {
		// TODO
		Gradient
	}

	pub fn texcoords(&mut self, texcoords: &[f32]) -> TexCoords {
		// TODO
		TexCoords
	}

	pub fn set_texture(&mut self, texture: &mut Texture, wh: (u16, u16),
		graphic: &VFrame)
	{
		// TODO
	}

	pub fn shape_solid(&mut self, model: &Model, transform: Transform,
		color: [f32; 4], blending: bool, fog: bool, camera: bool)
		-> Shape	
	{
		// TODO
		Shape
	}

	pub fn shape_gradient(&mut self, model: &Model, transform: Transform,
		gradient: Gradient, blending: bool, fog: bool, camera: bool)
		-> Shape
	{
		// TODO
		Shape
	}

	pub fn shape_texture(&mut self, model: &Model, transform: Transform,
		texture: &Texture, tc: TexCoords, blending: bool, fog: bool, 
		camera: bool) -> Shape
	{
		// TODO
		Shape
	}

	pub fn shape_faded(&mut self, model: &Model, transform: Transform,
		texture: &Texture, tc: TexCoords, alpha: f32, fog: bool,
		camera: bool) -> Shape
	{
		// TODO
		Shape
	}

	pub fn shape_tinted(&mut self, model: &Model, transform: Transform, 
		texture: &Texture, tc: TexCoords, tint: [f32; 4],
		blending: bool, fog: bool, camera: bool) -> Shape
	{
		// TODO
		Shape
	}

	pub fn shape_complex(&mut self, model: &Model, transform: Transform, 
		texture: &Texture, tc: TexCoords, gradient: Gradient,
		blending: bool, fog: bool, camera: bool) -> Shape
	{
		// TODO
		Shape
	}

	pub fn drop_shape(&mut self, shape: &Shape) {
		// TODO
	}

	pub fn transform(&mut self, shape: &Shape, transform: Transform) {
		// TODO
	}

	pub fn resize(&mut self, wh: (u16, u16)) {
		// TODO
	}

	pub fn wh(&self) -> (u16, u16) {
		unsafe { (WIDTH, HEIGHT) }
	}
}
