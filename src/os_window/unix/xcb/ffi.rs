// "awi" crate - Licensed under the MIT LICENSE
//  * Copyright (c) 2017-2018  Jeron A. Lau <jeron.lau@plopgrizzly.com>

use c_void;
use std::ptr::null_mut;

dl_api!(Xcb, "libxcb.so.1",
	fn xcb_send_event(*mut c_void, u8, u32, u32,
		*const XcbClientMessageEvent) -> (),
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

#[repr(C)]
struct XcbClientMessageEvent {
	response_type: u8,
	format: u8,
	sequence: u16,
	window: u32,
	stype: u32,
	data32: [u32; 5],
}

#[repr(C)]
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

#[allow(dead_code)]
#[repr(C)]
enum StateComponent { None }

#[repr(C)]
enum KeyDirection {
	Up,
	Down,
}

#[repr(C)]
enum CompileFlags { NoFlags = 0 }

#[repr(C)]
enum ContextFlags { NoFlags = 0 }

#[repr(C)]
struct XcbInternAtomReply {
	response_type: u8,
	pad0: u8,
	sequence: u16,
	length: u32,
	atom: u32,
}

#[repr(C)]
pub struct XcbScreen {
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

pub struct Dl {
	xcb: Xcb,
	xkb: XkbCommonX11,
}

pub type Connection = (*mut c_void, Dl);

pub unsafe fn load_dl() -> Result<Dl, ::dl_api::Error> {
	Ok(Dl {
		xcb: Xcb::new()?,
		xkb: XkbCommonX11::new()?,
	})
}

unsafe fn intern_atom(connection: &Connection, name: &[u8]) -> u32 {
	(connection.1.xcb.xcb_intern_atom)(connection.0, 0, name.len() as u16,
		&name[0])
}

unsafe fn intern_atom_reply(connection: &Connection, atom: u32) -> u32 {
	extern { fn free(this: *mut XcbInternAtomReply) -> (); }

	let reply = (connection.1.xcb.xcb_intern_atom_reply)(connection.0, atom,
		null_mut());
	let atom = (*reply).atom;

	free(reply);

	atom
}

pub unsafe fn get_atom(connection: &Connection, name: &[u8]) -> u32 {
	intern_atom_reply(connection, intern_atom(connection, name))
}

pub unsafe fn change_property(connection: &Connection, window: u32, t: u32,
	a: u32, data: &[u32])
{
	let len = data.len() as u32;
	let ptr = &data[0];

	(connection.1.xcb.xcb_change_property)(connection.0, 0, window, a, t,
		32, len, ptr as *const _ as *const c_void);
}

pub unsafe fn change_property_title(connection: &Connection, window: u32,
	title: &[u8])
{
	let atom1 = get_atom(connection, b"_NET_WM_NAME");
	let atom2 = get_atom(connection, b"_NET_WM_VISIBLE_NAME");
	let atom3 = get_atom(connection, b"_NET_WM_ICON_NAME");
	let atom4 = get_atom(connection, b"_NET_WM_VISIBLE_ICON_NAME");
	let atom = get_atom(connection, b"UTF8_STRING");
	let len = title.len() as u32;
	let ptr = &title[0];

	(connection.1.xcb.xcb_change_property)(connection.0, 0, window, atom1,
		atom, 8, len, ptr as *const _ as *const c_void);
	(connection.1.xcb.xcb_change_property)(connection.0, 0, window, atom2,
		atom, 8, len, ptr as *const _ as *const c_void);
	(connection.1.xcb.xcb_change_property)(connection.0, 0, window, atom3,
		atom, 8, len, ptr as *const _ as *const c_void);
	(connection.1.xcb.xcb_change_property)(connection.0, 0, window, atom4,
		atom, 8, len, ptr as *const _ as *const c_void);
}

pub unsafe fn send_event(connection: &Connection, window: u32, a: (u32,u32)) {
	let event = XcbClientMessageEvent {
		response_type: 33, // Client Message
		format: 32,
		sequence: 0,
		window: window,
		stype: a.0,
		data32: [2, a.1, 0, 0, 0],
	};

	(connection.1.xcb.xcb_send_event)(connection.0, 1, window,
		1048576 | 524288, &event);
}

pub unsafe fn map_window(connection: &Connection, window: u32) {
	(connection.1.xcb.xcb_map_window)(connection.0, window);
	(connection.1.xcb.xcb_flush)(connection.0);
}

pub unsafe fn screen_root(connection: &Connection) -> (u32, u32, u32) {
	let setup = (connection.1.xcb.xcb_get_setup)(connection.0);
	let screen = (connection.1.xcb.xcb_setup_roots_iterator)(setup).data;

	((*screen).root, (*screen).root_visual, (*screen).black_pixel)
}

pub unsafe fn generate_id(connection: &Connection) -> u32 {
	(connection.1.xcb.xcb_generate_id)(connection.0)
}

pub unsafe fn create_window(connection: &Connection, window: u32,
	rvb: (u32, u32, u32)) -> ()
{
	let (root, visual, black) = rvb;
	let mut value_list = [black, 0b01000100000000001101111];

	(connection.1.xcb.xcb_create_window)(connection.0, 0, window, root, 0,
		0, ::MWW as u16, ::MWH as u16, 10, 1, visual, 2|2048,
		&mut value_list[0]);
}

pub unsafe fn connect(dl: &Dl) -> *mut c_void {
	let connection = (dl.xcb.xcb_connect)(null_mut(), null_mut());

	if connection.is_null() {
		panic!("Couldn't connect to X Server.");
	}

	connection
}

pub unsafe fn destroy_window(connection: &Connection, window: u32) -> () {
	(connection.1.xcb.xcb_destroy_window)(connection.0, window);
}

pub unsafe fn disconnect(connection: &Connection) -> () {
	(connection.1.xcb.xcb_disconnect)(connection.0);
}

pub unsafe fn poll_for_event(connection: &Connection, state: *mut c_void)
	-> Option<(u8, u32, (i16, i16), (i16, i16), Option<String>)>
{
	use super::super::input::key;
	use std::string::String;

	extern { fn free(event: *mut XcbGenericEvent) -> (); }

	let event = (connection.1.xcb.xcb_poll_for_event)(connection.0);

	if event.is_null() {
		return None;
	}

	let response_type = (*event).response_type;
	let detail = (*event).detail as u32;
	let event_xy = ((*event).event_x, (*event).event_y);
	let root_xy = ((*event).root_x, (*event).root_y);

	free(event);

	let string = match response_type {
		2 => {
			Some(match detail {
				key::LEFT => String::from("\u{91}"),
				key::RIGHT => String::from("\u{92}"),
				key::UP => String::from("\u{9E}"),
				key::DOWN => String::from("\u{9F}"),
				key::ENTER | key::ext::NUM_PAD_ENTER
					=> String::from("\n"),
				key::LEFT_SHIFT | key::RIGHT_SHIFT |
					key::ext::ALT_GR | key::ext::NUMLOCK |
					9 =>
				{
					xkb_state_update_key(connection, state,
						detail, true);
					String::from("")
				},
				_ => {
					xkb_state_key_get_utf8(connection,
						state, detail)
				}
			})
		},
		3 => {
			xkb_state_update_key(connection, state, detail, false);
			None
		},
		_ => None
	};

	Some((response_type, detail, event_xy, root_xy, string))
}

pub unsafe fn xkb_get_core_keyboard_device_id(connection: &Connection) -> i32 {
	(connection.1.xkb.xkb_x11_get_core_keyboard_device_id)(connection.0)
}

pub unsafe fn xkb_context_new(connection: &Connection) -> *mut c_void {
	(connection.1.xkb.xkb_context_new)(ContextFlags::NoFlags)
}

pub unsafe fn xkb_x11_keymap_new_from_device(connection: &Connection,
	xkbctx: *mut c_void, device_id: i32) -> *mut c_void
{
	(connection.1.xkb.xkb_x11_keymap_new_from_device)(xkbctx, connection.0,
		device_id, CompileFlags::NoFlags)
}

pub unsafe fn xkb_x11_state_new_from_device(connection: &Connection,
	keymap: *mut c_void, device_id: i32) -> *mut c_void
{
	(connection.1.xkb.xkb_x11_state_new_from_device)(
		keymap, connection.0, device_id)
}

unsafe fn xkb_state_update_key(connection: &Connection, state: *mut c_void,
	keycode: u32, dn: bool)
{
	(connection.1.xkb.xkb_state_update_key)(state, keycode, if dn {
		KeyDirection::Down
	} else {
		KeyDirection::Up
	});
}

unsafe fn xkb_state_key_get_utf8(connection: &Connection, state: *mut c_void,
	key: u32) -> String
{
	let size = (connection.1.xkb.xkb_state_key_get_utf8)(state, key,
		::std::ptr::null_mut(), 0) as usize + 1;
	let mut utf8 = Vec::new();

	utf8.resize(size, b'\0'); // Size + 1 to include NULL byte from XKB.

	let buffer = utf8.as_mut_ptr();

	(connection.1.xkb.xkb_state_key_get_utf8)(state, key, buffer, size);

	utf8.pop();

	// TODO: Validate that is valid
	::std::string::String::from_utf8(utf8).unwrap()
}

pub unsafe fn use_xkb_extension(connection: &Connection) {
	(connection.1.xkb.xcb_xkb_use_extension)(connection.0, 1, 0);
}

pub unsafe fn xkb_state_unref(connection: &Connection, state: *mut c_void) {
	(connection.1.xkb.xkb_state_unref)(state);
}

pub unsafe fn xkb_keymap_unref(connection: &Connection, keymap: *mut c_void) {
	(connection.1.xkb.xkb_keymap_unref)(keymap);
}

pub unsafe fn xkb_context_unref(connection: &Connection, context: *mut c_void) {
	(connection.1.xkb.xkb_context_unref)(context);
}
