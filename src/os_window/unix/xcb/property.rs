// "awi" crate - Licensed under the MIT LICENSE
//  * Copyright (c) 2017-2018  Jeron A. Lau <jeron.lau@plopgrizzly.com>

use super::ffi as xcb;

pub struct Property(u32, u32);

impl Property {
	pub fn create(connection: &xcb::Connection, name: &[u8], name2: &[u8])
		-> Property
	{
		let atom1 = unsafe { xcb::get_atom(connection, name) };
		let atom2 = unsafe { xcb::get_atom(connection, name2) };

		Property(atom1, atom2)
	}

	pub fn catch(&self, connection: &xcb::Connection, window: u32) -> () {
		let data = [self.1];

		unsafe {
			xcb::change_property(connection,window,4,self.0,&data)
		}
	}

	pub fn apply(&self, connection: &xcb::Connection, window: u32) -> () {
		unsafe {
			xcb::send_event(connection, window, (self.0, self.1))
		}
	}
}
