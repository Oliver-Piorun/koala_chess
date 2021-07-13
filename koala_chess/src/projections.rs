use crate::mat4::Mat4;

// Right-handed, -1 to 1
pub fn orthogonal_projection(
    left: gl::types::GLfloat,
    right: gl::types::GLfloat,
    bottom: gl::types::GLfloat,
    top: gl::types::GLfloat,
    near: gl::types::GLfloat,
    far: gl::types::GLfloat,
) -> Mat4 {
    let mut projection = Mat4::default();
    projection[0][0] = 2.0 / (right - left);
    projection[1][1] = 2.0 / (top - bottom);
    projection[2][2] = -2.0 / (far - near);
    projection[3][0] = -(right + left) / (right - left);
    projection[3][1] = -(top + bottom) / (top - bottom);
    projection[3][2] = -(far + near) / (far - near);

    projection[3][3] = 1.0;

    projection
}
