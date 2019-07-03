#version 430 core
out vec4 o_color;

in vec2 o_vertex_tex_coords;

struct Material {
	vec3 ambient;
	vec3 diffuse;
	vec3 specular;
	float shininess;
};

uniform sampler2D tex;

void main() {
	o_color = texture(tex, o_vertex_tex_coords);
}
