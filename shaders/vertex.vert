#version 320 es
precision mediump float;

layout (location = 0) in vec3 in_position;
layout (location = 1) in vec3 in_color;
layout (location = 2) in vec2 in_texture_coordinate;

out vec3 color;
out vec2 texture_coordinate;

void main()
{
    gl_Position = vec4(in_position, 1.0);
    color = in_color;
    texture_coordinate = in_texture_coordinate;
}