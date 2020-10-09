#version 320 es
precision mediump float;

in vec3 color;
in vec2 texture_coordinate;

uniform sampler2D uniform_texture;

out vec4 fragment_color;

// TODO: Make this inputs
const float tile_x = 1.0;
const float tile_y = 2.0;

const float piece_size = 253.0;
const float texture_size = 1024.0;

// 253.0 / 1024.0 = 0.2470703125
const float scale = piece_size / texture_size;

void main()
{
    fragment_color = texture(uniform_texture, vec2((texture_coordinate.x + tile_x) * scale, (texture_coordinate.y + tile_y) * scale));
}