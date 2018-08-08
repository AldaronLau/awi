#[macro_use]
extern crate stdweb;
#[macro_use]
extern crate stdweb_derive;

mod webgl_rendering_context;

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
use webgl_rendering_context::{
	WebGLRenderingContext as gl,
};

macro_rules! enclose {
	( ($( $x:ident ),*) $y:expr ) => {
		{
			$(let $x = $x.clone();)*
			$y
		}
	};
}

fn main() {
	let canvas: CanvasElement = document().query_selector("#canvas")
		.unwrap().unwrap().try_into().unwrap();
	let context: gl = canvas.get_context().unwrap();

	canvas.set_width(canvas.offset_width() as u32);
	canvas.set_height(canvas.offset_height() as u32);

	context.clear_color(1.0, 0.0, 0.0, 1.0);
	context.clear(gl::COLOR_BUFFER_BIT);

	window().add_event_listener( enclose!( (canvas) move |_: ResizeEvent| {
		canvas.set_width(canvas.offset_width() as u32);
		canvas.set_height(canvas.offset_height() as u32);
	}));

	window().request_animation_frame(move |_time| {
//		state.borrow_mut().context.animate(time, state.clone());
	});
}
