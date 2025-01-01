use bevy::prelude::Query;
use itertools::izip;
use crate::{Player, Slot};

pub fn check_vertical(query: &Query<&Slot>, column: i8, row: i8, primary_player: &Option<Player>) -> bool {
    check_iterator(query, izip!(std::iter::repeat(column).take(3), row+1..row+4), primary_player)
}

pub fn check_horizontal(query: &Query<&Slot>, column: i8, row: i8, primary_player: &Option<Player>) -> bool {
    check_iterator(query, izip!(column+1..column+4, std::iter::repeat(row).take(3)), primary_player)
}

pub fn check_diagonal_up_right(query: &Query<&Slot>, column: i8, row: i8, primary_player: &Option<Player>) -> bool {
    check_iterator(query, izip!(column+1..column+4, row+1..row+4), primary_player)
}

pub fn check_diagonal_down_right(query: &Query<&Slot>, column: i8, row: i8, primary_player: &Option<Player>) -> bool {
    check_iterator(query, izip!(column+1..column+4, (row-3..row).rev()), primary_player)
}

pub fn check_iterator<I>(query: &Query<&Slot>, iterator: I, primary_player: &Option<Player>) -> bool
where
    I: Iterator<Item = (i8, i8)>
{
    let mut game_over = true;
    for (local_column, local_row) in iterator{
        if &crate::get_player(&query, local_column, local_row) != primary_player{
            game_over = false;
            break
        }
    }
    game_over
}