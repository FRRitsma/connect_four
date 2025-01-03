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

fn get_iterator(win_method: &WinMethod, column: i8, row: i8) -> Box<dyn Iterator<Item = (i8, i8)>> {
    match win_method {
        WinMethod::Vertical => Box::new(izip!(std::iter::repeat(column).take(3), (row + 1)..(row + 4))),
        WinMethod::Horizontal => Box::new(izip!((column + 1)..(column + 4), std::iter::repeat(row).take(3))),
        WinMethod::DiagonalDownRight => Box::new(izip!((column + 1)..(column + 4), (row + 1)..(row + 4))),
        WinMethod::DiagonalUpRight => Box::new(izip!((column + 1)..(column + 4), (row - 3..row).rev())),
    }
}

pub fn check_all_directions(query: &Query<&Slot>, column: i8, row: i8) -> Option<WinMethod>{
    let primary_player = get_player(query, column, row);
    if primary_player.is_none(){
        return None
    }
    let mut return_value = None;
    for win_method in WinMethod::iter(){
        let iterator = get_iterator(&win_method, column, row);
        if check_single_direction(query, iterator, &primary_player){
            return_value = Some(win_method);
            break
        }
    }
    return_value
}


pub fn check_single_direction<I>(query: &Query<&Slot>, iterator: I, primary_player: &Option<Player>) -> bool
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

