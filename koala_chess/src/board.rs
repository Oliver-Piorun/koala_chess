use crate::{
    bitmap,
    mat4::Mat4,
    shader::Shader,
    transformations::{rotate_z, scale, translate},
    vec3::Vec3,
};
use logger::*;
use std::{error::Error, lazy::SyncLazy, sync::Mutex};

static SHADER: SyncLazy<Mutex<Option<Shader>>> = SyncLazy::new(|| Mutex::new(None));
static mut VERTEX_BUFFER_OBJECT: gl::types::GLuint = 0;
static mut TEXTURE: gl::types::GLuint = 0;

pub struct Board {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub rotation: f32,
}

impl Board {
    pub const TEXTURE_SIZE: i32 = 2048;
    pub const BORDER_TEXTURE_SIZE: i32 = 12;

    pub fn initialize(shader: Shader) {
        *SHADER
            .lock()
            .unwrap_or_else(|e| fatal!("Could not lock shader mutex! ({})", e)) = Some(shader);

        // Load bitmap
        let bitmap = bitmap::load_bitmap("textures/board.bmp")
            .unwrap_or_else(|e| fatal!("Could not load board bitmap! ({})", e));

        #[rustfmt::skip]
        let vertices: [f32; 16] = [
            // positions, texture coordinates
            0.0, 0.0,     0.0, 0.0, // top left
            1.0, 0.0,     1.0, 0.0, // top right
            1.0, 1.0,     1.0, 1.0, // bottom right
            0.0, 1.0,     0.0, 1.0, // bottom left
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
                Board::TEXTURE_SIZE,
                Board::TEXTURE_SIZE,
                0,
                gl::BGRA_EXT,
                gl::UNSIGNED_BYTE,
                bitmap.data.as_ptr() as *const std::ffi::c_void,
            );
        }
    }

    pub fn draw(&self, projection: &Mat4) -> Result<(), Box<dyn Error>> {
        unsafe {
            // Bind vertex buffer object
            gl::BindBuffer(gl::ARRAY_BUFFER, VERTEX_BUFFER_OBJECT);

            // Position attribute
            gl::VertexAttribPointer(
                0,
                2,
                gl::FLOAT,
                gl::FALSE,
                16,
                std::ptr::null::<std::ffi::c_void>(),
            );
            gl::EnableVertexAttribArray(0);

            // Texture coordinates attribute
            gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, 16, 8 as *const std::ffi::c_void);
            gl::EnableVertexAttribArray(1);

            // Bind texture
            gl::BindTexture(gl::TEXTURE_2D, TEXTURE);
        }

        // Use specific shader
        let shader_mutex = SHADER
            .lock()
            .unwrap_or_else(|e| fatal!("Could not lock shader mutex! ({})", e));
        let shader = shader_mutex.unwrap_or_else(|| fatal!("Shader has not been initialized yet!"));
        shader.r#use();

        // Calculate model
        let mut model = Mat4::identity();
        model = translate(model, Vec3::new_xyz(self.x, self.y, 0.0));

        if self.rotation != 0.0 {
            let x_translation = self.width / 2.0;
            let y_translation = self.height / 2.0;

            model = translate(model, Vec3::new_xyz(x_translation, y_translation, 0.0));
            model = rotate_z(model, self.rotation);
            model = translate(model, Vec3::new_xyz(-x_translation, -y_translation, 0.0));
        }

        model = scale(model, Vec3::new_xyz(self.width, self.height, 1.0));

        shader.set_mat4("model\0", model.data.as_ptr() as *const gl::types::GLfloat)?;
        shader.set_mat4(
            "projection\0",
            projection.data.as_ptr() as *const gl::types::GLfloat,
        )?;

        // Draw elements
        unsafe {
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
        }

        Ok(())
    }
}
