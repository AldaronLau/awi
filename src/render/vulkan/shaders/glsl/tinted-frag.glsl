// Copyright Jeron A. Lau 2018.
// Dual-licensed under either the MIT License or the Boost Software License,
// Version 1.0.  (See accompanying file LICENSE_1_0.txt or copy at
// https://www.boost.org/LICENSE_1_0.txt)

#version 450
#extension GL_ARB_separate_shader_objects : enable

layout (binding = 0) uniform UniformBuffer {
	mat4 models_tfm; // The Models' Transform Matrix
	vec4 color;
	int has_camera;
} uniforms;
layout (binding = 1) uniform Camera {
	mat4 matrix; // The Camera's Transform & Projection Matrix
} camera;
layout (binding = 2) uniform Fog {
	vec4 fog; // The fog color.
	vec2 range; // The range of fog (fog to far clip)
} fog;
layout (binding = 3) uniform sampler2D tex;

layout (location = 0) in vec4 texcoord;
layout (location = 1) in float z;
layout (location = 2) in vec4 tint;

layout (location = 0) out vec4 frag_color;

void main() {
	vec4 sampled = texture(tex, texcoord.xy);
	vec4 out_color = vec4(sampled.rgb, sampled.a * texcoord.a) * tint;

	if(uniforms.has_camera == 2) {
		// Fog Calculation
		float linear = clamp((z-fog.range.x) / fog.range.y, 0.0, 1.0);
		float curved = linear * linear * linear;
		frag_color = mix(out_color, fog.fog, curved);
	} else {
		frag_color = out_color;
	}
}
