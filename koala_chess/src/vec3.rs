use std::ops::{Add, Index, IndexMut, Mul};

#[derive(Default, Clone, Copy)]
pub struct Vec3 {
    // data
    data: [gl::types::GLfloat; 3],
}

impl Index<usize> for Vec3 {
    type Output = gl::types::GLfloat;

    fn index(&self, index: usize) -> &Self::Output {
        // Return a specific element
        &self.data[index]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        // Return a specific element
        &mut self.data[index]
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        let mut result = self;
        result[0] += rhs[0];
        result[1] += rhs[1];
        result[2] += rhs[2];

        result
    }
}

impl Mul<gl::types::GLfloat> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: gl::types::GLfloat) -> Self::Output {
        let mut result = self;
        result[0] *= rhs;
        result[1] *= rhs;
        result[2] *= rhs;

        result
    }
}

impl Vec3 {
    pub fn new(value: gl::types::GLfloat) -> Vec3 {
        let mut result = Vec3::default();
        result[0] = value;
        result[1] = value;
        result[2] = value;

        result
    }
}
