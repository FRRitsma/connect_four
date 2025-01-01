use bevy::color::Color;

pub const GRID_WIDTH: i8 = 6;
pub const GRID_HEIGHT: i8 = 7;
pub const CELL_SIZE: f32 = 50.;

pub const UNOCCUPIED_SLOT_COLOR: Color = Color::WHITE;
pub const DARK_CELL_COLOR: Color = Color::BLACK;
pub const BRIGHT_CELL_COLOR: Color = Color::srgb(0.3, 0.3, 0.3);
pub const BRIGHT_CELL_SELECTION_COLOR: Color = Color::srgb(0.0, 0.3, 0.0);
pub const OCCUPIED_COLOR_PLAYER_1: Color = Color::srgb(1.0, 0.0, 0.0);
pub(crate) const OCCUPIED_COLOR_PLAYER_2: Color = Color::srgb(0.0, 1.0, 0.0);
