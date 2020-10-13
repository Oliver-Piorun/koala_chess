use crate::shader::Shader;
use crate::traits::Draw;

pub struct Board {
    pub shader: Shader,
    pub aspect_ratio: f32,
    pub vertex_buffer_object: gl::types::GLuint,
    pub texture: gl::types::GLuint,
}

impl Draw for Board {
    fn draw(&self) {
        unsafe {
            // Bind board VBO
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vertex_buffer_object);

            // Position attribute
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                32,
                std::ptr::null::<std::ffi::c_void>(),
            );
            gl::EnableVertexAttribArray(0);

            // Color attribute
            gl::VertexAttribPointer(
                1,
                3,
                gl::FLOAT,
                gl::FALSE,
                32,
                12 as *const std::ffi::c_void,
            );
            gl::EnableVertexAttribArray(1);

            // Texture coordinates attribute
            gl::VertexAttribPointer(
                2,
                2,
                gl::FLOAT,
                gl::FALSE,
                32,
                24 as *const std::ffi::c_void,
            );
            gl::EnableVertexAttribArray(2);

            // Bind board texture
            gl::BindTexture(gl::TEXTURE_2D, self.texture);
        }

        // Use specific shader
        self.shader.r#use();
        self.shader.set_float("aspect_ratio\0", self.aspect_ratio);

        // Draw elements
        unsafe {
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
        }
    }
}
