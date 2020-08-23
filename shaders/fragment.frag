#version 330 core
in vec3 color;
in vec2 texture_coordinates;

out vec4 fragment_color;

uniform sampler2D uniform_texture;

void main()
{
    fragment_color = texture(uniform_texture, texture_coordinates);
}