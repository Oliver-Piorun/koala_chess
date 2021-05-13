#version 300 es
precision mediump float;

layout (location = 0) in vec2 in_position;
layout (location = 1) in vec2 in_texture_coordinate;

uniform mat4 model;
uniform mat4 projection;

out vec2 texture_coordinate;

void main()
{
    gl_Position = projection * model * vec4(in_position.xy, 0.0, 1.0);
    texture_coordinate = in_texture_coordinate;
}