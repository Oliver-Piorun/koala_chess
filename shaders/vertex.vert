#version 300 es
precision mediump float;

layout (location = 0) in vec2 in_position;
layout (location = 1) in vec2 in_texture_coordinate;

uniform mat4 model;
uniform mat4 projection;

out vec2 texture_coordinate;

void main()
{
    // Vclip = Mprojection * Mview * Mmodel * Vlocal
    // Mview is not specified which means that the camera is positioned at the origin and looking torwards -Z
    // Vlocal is not specified which means that the object is positioned at its origin
    gl_Position = projection * model * vec4(
        in_position.x,
        // OpenGL expects 0.0 to be at the bottom, but 0.0 is at the top for the texture
        1.0 - in_position.y,
        0.0,
        1.0);
    texture_coordinate = in_texture_coordinate;
}