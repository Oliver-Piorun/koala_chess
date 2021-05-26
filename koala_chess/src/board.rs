use crate::shader::Shader;
use crate::{
    bitmap,
    mat4::Mat4,
    transformations::{rotate_z, translate},
    vec3::Vec3,
};
use crate::{traits::Draw, transformations::scale};
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
        let vertices: [f32; 16] = [
            // positions, texture coordinates
            1.0, 0.0,     1.0, 0.0, // top right
            1.0, 1.0,     1.0, 1.0, // bottom right
            0.0, 1.0,     0.0, 1.0, // bottom left
            0.0, 0.0,     0.0, 0.0, // top left
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

        let mut right = 800.0;
        let mut bottom = 800.0;

        if aspect_ratio >= 1.0 {
            right *= aspect_ratio;
        } else {
            bottom /= aspect_ratio;
        }

        let projection = orthogonal_projection(0.0, right, bottom, 0.0, -1.0, 1.0);

        let board_size = 620.0;

        // Calculate centering translation
        let mut translation = Vec3::default();
        translation[0] = right / 2.0 - board_size / 2.0;
        translation[1] = bottom / 2.0 - board_size / 2.0;

        let mut model = Mat4::identity();
        model = translate(model, translation);
        model = rotate_z(model, 0.0);
        model = scale(model, Vec3::new(board_size));

        shader.set_mat4("model\0", model.data.as_ptr() as *const gl::types::GLfloat)?;
        shader.set_mat4(
            "projection\0",
            projection.as_ptr() as *const gl::types::GLfloat,
        )?;

        // Draw elements
        unsafe {
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
        }

        Ok(())
    }
}

// Right-handed, -1 to 1
fn orthogonal_projection(
    left: gl::types::GLfloat,
    right: gl::types::GLfloat,
    bottom: gl::types::GLfloat,
    top: gl::types::GLfloat,
    near: gl::types::GLfloat,
    far: gl::types::GLfloat,
) -> [[gl::types::GLfloat; 4]; 4] {
    // right handed, -1 to 1
    let mut projection: [[gl::types::GLfloat; 4]; 4] = [[0.0; 4]; 4];
    projection[0][0] = 2.0 / (right - left);
    projection[1][1] = 2.0 / (top - bottom);
    projection[2][2] = -2.0 / (far - near);
    projection[3][0] = -(right + left) / (right - left);
    projection[3][1] = -(top + bottom) / (top - bottom);
    projection[3][2] = -(far + near) / (far - near);

    projection[3][3] = 1.0;

    projection
}

// Left-handed, 0 to 1
fn _orthogonal_projection_lh_zo(
    left: gl::types::GLfloat,
    right: gl::types::GLfloat,
    bottom: gl::types::GLfloat,
    top: gl::types::GLfloat,
    near: gl::types::GLfloat,
    far: gl::types::GLfloat,
) -> *const gl::types::GLfloat {
    // left handed, 0 to 1
    let mut projection: [[gl::types::GLfloat; 4]; 4] = [[0.0; 4]; 4];
    projection[0][0] = 2.0 / (right - left);
    projection[1][1] = 2.0 / (top - bottom);
    projection[2][2] = 1.0 / (far - near);
    projection[3][0] = -(right + left) / (right - left);
    projection[3][1] = -(top + bottom) / (top - bottom);
    projection[3][2] = -near / (far - near);

    projection[3][3] = 1.0;

    projection.as_ptr() as *const gl::types::GLfloat
}
