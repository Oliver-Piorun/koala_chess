use crate::bitmap;
use crate::shader::Shader;
use crate::traits::Draw;

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

static mut VERTEX_BUFFER_OBJECT: gl::types::GLuint = 0;
static mut TEXTURE: gl::types::GLuint = 0;

pub struct Piece {
    pub color: PieceColor,
    pub kind: PieceKind,
    pub board_x: u8,
    pub board_y: u8,
    pub atlas_shader: Shader,
    pub aspect_ratio: f32,
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
            piece_x,
            piece_y,
        }
    }

    pub fn initialize() {
        // Load bitmap
        let bitmap = bitmap::load_bitmap("textures/pieces.bmp");

        #[rustfmt::skip]
        let vertices: [f32; 32] = [
            // positions,    colors,        texture coordinates
             1.0,  1.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, // top right
             1.0, -1.0, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0, // bottom right
            -1.0, -1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, // bottom left
            -1.0,  1.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, // top left
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
}

impl Draw for Piece {
    fn draw(&self) {
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
