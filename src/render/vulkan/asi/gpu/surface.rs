// "asi_vulkan" - Aldaron's System Interface - Vulkan
//
// Copyright Jeron A. Lau 2018.
// Distributed under the Boost Software License, Version 1.0.
// (See accompanying file LICENSE_1_0.txt or copy at
// https://www.boost.org/LICENSE_1_0.txt)

// TODO: Make surface a buffer and blit onto screen with window manager.

use c_void;
use std::{ mem, ptr::{ null_mut } };

use super::super::gpu;
use super::super::types::*;
use WindowConnection;
use super::VulkanApi;

pub(crate) fn new(vk: VkInstance, lib: &VulkanApi, connection: WindowConnection)
	-> VkSurfaceKHR
{
	match connection {
		WindowConnection::Xcb(connection,window) => {
			if cfg!(unix) {
				new_xcb(vk, lib, connection, window)
			} else { unreachable!() }
		}
		WindowConnection::Wayland => {
			println!("Wayland not supported yet");
			unreachable!()
		},
		WindowConnection::DirectFB => {
			println!("DirectFB not supported yet");
			unreachable!()
		},
		WindowConnection::Windows(connection, window) => {
			if cfg!(windows) {
				new_windows(vk, lib, connection, window)
			} else { unreachable!() }
		}
		WindowConnection::Android => { // TODO
/*			if cfg!(android) {
				new_android(vk, lib, ___)
			} else {*/ unreachable!() /*}*/
		},
		WindowConnection::IOS => {
			println!("iOS not supported yet");
			unreachable!()
		},
		WindowConnection::AldaronsOS => {
			println!("Aldaron's OS not supported yet");
			unreachable!()
		},
		WindowConnection::Arduino => {
			println!("Arduino not supported yet");
			unreachable!()
		},
		WindowConnection::Switch => {
			println!("Nintendo Switch not supported yet");
			unreachable!()
		},
		WindowConnection::Web => {
			println!("Wasm not supported yet");
			unreachable!()
		},
		WindowConnection::NoOS => {
			println!("No OS not supported yet");
			unreachable!()
		},
	}
}

#[repr(C)] struct SurfaceCreateInfoXcb {
	s_type: VkStructureType,
	p_next: *mut c_void,
	flags: u32,
	connection: *mut c_void,
	window: u32,
}

#[repr(C)] struct SurfaceCreateInfoWindows {
	s_type: VkStructureType,
	p_next: *mut c_void,
	flags: u32,
	// TODO
	hinstance: *mut c_void,
	hwnd: *mut c_void,
}

#[repr(C)] struct SurfaceCreateInfoAndroid {
	s_type: VkStructureType,
	p_next: *mut c_void,
	flags: u32,
	window: *mut c_void, // ANativeWindow,
}

fn new_xcb(vk: VkInstance, lib: &VulkanApi, wc: *mut c_void, w: u32)
	-> VkSurfaceKHR
{
	let mut surface = unsafe { mem::uninitialized() };
	let surface_create_info = SurfaceCreateInfoXcb {
		s_type: VkStructureType::SurfaceCreateInfoXcb,
		p_next: null_mut(),
		flags: 0,
		connection: wc,
		window: w,
	};

	let new_surface : unsafe extern "system" fn(
		instance: VkInstance,
		pCreateInfo: *const SurfaceCreateInfoXcb,
		pAllocator: *mut c_void,
		surface: *mut VkSurfaceKHR) -> VkResult
		= unsafe
	{
		gpu::vk_sym(vk, lib, b"vkCreateXcbSurfaceKHR\0").unwrap()
	};

	unsafe {
		(new_surface)(vk, &surface_create_info, null_mut(),
			&mut surface)
		.unwrap();
	};

	surface
}

fn new_windows(vk: VkInstance, lib: &VulkanApi, wc: *mut c_void, w: *mut c_void)
	-> VkSurfaceKHR
{
	let mut surface = unsafe { mem::uninitialized() };
	let surface_create_info = SurfaceCreateInfoWindows {
		s_type: VkStructureType::SurfaceCreateInfoWindows,
		p_next: null_mut(),
		flags: 0,
		hinstance: wc,
		hwnd: w,
	};
	
	let new_surface: unsafe extern "system" fn(
		instance: VkInstance,
		pCreateInfo: *const SurfaceCreateInfoWindows,
		pAllocator: *mut c_void,
		surface: *mut VkSurfaceKHR) -> VkResult
		= unsafe
	{
		gpu::vk_sym(vk, lib, b"vkCreateWin32SurfaceKHR\0").unwrap()
	};

	unsafe {
		(new_surface)(vk, &surface_create_info, null_mut(),
			&mut surface)
		.unwrap();
	};

	surface
}

#[allow(unused)] // TODO: make used
fn new_android(vk: VkInstance, lib: &VulkanApi, w: *mut c_void) -> VkSurfaceKHR
{
	let mut surface = unsafe { mem::uninitialized() };
	let surface_create_info = SurfaceCreateInfoAndroid {
		s_type: VkStructureType::SurfaceCreateInfoAndroid,
		p_next: null_mut(),
		flags: 0,
		window: w,
	};

	let new_surface: unsafe extern "system" fn(
		instance: VkInstance,
		pCreateInfo: *const SurfaceCreateInfoAndroid,
		pAllocator: *mut c_void,
		surface: *mut VkSurfaceKHR) -> VkResult
		= unsafe
	{
		gpu::vk_sym(vk, lib, b"vkCreateAndroidSurfaceKHR\0").unwrap()
	};

	unsafe {
		(new_surface)(vk, &surface_create_info, null_mut(),
			&mut surface)
		.unwrap();
	};

	surface
}
