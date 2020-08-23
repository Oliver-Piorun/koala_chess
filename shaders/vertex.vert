#version 330 core
layout (location = 0) in vec3 in_position;
layout (location = 1) in vec3 in_color;
layout (location = 2) in vec2 in_texture_coordinates;

out vec3 color;
out vec2 texture_coordinates;

void main()
{
    gl_Position = vec4(in_position, 1.0);
    color = in_color;
    texture_coordinates = in_texture_coordinates;
}