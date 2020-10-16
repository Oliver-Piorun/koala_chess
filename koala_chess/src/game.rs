use crate::board::Board;
use crate::piece::{Piece, PieceColor, PieceKind};
use crate::traits::Draw;

pub struct Game {
    pub board: Board,
    pub pieces: Vec<Piece>,
}

impl Game {
    pub fn new(aspect_ratio: f32) -> Game {
        let board = Board { aspect_ratio };

        let mut pieces: Vec<Piece> = Vec::new();

        // Create pieces
        for board_x in 0..8 {
            #[rustfmt::skip]
            let white_pawn = Piece::new(PieceColor::White, PieceKind::Pawn, board_x, 1, aspect_ratio);
            #[rustfmt::skip]
            let black_pawn = Piece::new(PieceColor::Black, PieceKind::Pawn, board_x, 6, aspect_ratio);

            pieces.push(white_pawn);
            pieces.push(black_pawn);
        }

        #[rustfmt::skip]
        let left_white_rook = Piece::new(PieceColor::White, PieceKind::Rook, 0, 0, aspect_ratio);
        #[rustfmt::skip]
        let right_white_rook = Piece::new(PieceColor::White, PieceKind::Rook, 7, 0, aspect_ratio);
        #[rustfmt::skip]
        let left_black_rook = Piece::new(PieceColor::Black, PieceKind::Rook, 0, 7, aspect_ratio);
        #[rustfmt::skip]
        let right_black_rook = Piece::new(PieceColor::Black, PieceKind::Rook, 7, 7, aspect_ratio);

        #[rustfmt::skip]
        let left_white_knight = Piece::new(PieceColor::White, PieceKind::Knight, 1, 0, aspect_ratio);
        #[rustfmt::skip]
        let right_white_knight = Piece::new(PieceColor::White, PieceKind::Knight, 6, 0, aspect_ratio);
        #[rustfmt::skip]
        let left_black_knight = Piece::new(PieceColor::Black, PieceKind::Knight, 1, 7, aspect_ratio);
        #[rustfmt::skip]
        let right_black_knight = Piece::new(PieceColor::Black, PieceKind::Knight, 6, 7, aspect_ratio);

        #[rustfmt::skip]
        let left_white_bishop = Piece::new(PieceColor::White, PieceKind::Bishop, 2, 0, aspect_ratio);
        #[rustfmt::skip]
        let right_white_bishop = Piece::new(PieceColor::White, PieceKind::Bishop, 5, 0, aspect_ratio);
        #[rustfmt::skip]
        let left_black_bishop = Piece::new(PieceColor::Black, PieceKind::Bishop, 2, 7, aspect_ratio);
        #[rustfmt::skip]
        let right_black_bishop = Piece::new(PieceColor::Black, PieceKind::Bishop, 5, 7, aspect_ratio);

        #[rustfmt::skip]
        let white_queen = Piece::new(PieceColor::White, PieceKind::Queen, 3, 0, aspect_ratio);
        #[rustfmt::skip]
        let black_queen = Piece::new(PieceColor::Black, PieceKind::Queen, 3, 7, aspect_ratio);

        #[rustfmt::skip]
        let white_king = Piece::new(PieceColor::White, PieceKind::King, 4, 0, aspect_ratio);
        #[rustfmt::skip]
        let black_king = Piece::new(PieceColor::Black, PieceKind::King, 4, 7, aspect_ratio);

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
