#version 330 core
out vec4 FragColor;

in vec3 out_color;

//uniform vec4 triangleColor;

void main()
{
    FragColor = vec4(out_color, 1.0);
}