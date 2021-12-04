use crate::{
    bitmap,
    board::Board,
    input::Input,
    mat4::Mat4,
    shader::Shader,
    transformations::{rotate_z, scale, translate},
    vec3::Vec3,
};
use logger::*;
use std::{error::Error, lazy::SyncLazy, sync::Mutex};

#[derive(PartialEq)]
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
    pub const TEXTURE_ATLAS_SIZE: i32 = 1024;
    pub const TEXTURE_SIZE: i32 = 253;

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
                Piece::TEXTURE_ATLAS_SIZE,
                Piece::TEXTURE_ATLAS_SIZE,
                0,
                gl::BGRA_EXT,
                gl::UNSIGNED_BYTE,
                bitmap.data.as_ptr() as *const std::ffi::c_void,
            );
        }
    }

    pub fn draw(
        &self,
        input: &Input,
        projection: &Mat4,
        board: &Board,
    ) -> Result<(), Box<dyn Error>> {
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

        if board.rotation != 0.0 {
            // Rotate around board center (clock-wise)
            let board_center_x = board.x + board.width / 2.0;
            let board_center_y = board.y + board.height / 2.0;

            let x_translation = board_center_x - self.x;
            let y_translation = board_center_y - self.y;

            model = translate(model, Vec3::new_xyz(x_translation, y_translation, 0.0));
            model = rotate_z(model, board.rotation);
            model = translate(model, Vec3::new_xyz(-x_translation, -y_translation, 0.0));

            // Rotate around piece center
            let piece_center_x = self.x + self.width / 2.0;
            let piece_center_y = self.y + self.height / 2.0;

            let x_translation = piece_center_x - self.x;
            let y_translation = piece_center_y - self.y;

            model = translate(model, Vec3::new_xyz(x_translation, y_translation, 0.0));

            // Reset piece rotation, so that the piece is always facing upwards
            model = rotate_z(model, 0.0);

            model = translate(model, Vec3::new_xyz(-x_translation, -y_translation, 0.0));
        }

        model = scale(model, Vec3::new_xyz(self.width, self.height, 1.0));

        let result = *projection * model;

        let mut mouse_x = input.mouse_x as f32;
        let mut mouse_y = input.mouse_y as f32;
        let piece_x = result[0][0] + result[3][0];
        let piece_y = result[1][1] + result[3][1];
        if self.board_x == 0 && self.board_y == 0 {
            //println!("p: {} {}", self.x, self.y);
            println!("p2: {} {}", piece_x, piece_y);
            println!("m1: {} {}", mouse_x, mouse_y);
        }

        // Check if mouse is on piece

        // println!("m1: {} {}", mouse_x, mouse_y);

        if mouse_x >= piece_x
            && mouse_x <= piece_x + self.width
            && mouse_y >= piece_y
            && mouse_y <= piece_y + self.height
        {
            println!("yes");
        }

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
