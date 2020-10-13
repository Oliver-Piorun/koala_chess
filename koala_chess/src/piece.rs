use crate::{shader::Shader, traits::Draw};

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

pub struct Piece {
    pub color: PieceColor,
    pub kind: PieceKind,
    pub board_x: u8,
    pub board_y: u8,
    pub atlas_shader: Shader,
    pub aspect_ratio: f32,
    pub vertex_buffer_object: gl::types::GLuint,
    pub texture: gl::types::GLuint,
    piece_x: u8,
    piece_y: u8,
}

impl Piece {
    pub fn new(
        color: PieceColor,
        kind: PieceKind,
        board_x: u8,
        board_y: u8,
        atlas_shader: Shader,
        aspect_ratio: f32,
        vertex_buffer_object: gl::types::GLuint,
        texture: gl::types::GLuint,
    ) -> Piece {
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
            color,
            kind,
            board_x,
            board_y,
            atlas_shader,
            aspect_ratio,
            vertex_buffer_object,
            texture,
            piece_x,
            piece_y,
        }
    }
}

impl Draw for Piece {
    fn draw(&self) {
        unsafe {
            // Bind pieces VBO
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

            // Bind pieces texture
            gl::BindTexture(gl::TEXTURE_2D, self.texture);
        }

        // Use specific shader
        self.atlas_shader.r#use();
        self.atlas_shader
            .set_float("board_x\0", self.board_x as gl::types::GLfloat);
        self.atlas_shader
            .set_float("board_y\0", self.board_y as gl::types::GLfloat);
        self.atlas_shader
            .set_float("piece_x\0", self.piece_x as gl::types::GLfloat);
        self.atlas_shader
            .set_float("piece_y\0", self.piece_y as gl::types::GLfloat);
        self.atlas_shader
            .set_float("aspect_ratio\0", self.aspect_ratio);

        // Draw elements
        unsafe {
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
        }
    }
}
