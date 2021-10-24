#version 330 core
out vec4 FragColor;

uniform vec4 triangleColor;

void main()
{
    FragColor = triangleColor;
}