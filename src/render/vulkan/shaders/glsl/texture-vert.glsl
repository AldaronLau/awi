#version 450
#extension GL_ARB_separate_shader_objects : enable

layout (binding = 0) uniform UniformBuffer {
	mat4 transform; // The Transformation Matrix
} uniforms;
layout (binding = 1) uniform sampler2D tex;

layout (location = 0) in vec4 pos;
layout (location = 1) in vec4 texpos;

layout (location = 0) out vec4 texcoord;

void main() {
	texcoord = texpos;
	// TODO: replace vec4(pos.xyz, 1.0) with pos for all shaders.
	gl_Position = uniforms.transform * vec4(pos.xyz, 1.0);
}
