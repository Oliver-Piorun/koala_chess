#version 300 es
#define M_PI 3.1415926535897932384626433832795
precision mediump float;

layout (location = 0) in vec2 in_position;
layout (location = 1) in vec2 in_texture_coordinate;

uniform float board_x;
uniform float board_y;
uniform float rotation;
uniform float aspect_ratio;

out vec3 color;
out vec2 texture_coordinate;

const float texture_size = 1024.0;
const float border_size = 6.0;
const float margin = 0.2;
const float tile_count = 8.0;
const float tile_offset = -7.0;

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
    // Map each board coordinate from [0,7] to [-7,7]
    // (0,0) => (-7,-7)
    // (1,0) => (-5,-7)
    // ...
    // (7,0) => ( 7, 0)
    float corrected_board_x = board_x * 2.0 + tile_offset;
    float corrected_board_y = board_y * 2.0 + tile_offset;

    // Scaling
    float scaled_x = (in_position.x + corrected_board_x) * scale;
    float scaled_y = (in_position.y + corrected_board_y) * scale;

    // Rotation
    float rotated_x = scaled_x;
    float rotated_y = scaled_y;

    if (rotation > 0.0)
    {
        float radian = rotation * (M_PI / 180.0);

        rotated_x = cos(radian) * scaled_x - sin(radian) * scaled_y;
        rotated_y = sin(radian) * scaled_x + cos(radian) * scaled_y;
    }

    // Aspect ratio
    if (aspect_ratio >= 1.0)
    {
        rotated_x /= aspect_ratio;
    }
    else
    {
        rotated_y *= aspect_ratio;
    }

    gl_Position = vec4(rotated_x, rotated_y, 0.0, 1.0);
    texture_coordinate = in_texture_coordinate;
}