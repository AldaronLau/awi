#version 450
#extension GL_ARB_separate_shader_objects : enable

layout (binding = 0) uniform UniformBuffer {
	mat4 transform; // The Transformation Matrix
} uniforms;

layout (location = 0) in vec4 pos;
layout (location = 1) in vec4 color;

layout (location = 0) out vec4 fragcolor;

void main() {
	fragcolor = color;
	gl_Position = uniforms.transform * vec4(pos.xyz, 1.0);
}
