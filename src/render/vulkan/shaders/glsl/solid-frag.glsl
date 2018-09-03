#version 450
#extension GL_ARB_separate_shader_objects : enable

layout (binding = 0) uniform UniformBuffer {
	mat4 transform; // The Transformation Matrix
	vec4 color;
} uniforms;

// layout (location = 0) in vec4 in_color;

layout (location = 0) out vec4 frag_color;

void main() {
	frag_color = uniforms.color;
}
