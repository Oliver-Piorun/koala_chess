#version 320 es
precision mediump float;

layout (location = 0) in vec3 in_position;
layout (location = 1) in vec3 in_color;
layout (location = 2) in vec2 in_texture_coordinate;

uniform float aspect_ratio;

out vec3 color;
out vec2 texture_coordinate;

void main()
{
    float corrected_x = in_position.x;
    float corrected_y = in_position.y;

    if (aspect_ratio >= 1.0)
    {
        corrected_x = in_position.x / aspect_ratio;
    }
    else
    {
        corrected_y = in_position.y * aspect_ratio;
    }

    gl_Position = vec4(vec3(corrected_x, corrected_y, in_position.z), 1.0);
    color = in_color;
    texture_coordinate = in_texture_coordinate;
}