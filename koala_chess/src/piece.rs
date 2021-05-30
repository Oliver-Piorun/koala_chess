use crate::{
    bitmap,
    mat4::Mat4,
    shader::Shader,
    transformations::{scale, translate},
    vec3::Vec3,
};
use logger::*;
use std::{error::Error, lazy::SyncLazy, sync::Mutex};

pub enum PieceColor {
    White,
    Black,
}

pub enum PieceKind {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

static ATLAS_SHADER: SyncLazy<Mutex<Option<Shader>>> = SyncLazy::new(|| Mutex::new(None));
static mut VERTEX_BUFFER_OBJECT: gl::types::GLuint = 0;
static mut TEXTURE: gl::types::GLuint = 0;

pub struct Piece {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub rotation: f32,
    pub color: PieceColor,
    pub kind: PieceKind,
    pub board_x: u8,
    pub board_y: u8,
    piece_x: u8,
    piece_y: u8,
}

impl Piece {
    pub fn new(color: PieceColor, kind: PieceKind, board_x: u8, board_y: u8) -> Piece {
        let (piece_x, piece_y) = match (&color, &kind) {
            (PieceColor::White, PieceKind::Pawn) => (0, 2),
            (PieceColor::White, PieceKind::Knight) => (2, 2),
            (PieceColor::White, PieceKind::Bishop) => (0, 1),
            (PieceColor::White, PieceKind::Rook) => (2, 1),
            (PieceColor::White, PieceKind::Queen) => (0, 0),
            (PieceColor::White, PieceKind::King) => (2, 0),
            (PieceColor::Black, PieceKind::Pawn) => (1, 2),
            (PieceColor::Black, PieceKind::Knight) => (3, 2),
            (PieceColor::Black, PieceKind::Bishop) => (1, 1),
            (PieceColor::Black, PieceKind::Rook) => (3, 1),
            (PieceColor::Black, PieceKind::Queen) => (1, 0),
            (PieceColor::Black, PieceKind::King) => (3, 0),
        };

        Piece {
            x: 0.0,
            y: 0.0,
            width: 0.0,
            height: 0.0,
            rotation: 0.0,
            color,
            kind,
            board_x,
            board_y,
            piece_x,
            piece_y,
        }
    }

    pub fn initialize(atlas_shader: Shader) {
        *ATLAS_SHADER
            .lock()
            .unwrap_or_else(|e| fatal!("Could not lock atlas shader mutex! {}", e)) =
            Some(atlas_shader);

        // Load bitmap
        let bitmap = bitmap::load_bitmap("textures/pieces.bmp")
            .unwrap_or_else(|e| fatal!("Could not load pieces bitmap! ({})", e));

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
                gl::LINEAR as gl::types::GLint,
            );
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MAG_FILTER,
                gl::LINEAR as gl::types::GLint,
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
                1024,
                1024,
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
        let atlas_shader_mutex = ATLAS_SHADER
            .lock()
            .unwrap_or_else(|e| fatal!("Could not lock atlas shader mutex! {}", e));
        let atlas_shader = atlas_shader_mutex
            .unwrap_or_else(|| fatal!("Atlas shader has not been initialized yet!"));
        atlas_shader.r#use();

        // Calculate model
        let mut model = Mat4::identity();
        model = translate(model, Vec3::new_xyz(self.x, self.y, 0.0));
        model = scale(model, Vec3::new_xyz(self.width, self.height, 1.0));

        atlas_shader.set_mat4("model\0", model.data.as_ptr() as *const gl::types::GLfloat)?;
        atlas_shader.set_mat4(
            "projection\0",
            projection.data.as_ptr() as *const gl::types::GLfloat,
        )?;
        atlas_shader.set_float("piece_x\0", self.piece_x as gl::types::GLfloat)?;
        atlas_shader.set_float("piece_y\0", self.piece_y as gl::types::GLfloat)?;

        // Draw elements
        unsafe {
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
        }

        Ok(())
    }
}
