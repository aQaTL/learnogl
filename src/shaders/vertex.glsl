#version 430 core
layout (location = 0) in vec3 pos;
layout (location = 1) in vec2 tex_coords;

out vec2 o_vertex_tex_coords;

uniform mat4 transform;

void main() {
	o_vertex_tex_coords = tex_coords;
	gl_Position = transform * vec4(pos, 1.0);
}
