// "awi" - Aldaron's Window Interface
//
// Copyright Jeron A. Lau 2017-2018.
// Distributed under the Boost Software License, Version 1.0.
// (See accompanying file LICENSE_1_0.txt or copy at
// https://www.boost.org/LICENSE_1_0.txt)

use input::keyboard;
use c_void;
use std::ptr::null_mut;

pub struct Window {
	// Keyboard (XKB)
	keymap: *mut c_void,
	context: *mut c_void,
	state: *mut c_void,
	xkb: XkbCommonX11,
	// Window (XCB)
	window: u32,
	connection: *mut c_void,
	wh: (u16, u16),
	xcb: Xcb,
}

impl Window {
	pub fn new(_title: &str, _icon: (u16, u16, Vec<u32>), v: Option<i32>) -> Self{
		// TODO: Try Wayland first

		let (xcb, xkb) = xcb_load();
		let connection = xcb_connect(&xcb);
		let mut screen = xcb_screen(connection, &xcb);
		let window = xcb_window(connection, &xcb, &mut screen, v);
		let (state, keymap, context) = xkb_keyboard(connection, &xkb);
		let wh = (screen.width_in_pixels, screen.height_in_pixels);

		Window {
			state, keymap, context, xkb, window, connection, wh,
			xcb
		}
	}

	pub fn poll_event(&mut self, input: &mut ::input::InputQueue,
		keyboard: &mut ::input::keyboard::Keyboard)
		-> bool
	{
		xcb_poll_for_event(self.connection, &self.xcb, &self.xkb,
			self.state, input, &mut self.wh,
			keyboard)
	}

	pub fn get_connection(&self) -> ::WindowConnection {
		::WindowConnection::Xcb(self.connection, self.window)
	}

	pub fn wh(&self) -> (u16, u16) {
		self.wh
	}
}

impl Drop for Window {
	fn drop(&mut self) {
		unsafe {
			(self.xkb.xkb_state_unref)(self.state);
			(self.xkb.xkb_keymap_unref)(self.keymap);
			(self.xkb.xkb_context_unref)(self.context);
			(self.xcb.xcb_destroy_window)(self.connection,
				self.window);
			(self.xcb.xcb_disconnect)(self.connection);
		}
	}
}

dl_api!(Xcb, "libxcb.so.1",
	fn xcb_poll_for_event(*mut c_void) -> *mut XcbGenericEvent,
	fn xcb_flush(*mut c_void) -> i32,
	fn xcb_intern_atom(*mut c_void, u8, u16, *const u8) -> u32,
	fn xcb_intern_atom_reply(*mut c_void, u32, *mut c_void)
		-> *mut XcbInternAtomReply,
	fn xcb_change_property(*mut c_void, u8, u32, u32, u32, u8, u32,
		*const c_void) -> u32,
	fn xcb_map_window(*mut c_void, u32) -> u32,
	fn xcb_get_setup(*mut c_void) -> *mut c_void,
	fn xcb_setup_roots_iterator(*mut c_void) -> XcbScreenIterator,
	fn xcb_generate_id(*mut c_void) -> u32,
	fn xcb_create_window(*mut c_void, u8, u32, u32, i16, i16, u16, u16, u16,
		u16, u32, u32, *mut u32) -> u32,
	fn xcb_connect(*mut c_void, *mut c_void) -> *mut c_void,
	fn xcb_destroy_window(*mut c_void, u32) -> u32,
	fn xcb_disconnect(*mut c_void) -> ()
);

dl_api!(XkbCommonX11, "libxkbcommon-x11.so.0",
	fn xkb_context_unref(*mut c_void) -> (),
	fn xkb_keymap_unref(*mut c_void) -> (),
	fn xkb_state_unref(*mut c_void) -> (),
	fn xcb_xkb_use_extension(*mut c_void, u16, u16) -> u32,
	fn xkb_state_key_get_utf8(*mut c_void, u32, *mut u8, usize) -> i32,
	fn xkb_state_update_key(*mut c_void, u32, KeyDirection)
		-> StateComponent,
	fn xkb_x11_state_new_from_device(*mut c_void, *mut c_void, i32)
		-> *mut c_void,
	fn xkb_x11_keymap_new_from_device(*mut c_void, *mut c_void, i32,
		CompileFlags) -> *mut c_void,
	fn xkb_context_new(ContextFlags) -> *mut c_void,
	fn xkb_x11_get_core_keyboard_device_id(*mut c_void) -> i32
);

#[allow(dead_code)]
#[repr(C)]
enum StateComponent { None }

#[repr(C)]
enum KeyDirection {
	Up,
	Down,
}

#[repr(C)]
struct XcbInternAtomReply {
	response_type: u8,
	pad0: u8,
	sequence: u16,
	length: u32,
	atom: u32,
}

#[repr(C)]
enum CompileFlags { NoFlags = 0 }

#[repr(C)]
enum ContextFlags { NoFlags = 0 }

#[repr(C)] #[derive(Clone)]
struct XcbScreen {
	root: u32,
	default_colormap: u32,
	white_pixel: u32,
	black_pixel: u32,
	current_input_masks: u32,
	width_in_pixels: u16,
	height_in_pixels: u16,
	width_in_millimeters: u16,
	height_in_millimeters: u16,
	min_installed_maps: u16,
	max_installed_maps: u16,
	root_visual: u32,
	backing_stores: u8,
	save_unders: u8,
	root_depth: u8,
	allowed_depths_len: u8,
}

#[repr(C)]
struct XcbScreenIterator {
	data: *mut XcbScreen,
	rem: i32,
	index: i32,
}

#[repr(C)] #[derive(Clone)]
struct XcbGenericEvent {
	response_type: u8,
	detail: u8,
	sequence: u16,
	timestamp: u32,
	root: u32,
	event: u32,
	child: u32,
	root_x: i16,
	root_y: i16,
	event_x: i16,
	event_y: i16,
	state: u16,
	same_screen: u8,
	pad0: u8,
}

fn xcb_load() -> (Xcb, XkbCommonX11) {
	unsafe fn load_xcb_dl() -> Result<(Xcb,XkbCommonX11), ::dl_api::Error> {
		Ok((Xcb::new()?, XkbCommonX11::new()?))
	}
	unsafe { load_xcb_dl() }.unwrap_or_else(|err| {
		eprintln!("ERROR: couldn't find XCB: \"{}\", aborting...", err);
		::std::process::abort();
	})
}

fn xcb_connect(xcb: &Xcb) -> *mut c_void {
	let connection = unsafe { (xcb.xcb_connect)(null_mut(), null_mut()) };
	if connection.is_null() {
		eprintln!(
			"ERROR: XCB couldn't connect to X server, aborting..."
		);
		::std::process::abort();
	}
	connection
}

fn xcb_screen(connection: *mut c_void, xcb: &Xcb) -> XcbScreen {
	let setup = unsafe { (xcb.xcb_get_setup)(connection) };
	unsafe { (*((xcb.xcb_setup_roots_iterator)(setup).data)).clone() }
}

fn xcb_window(connection: *mut c_void, xcb: &Xcb, screen: &mut XcbScreen,
	v: Option<i32>) -> u32
{
	let atom1 = get_atom(connection, xcb, b"_NET_WM_STATE");
	let atom = get_atom(connection, xcb, b"_NET_WM_STATE_FULLSCREEN");
	let window = unsafe { (xcb.xcb_generate_id)(connection) };
	let mut value_list = [screen.black_pixel, 0b01000100000000001101111];
	if let Some(v) = v {
		screen.root_visual = unsafe { ::std::mem::transmute(v) };
	}
	unsafe {
		(xcb.xcb_create_window)(
			connection, 0, window, screen.root, 0, 0,
			screen.width_in_pixels, screen.height_in_pixels, 10, 1,
			screen.root_visual, 2 | 2048, &mut value_list[0]
		);
		(xcb.xcb_change_property)(connection, 0, window, atom1,
			4, 32, 1, &atom as *const _ as *const c_void);
		(xcb.xcb_map_window)(connection, window);
		(xcb.xcb_flush)(connection);
	}
	window
}

fn get_atom(connection: *mut c_void, xcb: &Xcb, name: &[u8]) -> u32 {
	let atom = unsafe {
		(xcb.xcb_intern_atom)(
			connection, 0, name.len() as u16, &name[0]
		)
	};
	let reply = unsafe {
		(xcb.xcb_intern_atom_reply)(connection, atom, null_mut())
	};
	let atom = unsafe {
		extern { fn free(this: *mut XcbInternAtomReply) -> (); }
		let r_atom = (*reply).atom;
		free(reply);
		r_atom
	};
	atom
}

fn xkb_keyboard(connection: *mut c_void, xkb: &XkbCommonX11)
	-> (*mut c_void, *mut c_void, *mut c_void)
{
	unsafe {
		(xkb.xcb_xkb_use_extension)(connection, 1, 0);
	}
	let device_id = unsafe {
		(xkb.xkb_x11_get_core_keyboard_device_id)(connection)
	};
	let context = unsafe {
		(xkb.xkb_context_new)(ContextFlags::NoFlags)
	};
	let keymap = unsafe {
		(xkb.xkb_x11_keymap_new_from_device)(
			context, connection, device_id,
			CompileFlags::NoFlags
		)
	};
	let state = unsafe {
		(xkb.xkb_x11_state_new_from_device)(
			keymap, connection, device_id
		)
	};

	(state, keymap, context)
}

fn xcb_poll_for_event(connection: *mut c_void, xcb: &Xcb,
	xkb: &XkbCommonX11, state: *mut c_void, queue: &mut ::input::InputQueue,
	wh: &mut (u16, u16), keyboard: &mut ::Keyboard) -> bool
{
	use std::string::String;

	extern { fn free(event: *mut XcbGenericEvent) -> (); }

	let event = unsafe { (xcb.xcb_poll_for_event)(connection) };
	let event = if event.is_null() {
		return false;
	} else {
		unsafe {
			let r_event = (*event).clone();
			free(event);
			r_event
		}
	};

	let response_type = event.response_type;
	let detail = event.detail;
	let event_xy = (event.event_x, event.event_y);
	let root_xy = (event.root_x as u16, event.root_y as u16); // i16 -> u16

	let string = match response_type {
		2 => {
			Some(match detail {
				// Enter: Keyboard & NumPad
				36 | 104 => String::from("\n"),
				// Left & Right Shift, Alt Gr & NumLock & Esc
				50 | 62 | 108 | 77 | 9 => {
					xkb_state_update_key(xkb, state, detail,
						true);
					String::from("")
				},
				// Everything else
				_ => xkb_state_key_get_utf8(xkb,state,detail)
			})
		},
		3 => {
			xkb_state_update_key(xkb, state, detail, false);
			None
		},
		_ => None
	};

	const KEY_DOWN: u8 = 2;
	const KEY_UP: u8 = 3;
	const BUTTON_DOWN: u8 = 4;
	const BUTTON_UP: u8 = 5;
	const CURSOR_MOVE: u8 = 6;
	const CURSOR_LEAVE: u8 = 8;
	const GAIN_FOCUS: u8 = 9;
	const LOSE_FOCUS: u8 = 10;
	const WINDOW_RESIZE: u8 = 22;
	const WINDOW_SELECT: u8 = 31;
	const WINDOW_CLOSE: u8 = 128 | 33;

	match response_type {
		KEY_DOWN => if let Some(key) = key(detail) {
			keyboard.press(key);
		} else if detail == 9 {
			queue.back();
		},
		KEY_UP => if let Some(key) = key(detail) {
			keyboard.release(key);
		},
		BUTTON_DOWN => match detail {
			1 => queue.left_button_press(*wh, event_xy),
			2 => queue.middle_button_press(*wh, event_xy),
			3 => queue.right_button_press(*wh, event_xy),
			4 => queue.scroll(*wh, event_xy, (0.0, -1.0)),
			5 => queue.scroll(*wh, event_xy, (0.0, 1.0)),
			6 => queue.scroll(*wh, event_xy, (-1.0, 0.0)),
			7 => queue.scroll(*wh, event_xy, (1.0, 0.0)),
			uc => panic!("awi: Unknown Click {}!", uc)
		},
		BUTTON_UP => match detail {
			1 => queue.left_button_release(*wh, event_xy),
			2 => queue.middle_button_release(*wh, event_xy),
			3 => queue.right_button_release(*wh, event_xy),
			_ => {},
		},
		CURSOR_MOVE => queue.cursor_move(*wh, event_xy),
		CURSOR_LEAVE => queue.cursor_leave(),
		GAIN_FOCUS => queue.resume(),
		LOSE_FOCUS => queue.pause(),
		WINDOW_RESIZE => queue.resize(wh, root_xy),
		WINDOW_SELECT => println!("!SELECT!"),
		WINDOW_CLOSE => queue.back(),
		_ => { }, // ignore all other messages
	}

	if let Some(string) = string {
		queue.text(string);
	}

	true
}

fn xkb_state_update_key(xkb: &XkbCommonX11, state: *mut c_void,
	keycode: u8, dn: bool)
{
	unsafe {
		(xkb.xkb_state_update_key)(state, keycode as u32, if dn {
			KeyDirection::Down
		} else {
			KeyDirection::Up
		});
	}
}

fn xkb_state_key_get_utf8(xkb: &XkbCommonX11, state: *mut c_void,
	key: u8) -> String
{
	let size = unsafe {
		(xkb.xkb_state_key_get_utf8)(state, key as u32,
			::std::ptr::null_mut(), 0) as usize + 1
	};
	let mut utf8 = Vec::new();

	utf8.resize(size, b'\0'); // Size + 1 to include NULL byte from XKB.

	let buffer = utf8.as_mut_ptr();

	unsafe {
		(xkb.xkb_state_key_get_utf8)(state, key as u32, buffer, size);
	}

	utf8.pop();

	// TODO: Validate that is valid
	::std::string::String::from_utf8(utf8).unwrap()
}

// Keycode translator
fn key(physical_key: u8) -> Option<u8> {
	Some(match physical_key {
		49 => keyboard::EXT_BACKTICK,
		86 => keyboard::EXT_PLUS,
		63 => keyboard::EXT_ASTERISK,
		61 | 106 => keyboard::SLASH,
		36 | 104 => keyboard::ENTER,
		10 | 87 => keyboard::NUM1,
		11 | 88 => keyboard::NUM2,
		12 | 89 => keyboard::NUM3,
		13 | 83 => keyboard::NUM4,
		14 | 84 => keyboard::NUM5,
		15 | 85 => keyboard::NUM6,
		16 | 79 => keyboard::NUM7,
		17 | 80 => keyboard::NUM8,
		18 | 81 => keyboard::NUM9,
		19 | 90 => keyboard::NUM0,
		60 | 91 => keyboard::PERIOD,
		20 | 82 => keyboard::MINUS,
		21 => keyboard::EQUAL_SIGN,
		22 => keyboard::BACKSPACE,
		23 => keyboard::TAB,
		38 => keyboard::A,
		56 => keyboard::B,
		54 => keyboard::C,
		40 => keyboard::D,
		26 => keyboard::E,
		41 => keyboard::F,
		42 => keyboard::G,
		43 => keyboard::H,
		31 => keyboard::I,
		44 => keyboard::J,
		45 => keyboard::K,
		46 => keyboard::L,
		58 => keyboard::M,
		57 => keyboard::N,
		32 => keyboard::O,
		33 => keyboard::P,
		24 => keyboard::Q,
		27 => keyboard::R,
		39 => keyboard::S,
		28 => keyboard::T,
		30 => keyboard::U,
		55 => keyboard::V,
		25 => keyboard::W,
		53 => keyboard::X,
		29 => keyboard::Y,
		52 => keyboard::Z,
		34 => keyboard::BRACKET_OPEN,
		35 => keyboard::BRACKET_CLOSE,
		37 => keyboard::LCTRL,
		105 => keyboard::RCTRL,
		50 => keyboard::LSHIFT,
		62 => keyboard::RSHIFT,
		64 => keyboard::ALT,
		108 => keyboard::EXT_ALT_GR,
		66 => keyboard::COMPOSE,
		47 => keyboard::SEMICOLON,
		48 => keyboard::APOSTROPHE,
		51 => keyboard::BACKSLASH,
		59 => keyboard::COMMA,
		65 => keyboard::SPACE,
		77 => keyboard::EXT_NUM_LOCK,
		110 => keyboard::EXT_HOME,
		115 => keyboard::EXT_END,
		112 => keyboard::EXT_PAGE_UP,
		117 => keyboard::EXT_PAGE_DOWN,
		118 => keyboard::EXT_INSERT,
		119 => keyboard::EXT_DELETE,
		111 => keyboard::UP,
		113 => keyboard::LEFT,
		114 => keyboard::RIGHT,
		116 => keyboard::DOWN,
		_ => return None,
	} )
}
