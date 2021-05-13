use std::ops::{Add, Index, IndexMut, Mul};

#[derive(Default, Clone, Copy)]
pub struct Vec4 {
    // data
    data: [gl::types::GLfloat; 4],
}

impl Index<usize> for Vec4 {
    type Output = gl::types::GLfloat;

    fn index(&self, index: usize) -> &Self::Output {
        // Return a specific element
        &self.data[index]
    }
}

impl IndexMut<usize> for Vec4 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        // Return a specific element
        &mut self.data[index]
    }
}

impl Add<Vec4> for Vec4 {
    type Output = Vec4;

    fn add(self, rhs: Vec4) -> Self::Output {
        let mut result = self.clone();
        result[0] = result[0] + rhs[0];
        result[1] = result[1] + rhs[1];
        result[2] = result[2] + rhs[2];
        result[3] = result[3] + rhs[3];

        result
    }
}

impl Mul<gl::types::GLfloat> for Vec4 {
    type Output = Vec4;

    fn mul(self, rhs: gl::types::GLfloat) -> Self::Output {
        let mut result = self.clone();
        result[0] = result[0] * rhs;
        result[1] = result[1] * rhs;
        result[2] = result[2] * rhs;
        result[3] = result[3] * rhs;

        result
    }
}

impl Vec4 {
    pub fn new(value: gl::types::GLfloat) -> Vec4 {
        let mut result = Vec4::default();
        result[0] = value;
        result[1] = value;
        result[2] = value;
        result[3] = value;

        result
    }
}
