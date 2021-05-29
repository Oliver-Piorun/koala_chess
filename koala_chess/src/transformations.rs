use crate::{mat4::Mat4, vec3::Vec3};

pub fn translate(matrix: Mat4, vector: Vec3) -> Mat4 {
    let mut result = Mat4::default();
    result[0] = matrix[0]; // keep 1st column
    result[1] = matrix[1]; // keep 2nd column
    result[2] = matrix[2]; // keep 3rd column
    result[3] = matrix[0] * vector[0] + matrix[1] * vector[1] + matrix[2] * vector[2] + matrix[3]; // calculate 4th column

    /*
    Example (2D):

    Identity matrix:

    (1 0 0 0)
    (0 1 0 0)
    (0 0 1 0)
    (0 0 0 1)

    Vector:

    (5)
    (5)
    (0)

    Results in:

    (1 0 0 5)
    (0 1 0 5)
    (0 0 1 0)
    (0 0 0 1)
    */

    result
}

pub fn rotate_z(matrix: Mat4, angle_in_degrees: gl::types::GLfloat) -> Mat4 {
    let sin = angle_in_degrees.to_radians().sin();
    let cos = angle_in_degrees.to_radians().cos();

    let mut result = Mat4::default();
    result[0] = matrix[0];
    result[1] = matrix[1];
    result[2] = matrix[2]; // keep 3rd column
    result[3] = matrix[3]; // keep 4th column

    result[0][0] = cos;
    result[0][1] = sin;
    result[1][0] = -sin;
    result[1][1] = cos;

    /*
    Example (2D):

    Identity matrix:

    (1 0 0 0)
    (0 1 0 0)
    (0 0 1 0)
    (0 0 0 1)

    Angle (in degrees):

    45°

    Results in:

    (cos(rad(45)) -sin(rad(45)) 0 0)
    (sin(rad(45)) cos(rad(45))  0 0)
    (0            0             1 0)
    (0            0             0 1)
    */

    result
}

pub fn scale(matrix: Mat4, vector: Vec3) -> Mat4 {
    let mut result = Mat4::default();
    result[0] = matrix[0] * vector[0]; // multiply 1st column
    result[1] = matrix[1] * vector[1]; // multiply 2nd column
    result[2] = matrix[2] * vector[2]; // multiply 3rd column
    result[3] = matrix[3]; // keep 4th column

    /*
    Example (2D):

    Identity matrix:

    (1 0 0 0)
    (0 1 0 0)
    (0 0 1 0)
    (0 0 0 1)

    Vector:

    (5)
    (5)
    (1)

    Results in:

    (5 0 0 0)
    (0 5 0 0)
    (0 0 1 0)
    (0 0 0 1)
    */

    result
}
