#version 300 es
#define M_PI 3.1415926535897932384626433832795
precision mediump float;

layout (location = 0) in vec3 in_position;
layout (location = 1) in vec3 in_color;
layout (location = 2) in vec2 in_texture_coordinate;

uniform float rotation;
uniform float aspect_ratio;

out vec3 color;
out vec2 texture_coordinate;

void main()
{
    float initial_x = in_position.x;
    float initial_y = in_position.y;

    // Rotation
    float rotated_x = initial_x;
    float rotated_y = initial_y;

    if (rotation > 0.0)
    {
        float radian = rotation * (M_PI / 180.0);

        rotated_x = cos(radian) * initial_x - sin(radian) * initial_y;
        rotated_y = sin(radian) * initial_x + cos(radian) * initial_y;
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

    gl_Position = vec4(vec3(rotated_x, rotated_y, in_position.z), 1.0);
    color = in_color;
    texture_coordinate = in_texture_coordinate;
}