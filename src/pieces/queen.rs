use crate::{
    moves::{Placable, Position}, board::Board,
};

use super::Color;

pub struct Queen {
    color: Color,
    position: Position,
    is_selected: bool
}

impl Queen {
    pub fn new(color: Color) -> Self {
        Self {
            color,
            position: Position::from_tuple((0, 0)),
            is_selected: false
        }
    }
}

impl Placable for Queen {
    fn set_position(&mut self, position: Position) -> Result<(), ()> {
        self.position = position;
        Ok(())
    }

    fn get_valid_moves(&self, board: &Board) -> Vec<Position> {
        let mut valid_moves = Vec::new();
        let current_file = self.position.get_file();
        let current_rank = self.position.get_rank();
        
        for i in 0..8 {
            if i != current_file {
                valid_moves.push(Position::from_tuple((i, current_rank)));
            }
        }

        // Vertical moves
        for i in 0..8 {
            if i != current_rank {
                valid_moves.push(Position::from_tuple((current_file, i)));
            }
        }

        // Top-right to bottom-left diagonal moves
        for i in 1..=i32::min(current_file, 7 - current_rank) {
            valid_moves.push(Position::from_tuple((current_file - i, current_rank + i)));
        }

        // Top-left to bottom-right diagonal moves
        for i in 1..=i32::min(7 - current_file, 7 - current_rank) {
            valid_moves.push(Position::from_tuple((current_file + i, current_rank + i)));
        }

        // Bottom-left to top-right diagonal moves
        for i in 1..=i32::min(7 - current_file, current_rank) {
            valid_moves.push(Position::from_tuple((current_file + i, current_rank - i)));
        }

        // Bottom-right to top-left diagonal moves
        for i in 1..=i32::min(current_file, current_rank) {
            valid_moves.push(Position::from_tuple((current_file - i, current_rank - i)));
        }

        valid_moves
    }

    fn get_position(&self) -> Position {
        self.position
    }
}

impl ToString for Queen {
    fn to_string(&self) -> String {
        match self.color {
            Color::Black => "♕".to_owned(),
            Color::White => "♛".to_owned(),
        }
    }
}