// Copyright Jeron A. Lau 2018.
// Dual-licensed under either the MIT License or the Boost Software License,
// Version 1.0.  (See accompanying file LICENSE_1_0.txt or copy at
// https://www.boost.org/LICENSE_1_0.txt)

#version 450
#extension GL_ARB_separate_shader_objects : enable

layout (binding = 0) uniform UniformBuffer {
	mat4 models_tfm; // The Models' Transform Matrix
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

layout (location = 0) in vec4 pos;
layout (location = 1) in vec4 texpos;
layout (location = 2) in vec4 color;

layout (location = 0) out vec4 texcoord;
layout (location = 1) out float z;
layout (location = 2) out vec4 tint;

void main() {
	texcoord = texpos;
	tint = color;

	vec4 place = uniforms.models_tfm * vec4(pos.xyz, 1.0);

	if(uniforms.has_camera >= 1) {
		gl_Position = camera.matrix * place;
	} else {
		gl_Position = place;
	}

	z = length(gl_Position.xyz);
}
