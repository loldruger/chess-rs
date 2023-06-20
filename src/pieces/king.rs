use crate::{square::{Square, SquareKind, SquareStatus}, board::Board};

use super::{Color, MoveStatus};

#[derive(Clone, Copy)]
pub struct King {
    color: Color,
    is_checked: bool,
}

impl King {
    pub fn new(color: Color) -> King {
        King {
            color,
            is_checked: false,
        }
    }

    pub fn is_checked(&self) -> bool {
        self.is_checked
    }

    pub fn set_checked(&mut self, is_checked: bool) {
        self.is_checked = is_checked;
    }

    pub fn get_color(&self) -> Color {
        self.color
    }

    pub fn get_valid_moves(&self, board: &Board, coord_from: Square) -> Vec<(Square, MoveStatus)> {
        let mut valid_moves = Vec::new();

        let current_file = coord_from.get_rank();
        let current_rank = coord_from.get_file();

        let direction = [
            (-1, -1), (-1, 0), (-1, 1),
            ( 0, -1),          ( 0, 1),
            ( 1, -1), ( 1, 0), ( 1, 1)
        ];

        for (file, rank) in direction {
            let dest_file = current_file + file;
            let dest_rank = current_rank + rank;

            if dest_file >= 0 && dest_file < 8 && dest_rank >= 0 && dest_rank < 8 {
                let position = Square::from_position((dest_file, dest_rank));
                if board.is_empty(position) {
                    valid_moves.push((position, MoveStatus::Capturable));
                }
            }
        }
        
        match self.color {
            Color::Black => (),
            Color::White => {
                if board.get_piece(Square::H1).is_some_and(|x| x.get_color() == self.color) &&
                    board.is_empty(Square::F1) &&
                    board.is_empty(Square::G1) &&
                    !board.is_under_attack(Square::G1, Color::Black) &&
                    !board.is_under_attack(Square::F1, Color::Black) &&
                    self.is_checked == false
                {
                    valid_moves.push((Square::G1, MoveStatus::Capturable));
                    valid_moves.push((Square::F1, MoveStatus::Capturable));
                }
                
                if board.get_piece(Square::A1).is_some_and(|x| x.get_color() == self.color) &&
                    board.is_empty(Square::B1) &&
                    board.is_empty(Square::C1) &&
                    board.is_empty(Square::D1) &&
                    !board.is_under_attack(Square::B1, Color::Black) &&
                    !board.is_under_attack(Square::C1, Color::Black) &&
                    !board.is_under_attack(Square::D1, Color::Black) &&
                    self.is_checked == false
                {
                    valid_moves.push((Square::B1, MoveStatus::Capturable));
                    valid_moves.push((Square::C1, MoveStatus::Capturable));
                    valid_moves.push((Square::D1, MoveStatus::Capturable));
                }
            },
        }

        let opponent_move = board
            .get_valid_moves_all(Color::Black)
            .iter()
            .map(|x| {
                match x.1 {
                    MoveStatus::Capturable => x.0,
                    MoveStatus::CapturablePossibly => x.0,
                    _ => Square::None,
                }
            })
            .collect::<Vec<Square>>();

        valid_moves.retain(|x| !opponent_move.contains(&x.0));
        
        valid_moves
    }
}