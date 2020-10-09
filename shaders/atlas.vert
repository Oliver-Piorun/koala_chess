#version 320 es
precision mediump float;

layout (location = 0) in vec3 in_position;
layout (location = 1) in vec3 in_color;
layout (location = 2) in vec2 in_texture_coordinate;

uniform float aspect_ratio;

out vec3 color;
out vec2 texture_coordinate;

// TODO: Make this inputs
const float tile_x = 0.0 * 2.0 + 1.0;
const float tile_y = 0.0 * 2.0 + 1.0;

const float texture_size = 1024.0;
const float border_size = 6.0;
const float margin = 0.2;
const float tile_count = 8.0;

// (1024.0 - 2.0 * 6.0) * (1.0 - 0.2) / 8.0
// (1024.0 - 12.0) * 0.8 / 8.0
// 1012.0 * 0.8 / 8.0
// 101.2
const float tile_size = (texture_size - 2.0 * border_size) * (1.0 - margin) / tile_count;

// 101.2 / 1024.0
// 0.098828125
const float scale = tile_size / texture_size;

void main()
{
    float corrected_x = (in_position.x + tile_x) * scale;
    float corrected_y = (in_position.y + tile_y) * scale;

    if (aspect_ratio >= 1.0)
    {
        corrected_x /= aspect_ratio;
    }
    else
    {
        corrected_y *= aspect_ratio;
    }
   
    gl_Position = vec4(vec3(corrected_x, corrected_y, in_position.z), 1.0);
    color = in_color;
    texture_coordinate = in_texture_coordinate;
}