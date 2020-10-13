use crate::traits::Draw;
use crate::{bitmap::Bitmap, shader::Shader};

pub struct Board {
    pub shader: Shader,
    pub aspect_ratio: f32,
    pub vertex_buffer_object: gl::types::GLuint,
    pub texture: gl::types::GLuint,
}

impl Board {
    pub fn initialize(bitmap: &Bitmap) -> (gl::types::GLuint, gl::types::GLuint) {
        #[rustfmt::skip]
        let board_vertices: [f32; 32] = [
            // positions,    colors,        texture coordinates
             0.8,  0.8, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, // top right
             0.8, -0.8, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0, // bottom right
            -0.8, -0.8, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, // bottom left
            -0.8,  0.8, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, // top left
        ];

        let mut vertex_buffer_object: gl::types::GLuint = 0;
        let mut texture: gl::types::GLuint = 0;

        unsafe {
            // Generate vertex buffer object
            gl::GenBuffers(1, &mut vertex_buffer_object);

            // Bind vertex buffer object
            gl::BindBuffer(gl::ARRAY_BUFFER, vertex_buffer_object);

            // Set vertex buffer object data
            gl::BufferData(
                gl::ARRAY_BUFFER,
                std::mem::size_of_val(&board_vertices) as gl::types::GLsizeiptr,
                board_vertices.as_ptr() as *const std::ffi::c_void,
                gl::STATIC_DRAW,
            );

            gl::Enable(gl::TEXTURE_2D);
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);

            // Generate texture
            gl::GenTextures(1, &mut texture);

            // Bind texture
            gl::BindTexture(gl::TEXTURE_2D, texture);

            // Parameterize texture
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MIN_FILTER,
                gl::NEAREST as gl::types::GLint,
            );
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MAG_FILTER,
                gl::NEAREST as gl::types::GLint,
            );
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_WRAP_S,
                gl::CLAMP_TO_EDGE as gl::types::GLint,
            );
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_WRAP_T,
                gl::CLAMP_TO_EDGE as gl::types::GLint,
            );

            // Setup texture
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA8 as gl::types::GLint,
                2048,
                2048,
                0,
                gl::BGRA_EXT,
                gl::UNSIGNED_BYTE,
                bitmap.data.as_ptr() as *const std::ffi::c_void,
            );
        }

        (vertex_buffer_object, texture)
    }
}

impl Draw for Board {
    fn draw(&self) {
        unsafe {
            // Bind VBO
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

            // Bind texture
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
