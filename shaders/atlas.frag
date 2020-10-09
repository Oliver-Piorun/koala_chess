#version 320 es
precision mediump float;

in vec3 color;
in vec2 texture_coordinate;

uniform sampler2D uniform_texture;
uniform float piece_x;
uniform float piece_y;

out vec4 fragment_color;

const float piece_size = 253.0;
const float texture_size = 1024.0;

// 253.0 / 1024.0 = 0.2470703125
const float scale = piece_size / texture_size;

void main()
{
    fragment_color = texture(uniform_texture, vec2((texture_coordinate.x + piece_x) * scale, (texture_coordinate.y + piece_y) * scale));
}