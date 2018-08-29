// Copyright Jeron A. Lau 2017-2018.
// Dual-licensed under either the MIT License or the Boost Software License,
// Version 1.0.  (See accompanying file LICENSE_1_0.txt or copy at
// https://www.boost.org/LICENSE_1_0.txt)

use render::Vec3;

/// Convert 3 u8 to 4 f32 for color.
fn to_f32_rgba(rgb: [u8; 3]) -> [f32; 4] {
	[
		(rgb[0] as f32) / 255.0, (rgb[1] as f32) / 255.0,
		(rgb[2] as f32) / 255.0, 1.0,
	]
}
