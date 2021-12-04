use crate::vec4::Vec4;
use std::ops::{Index, IndexMut, Mul};

#[derive(Default, Clone, Copy)]
pub struct Mat4 {
    // data[column][row]
    pub data: [Vec4; 4],
}

impl Index<usize> for Mat4 {
    type Output = Vec4;

    fn index(&self, index: usize) -> &Self::Output {
        // Return a specific column
        &self.data[index]
    }
}

impl IndexMut<usize> for Mat4 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        // Return a specific column
        &mut self.data[index]
    }
}

impl Mul<Mat4> for Mat4 {
    type Output = Mat4;

    #[rustfmt::skip]
    fn mul(self, rhs: Mat4) -> Self::Output {
        let mut result = self.clone();
        result[0][0] = self[0][0] * rhs[0][0] + self[0][1] * rhs[1][0] + self[0][2] * rhs[2][0] + self[0][3] * rhs[3][0];
        result[0][1] = self[0][0] * rhs[0][1] + self[0][1] * rhs[1][1] + self[0][2] * rhs[2][1] + self[0][3] * rhs[3][1];
        result[0][2] = self[0][0] * rhs[0][2] + self[0][1] * rhs[1][2] + self[0][2] * rhs[2][2] + self[0][3] * rhs[3][2];
        result[0][3] = self[0][0] * rhs[0][3] + self[0][1] * rhs[1][3] + self[0][2] * rhs[2][3] + self[0][3] * rhs[3][3];
        result[1][0] = self[1][0] * rhs[0][0] + self[1][1] * rhs[1][0] + self[1][2] * rhs[2][0] + self[1][3] * rhs[3][0];
        result[1][1] = self[1][0] * rhs[0][1] + self[1][1] * rhs[1][1] + self[1][2] * rhs[2][1] + self[1][3] * rhs[3][1];
        result[1][2] = self[1][0] * rhs[0][2] + self[1][1] * rhs[1][2] + self[1][2] * rhs[2][2] + self[1][3] * rhs[3][2];
        result[1][3] = self[1][0] * rhs[0][3] + self[1][1] * rhs[1][3] + self[1][2] * rhs[2][3] + self[1][3] * rhs[3][3];
        result[2][0] = self[2][0] * rhs[0][0] + self[2][1] * rhs[1][0] + self[2][2] * rhs[2][0] + self[2][3] * rhs[3][0];
        result[2][1] = self[2][0] * rhs[0][1] + self[2][1] * rhs[1][1] + self[2][2] * rhs[2][1] + self[2][3] * rhs[3][1];
        result[2][2] = self[2][0] * rhs[0][2] + self[2][1] * rhs[1][2] + self[2][2] * rhs[2][2] + self[2][3] * rhs[3][2];
        result[2][3] = self[2][0] * rhs[0][3] + self[2][1] * rhs[1][3] + self[2][2] * rhs[2][3] + self[2][3] * rhs[3][3];
        result[3][0] = self[3][0] * rhs[0][0] + self[3][1] * rhs[1][0] + self[3][2] * rhs[2][0] + self[3][3] * rhs[3][0];
        result[3][1] = self[3][0] * rhs[0][1] + self[3][1] * rhs[1][1] + self[3][2] * rhs[2][1] + self[3][3] * rhs[3][1];
        result[3][2] = self[3][0] * rhs[0][2] + self[3][1] * rhs[1][2] + self[3][2] * rhs[2][2] + self[3][3] * rhs[3][2];
        result[3][3] = self[3][0] * rhs[0][3] + self[3][1] * rhs[1][3] + self[3][2] * rhs[2][3] + self[3][3] * rhs[3][3];

        result
    }
}

impl Mat4 {
    pub fn identity() -> Mat4 {
        let mut result = Mat4::default();
        result[0][0] = 1.0;
        result[1][1] = 1.0;
        result[2][2] = 1.0;
        result[3][3] = 1.0;

        result
    }
}
