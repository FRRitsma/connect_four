use bevy::prelude::Component;
use crate::Player;

pub trait Coordinates{
    fn check_coordinates(&self, column: &i8, row: &i8) -> bool;
}

#[derive(Component)]
pub struct Cell{
    pub winning_cell: bool,
    pub column: i8,
    pub row: i8,
}

impl Cell{
    pub fn new(column: i8, row: i8) -> Cell{
        Cell{
            winning_cell: false,
            column,
            row,
        }
    }
}

impl Coordinates for Cell{
    fn check_coordinates(&self, column: &i8, row: &i8) -> bool {
        self.column == *column && self.row == *row
    }
}

#[derive(Component)]
pub struct Slot{
    pub player: Option<Player>,
    pub column: i8,
    pub row: i8,
}

impl Slot{
    pub fn new(column: i8, row: i8) -> Slot {
        Slot{player: None, column, row}
    }
}

impl Coordinates for Slot{
    fn check_coordinates(&self, column: &i8, row: &i8) -> bool {
        self.column == *column && self.row == *row
    }
}