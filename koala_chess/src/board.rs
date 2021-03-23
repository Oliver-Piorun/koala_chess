use crate::bitmap;
use crate::shader::Shader;
use crate::traits::Draw;
use logger::*;
use std::sync::Mutex;
use std::{error::Error, lazy::SyncLazy};

static SHADER: SyncLazy<Mutex<Option<Shader>>> = SyncLazy::new(|| Mutex::new(None));
static mut VERTEX_BUFFER_OBJECT: gl::types::GLuint = 0;
static mut TEXTURE: gl::types::GLuint = 0;

pub struct Board;

impl Board {
    pub fn initialize(shader: Shader) {
        *SHADER
            .lock()
            .unwrap_or_else(|e| fatal!("Could not lock shader mutex! ({})", e)) = Some(shader);

        // Load bitmap
        let bitmap = bitmap::load_bitmap("textures/board.bmp")
            .unwrap_or_else(|e| fatal!("Could not load board bitmap! ({})", e));

        #[rustfmt::skip]
        let vertices: [f32; 32] = [
            // positions,    colors,        texture coordinates
             0.8,  0.8, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, // top right
             0.8, -0.8, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0, // bottom right
            -0.8, -0.8, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, // bottom left
            -0.8,  0.8, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, // top left
        ];

        unsafe {
            // Generate vertex buffer object
            gl::GenBuffers(1, &mut VERTEX_BUFFER_OBJECT);

            // Bind vertex buffer object
            gl::BindBuffer(gl::ARRAY_BUFFER, VERTEX_BUFFER_OBJECT);

            // Set vertex buffer object data
            gl::BufferData(
                gl::ARRAY_BUFFER,
                std::mem::size_of_val(&vertices) as gl::types::GLsizeiptr,
                vertices.as_ptr() as *const std::ffi::c_void,
                gl::STATIC_DRAW,
            );

            gl::Enable(gl::TEXTURE_2D);
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);

            // Generate texture
            gl::GenTextures(1, &mut TEXTURE);

            // Bind texture
            gl::BindTexture(gl::TEXTURE_2D, TEXTURE);

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
    }
}

impl Draw for Board {
    fn draw(&self, aspect_ratio: f32) -> Result<(), Box<dyn Error>> {
        unsafe {
            // Bind vertex buffer object
            gl::BindBuffer(gl::ARRAY_BUFFER, VERTEX_BUFFER_OBJECT);

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
            gl::BindTexture(gl::TEXTURE_2D, TEXTURE);
        }

        // Use specific shader
        let shader_mutex = SHADER
            .lock()
            .unwrap_or_else(|e| fatal!("Could not lock shader mutex! ({})", e));
        let shader = shader_mutex.unwrap_or_else(|| fatal!("Shader has not been initialized yet!"));
        shader.r#use();
        shader.set_float("aspect_ratio\0", aspect_ratio)?;

        // Draw elements
        unsafe {
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
        }

        Ok(())
    }
}
