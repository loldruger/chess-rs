use crate::{board::Board, pieces::Color, square::Square};

pub trait Placable {
    fn get_color(&self) -> Color;
    fn get_position(&self, board: &Board) -> Option<Square>;
    fn get_valid_moves(&self, board: &Board, coord: Square, is_threaten: bool) -> Vec<Square>;
}
