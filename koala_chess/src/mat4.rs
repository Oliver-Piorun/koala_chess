use crate::vec4::Vec4;
use std::ops::{Index, IndexMut};

#[derive(Default)]
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
