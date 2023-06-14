use crate::{
    moves::{Promotable, Placable}, board::Board, square::Square,
};

use super::{Color, Rook, Bishop, Knight, Queen};

#[derive(Clone, Copy)]
pub struct Pawn {
    color: Color,
    coord: Square,
    is_threatened: bool,
}

impl Pawn {
    pub fn new(color: Color) -> Self { 
        Self {
            color,
            coord: Square::None,
            is_threatened: false,
        }
    }

    pub fn en_passant(&mut self, board: &Board) {

        match self.color {
            Color::Black => todo!(),
            Color::White => todo!(),
        }
    }

    pub fn is_threatened(&self) -> bool {
        self.is_threatened
    }

    pub fn set_threatened(&mut self, is_threatened: bool) {
        self.is_threatened = is_threatened;
    }
}

impl Placable for Pawn {
    fn set_position(&mut self, position: Square) {
        self.coord = position;
    }

    fn get_valid_moves(&self, board: &mut Board, coord: Square, _: bool) -> Vec<Square> {
        let mut valid_moves = Vec::new();

        let (current_file, current_rank) = coord.into_position();

        // White pawn moves forward by one rank
        match self.color {
            Color::White => {
                let target_rank = current_rank - 1;

                let is_enemy_piece1 = board.get_piece(Square::from_position((current_file - 1, target_rank))).is_some_and(|x| x.get_color() != self.color );
                let is_enemy_piece2 = board.get_piece(Square::from_position((current_file + 1, target_rank))).is_some_and(|x| x.get_color() != self.color );

                if board.is_empty(Square::from_position((current_file, target_rank))) {
                    valid_moves.push(Square::from_position((current_file, target_rank)));
                }
                // Double move from the starting rank
                if current_rank == 6 && target_rank >= 1 && board.is_empty(Square::from_position((current_file, target_rank))) {
                    valid_moves.push(Square::from_position((current_file, target_rank - 1)));
                }
                // Capture diagonally to the left
                if current_file > 0 && is_enemy_piece1 {
                    let query = board.get_piece_mut(Square::from_position((current_file - 1, target_rank))).unwrap();

                    query.set_threatened(true);
                    // valid_moves.push(Square::from_position((current_file - 1, target_rank)));
                }
                // Capture diagonally to the right
                if current_file < 7 && is_enemy_piece2 {
                    let query = board.get_piece_mut(Square::from_position((current_file + 1, target_rank))).unwrap();
  
                    query.set_threatened(true);
                    // valid_moves.push(Square::from_position((current_file + 1, target_rank)));
                }
            },
            Color::Black => {
                let target_rank = current_rank + 1;

                let is_enemy_piece1 = board.get_piece(Square::from_position((current_file - 1, target_rank))).is_some_and(|x| x.get_color() != self.color );
                let is_enemy_piece2 = board.get_piece(Square::from_position((current_file + 1, target_rank))).is_some_and(|x| x.get_color() != self.color );

                if target_rank <= 7 && board.is_empty(Square::from_position((current_file, target_rank))) {
                    valid_moves.push(Square::from_position((current_file, target_rank)));
                }
                // Double move from the starting rank
                if current_rank == 1 && target_rank <= 6 && board.is_empty(Square::from_position((current_file, target_rank))) {
                    valid_moves.push(Square::from_position((current_file, target_rank + 1)));
                }
                // Capture diagonally to the left
                if current_file > 0 && target_rank <= 7 && is_enemy_piece1 {
                    let query = board.get_piece_mut(Square::from_position((current_file - 1, target_rank))).unwrap();

                    query.set_threatened(true);
                    // valid_moves.push(Square::from_position((current_file - 1, target_rank)));
                }
                // Capture diagonally to the right
                if current_file < 7 && target_rank <= 7 && is_enemy_piece2 {
                    let query = board.get_piece_mut(Square::from_position((current_file + 1, target_rank))).unwrap();
 
                    query.set_threatened(true);
                    // valid_moves.push(Square::from_position((current_file + 1, target_rank)));
                }
            }
        }

        valid_moves
    }

    fn get_position(&self) -> Square {
        self.coord
    }

    fn get_color(&self) -> Color {
        self.color
    }

    
}

impl Promotable for Pawn {
    fn into_rook(self) -> Rook {
        Rook::new(self.color)
    }

    fn into_bishop(self) -> Bishop {
        Bishop::new(self.color)
    }

    fn into_knight(self) -> Knight {
        Knight::new(self.color)
    }

    fn into_queen(self) -> Queen {
        Queen::new(self.color)
    }
}
