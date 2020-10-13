use crate::{
    board::Board, piece::Piece, piece::PieceColor, piece::PieceKind, shader::Shader, traits::Draw,
};

pub struct Game {
    pub board: Board,
    pub pieces: Vec<Piece>,
}

impl Game {
    pub fn new(shader: Shader, atlas_shader: Shader, aspect_ratio: f32) -> Game {
        let board = Board {
            shader,
            aspect_ratio,
        };

        let mut pieces: Vec<Piece> = Vec::new();

        // Create white pieces
        for board_x in 0..8 {
            #[rustfmt::skip]
            let white_pawn = Piece::new(PieceColor::White, PieceKind::Pawn, board_x, 1, atlas_shader, aspect_ratio);

            pieces.push(white_pawn);
        }

        #[rustfmt::skip]
        let left_white_rook = Piece::new(PieceColor::White, PieceKind::Rook, 0, 0, atlas_shader, aspect_ratio);
        #[rustfmt::skip]
        let right_white_rook = Piece::new(PieceColor::White, PieceKind::Rook, 7, 0, atlas_shader, aspect_ratio);

        #[rustfmt::skip]
        let left_white_knight = Piece::new(PieceColor::White, PieceKind::Knight, 1, 0, atlas_shader, aspect_ratio);
        #[rustfmt::skip]
        let right_white_knight = Piece::new(PieceColor::White, PieceKind::Knight, 6, 0, atlas_shader, aspect_ratio);

        #[rustfmt::skip]
        let left_white_bishop = Piece::new(PieceColor::White, PieceKind::Bishop, 2, 0, atlas_shader, aspect_ratio);
        #[rustfmt::skip]
        let right_white_bishop = Piece::new(PieceColor::White, PieceKind::Bishop, 5, 0, atlas_shader, aspect_ratio);

        #[rustfmt::skip]
        let white_queen = Piece::new(PieceColor::White, PieceKind::Queen, 3, 0, atlas_shader, aspect_ratio);

        #[rustfmt::skip]
        let white_king = Piece::new(PieceColor::White, PieceKind::King, 4, 0, atlas_shader, aspect_ratio);

        pieces.push(left_white_rook);
        pieces.push(right_white_rook);
        pieces.push(left_white_knight);
        pieces.push(right_white_knight);
        pieces.push(left_white_bishop);
        pieces.push(right_white_bishop);
        pieces.push(white_queen);
        pieces.push(white_king);

        Game { board, pieces }
    }
}

impl Draw for Game {
    fn draw(&self) {
        self.board.draw();

        for piece in self.pieces.iter() {
            piece.draw();
        }
    }
}
