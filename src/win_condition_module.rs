use bevy::prelude::Query;
use itertools::izip;
use crate::{get_player, Player, Slot};
use strum_macros::EnumIter;
use strum::IntoEnumIterator;


#[derive(EnumIter)]
pub enum WinMethod{
    Vertical,
    Horizontal,
    DiagonalUpRight,
    DiagonalDownRight,
}

fn get_positions(win_method: &WinMethod, column: i8, row: i8) -> Vec<(i8, i8)> {
    match win_method {
        WinMethod::Vertical => izip!(
            std::iter::repeat(column).take(4),
            (row)..(row + 4)
        )
        .collect(),
        WinMethod::Horizontal => izip!(
            (column)..(column + 4),
            std::iter::repeat(row).take(4)
        )
        .collect(),
        WinMethod::DiagonalDownRight => izip!(
            (column)..(column + 4),
            (row)..(row + 4)
        )
        .collect(),
        WinMethod::DiagonalUpRight => izip!(
            (column)..(column + 4),
            (row - 3..row + 1).rev()
        )
        .collect(),
    }
}

pub fn get_winning_positions(
    query: &Query<&Slot>,
    column: i8,
    row: i8,
) -> Option<Vec<(i8, i8)>> {
    let primary_player = get_player(query, &column, &row);
    if primary_player.is_none() {
        return None;
    }
    for win_method in WinMethod::iter() {
        // Create a new iterator for each win_method
        let positions: Vec<(i8, i8)> = get_positions(&win_method, column, row);
        if check_single_direction(query, &positions, &primary_player) {
            return Some(positions);
        }
    }
    None
}


pub fn check_single_direction(query: &Query<&Slot>, iterator: &Vec<(i8, i8)>, primary_player: &Option<Player>) -> bool
{
    let mut game_over = true;
    for (local_column, local_row) in iterator{
        if &get_player(&query, local_column, local_row) != primary_player{
            game_over = false;
            break
        }
    }
    game_over
}

