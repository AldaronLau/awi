#version 450
#extension GL_ARB_separate_shader_objects : enable

layout (binding = 0) uniform UniformBuffer {
	mat4 transform; // The Transformation Matrix
	vec4 color;
} uniforms;
layout (binding = 1) uniform sampler2D tex;

layout (location = 0) in vec4 texcoord;
// layout (location = 1) in vec4 tint;

layout (location = 0) out vec4 frag_color;

void main() {
	vec4 sampled = texture(tex, texcoord.xy);
	frag_color = sampled * uniforms.color;
}
