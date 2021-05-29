#version 300 es
precision mediump float;

in vec2 texture_coordinate;

uniform sampler2D uniform_texture;

out vec4 fragment_color;

void main()
{
    fragment_color = texture(uniform_texture, texture_coordinate);
}