use crate::{
    board::Board,
    piece::{Piece, PieceColor, PieceKind},
    projections::orthogonal_projection,
    shader,
};
use logger::*;
use std::error::Error;

pub struct Game {
    pub aspect_ratio: f32,
    pub world_width: f32,
    pub world_height: f32,
    pub board: Board,
    pub pieces: Vec<Piece>,
}

impl Game {
    pub fn new() -> Game {
        let board = Board {
            x: 0.0,
            y: 0.0,
            width: 620.0,
            height: 620.0,
            rotation: 0.0,
            pov: PieceColor::Black,
        };

        let mut pieces: Vec<Piece> = Vec::new();

        // Create pieces
        for board_x in 0..8 {
            #[rustfmt::skip]
            let white_pawn = Piece::new(PieceColor::White, PieceKind::Pawn, board_x, 1);
            #[rustfmt::skip]
            let black_pawn = Piece::new(PieceColor::Black, PieceKind::Pawn, board_x, 6);

            pieces.push(white_pawn);
            pieces.push(black_pawn);
        }

        #[rustfmt::skip]
        let left_white_rook = Piece::new(PieceColor::White, PieceKind::Rook, 0, 0);
        #[rustfmt::skip]
        let right_white_rook = Piece::new(PieceColor::White, PieceKind::Rook, 7, 0);
        #[rustfmt::skip]
        let left_black_rook = Piece::new(PieceColor::Black, PieceKind::Rook, 0, 7);
        #[rustfmt::skip]
        let right_black_rook = Piece::new(PieceColor::Black, PieceKind::Rook, 7, 7);

        #[rustfmt::skip]
        let left_white_knight = Piece::new(PieceColor::White, PieceKind::Knight, 1, 0);
        #[rustfmt::skip]
        let right_white_knight = Piece::new(PieceColor::White, PieceKind::Knight, 6, 0);
        #[rustfmt::skip]
        let left_black_knight = Piece::new(PieceColor::Black, PieceKind::Knight, 1, 7);
        #[rustfmt::skip]
        let right_black_knight = Piece::new(PieceColor::Black, PieceKind::Knight, 6, 7);

        #[rustfmt::skip]
        let left_white_bishop = Piece::new(PieceColor::White, PieceKind::Bishop, 2, 0);
        #[rustfmt::skip]
        let right_white_bishop = Piece::new(PieceColor::White, PieceKind::Bishop, 5, 0);
        #[rustfmt::skip]
        let left_black_bishop = Piece::new(PieceColor::Black, PieceKind::Bishop, 2, 7);
        #[rustfmt::skip]
        let right_black_bishop = Piece::new(PieceColor::Black, PieceKind::Bishop, 5, 7);

        #[rustfmt::skip]
        let white_queen = Piece::new(PieceColor::White, PieceKind::Queen, 3, 0);
        #[rustfmt::skip]
        let black_queen = Piece::new(PieceColor::Black, PieceKind::Queen, 3, 7);

        #[rustfmt::skip]
        let white_king = Piece::new(PieceColor::White, PieceKind::King, 4, 0);
        #[rustfmt::skip]
        let black_king = Piece::new(PieceColor::Black, PieceKind::King, 4, 7);

        pieces.push(left_white_rook);
        pieces.push(right_white_rook);
        pieces.push(left_black_rook);
        pieces.push(right_black_rook);
        pieces.push(left_white_knight);
        pieces.push(right_white_knight);
        pieces.push(left_black_knight);
        pieces.push(right_black_knight);
        pieces.push(left_white_bishop);
        pieces.push(right_white_bishop);
        pieces.push(left_black_bishop);
        pieces.push(right_black_bishop);
        pieces.push(white_queen);
        pieces.push(black_queen);
        pieces.push(white_king);
        pieces.push(black_king);

        Game {
            aspect_ratio: 0.0,
            world_width: 800.0,
            world_height: 800.0,
            board,
            pieces,
        }
    }

    pub fn initialize() {
        // Create shaders
        let shader = shader::Shader::new("shaders/vertex.vert", "shaders/fragment.frag")
            .unwrap_or_else(|e| fatal!("{}", e));
        let atlas_shader = shader::Shader::new("shaders/atlas.vert", "shaders/atlas.frag")
            .unwrap_or_else(|e| fatal!("{}", e));

        let mut vertex_array_object: gl::types::GLuint = 0;
        let mut element_buffer_object: gl::types::GLuint = 0;

        let indices: [u32; 6] = [
            0, 1, 3, // first triangle
            1, 2, 3, // second triangle
        ];

        unsafe {
            // Generate vertex array object
            gl::GenVertexArrays(1, &mut vertex_array_object);

            // Bind vertex array object
            gl::BindVertexArray(vertex_array_object);

            // Generate element buffer object
            gl::GenBuffers(1, &mut element_buffer_object);

            // Bind element buffer object
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, element_buffer_object);

            // Set element buffer object data
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                std::mem::size_of_val(&indices) as gl::types::GLsizeiptr,
                indices.as_ptr() as *const std::ffi::c_void,
                gl::STATIC_DRAW,
            );
        }

        Board::initialize(shader);
        Piece::initialize(atlas_shader);

        unsafe {
            // Generate mipmap
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }
    }

    pub fn draw(&mut self, aspect_ratio: f32) -> Result<(), Box<dyn Error>> {
        unsafe {
            // Set the clear color (#1f9b86)
            gl::ClearColor(
                0x1f as gl::types::GLfloat / 255.0,
                0x9b as gl::types::GLfloat / 255.0,
                0x86 as gl::types::GLfloat / 255.0,
                0.0,
            );

            // Clear the viewport with the clear color
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        self.aspect_ratio = aspect_ratio;
        self.world_width = 800.0;
        self.world_height = 800.0;

        if self.aspect_ratio >= 1.0 {
            self.world_width *= self.aspect_ratio;
        } else {
            self.world_height /= self.aspect_ratio;
        }

        // Calculate projection
        let projection =
            orthogonal_projection(0.0, self.world_width, self.world_height, 0.0, -1.0, 1.0);

        // Center board
        self.board.x = self.world_width / 2.0 - self.board.width / 2.0;
        self.board.y = self.world_height / 2.0 - self.board.height / 2.0;

        if self.board.pov == PieceColor::White {
            // Reset board rotation
            self.board.rotation = 0.0;
        } else {
            // Rotate the board by 180 degrees (clock-wise)
            self.board.rotation = 180.0;
        }

        // Draw board
        self.board.draw(&projection)?;

        // Draw pieces
        let ratio = self.board.width / Board::TEXTURE_SIZE as f32;
        let scaled_piece_size = Piece::TEXTURE_SIZE as f32 * ratio;
        let scaled_border_size = Board::BORDER_TEXTURE_SIZE as f32 * ratio;

        for piece in self.pieces.iter_mut() {
            piece.x = self.board.x
                + scaled_border_size
                + (piece.board_x as i8 - 7).abs() as f32 * scaled_piece_size;
            piece.y = self.board.y
                + scaled_border_size
                + (piece.board_y as i8 - 7).abs() as f32 * scaled_piece_size;
            piece.width = scaled_piece_size;
            piece.height = scaled_piece_size;

            piece.draw(&projection, &self.board)?;
        }

        Ok(())
    }
}
