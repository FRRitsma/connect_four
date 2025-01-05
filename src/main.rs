use std::fmt;
use bevy::app::Startup;
use bevy::DefaultPlugins;
use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::{App, Camera2d, Color, Commands, default, Event, EventReader, EventWriter, KeyCode, Query, Res, ResMut, Resource, Sprite, Transform, Update, Vec2, Vec3, Window};
use bevy::tasks::futures_lite::StreamExt;
use settings::{BRIGHT_CELL_COLOR, BRIGHT_CELL_SELECTION_COLOR, DARK_CELL_COLOR, OCCUPIED_COLOR_PLAYER_1, OCCUPIED_COLOR_PLAYER_2, UNOCCUPIED_SLOT_COLOR};
use crate::settings::{CELL_SIZE, GRID_HEIGHT, GRID_WIDTH, WINNING_CELL_COLOR};
pub mod settings;
mod win_condition_module;
mod cell_and_slot;

use itertools::iproduct;
use cell_and_slot::{Cell, Slot, Coordinates};
use crate::win_condition_module::get_winning_positions;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_event::<GameOverEvent>()
        .insert_resource(SelectedColumn{column: 0})
        .insert_resource(ActivePlayer{player: Player::Player1})
        .add_systems(Startup, (setup, setup_board_game))
        .add_systems(Update, (process_selected_column, process_occupy_slot, check_game_for_win_condition, process_win_condition))
        .run();
}

#[derive(Event, Default)]
struct GameOverEvent{
    winning_positions: Vec<(i8, i8)>
}

#[derive(Resource)]
struct SelectedColumn {
    column: i8,
}


#[derive(PartialEq, Clone)]
enum Player{
    Player1,
    Player2,
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Player::Player1 => write!(f, "Player 1"),
            Player::Player2 => write!(f, "Player 2"),
        }
    }
}


#[derive(Resource)]
struct ActivePlayer {
    player: Player,
}


fn setup(mut commands: Commands){
    commands.spawn(Camera2d);
}


fn process_selected_column(
    mut selected_column: ResMut<SelectedColumn>,
    mut key_event: EventReader<KeyboardInput>,
    mut query: Query<(&mut Cell, &mut Sprite)>,
) {
    for event in key_event.read() {
        if !event.state.is_pressed() { return }
        let key_code = event.key_code;
        match key_code {
            KeyCode::ArrowLeft => {
                selected_column.column = (selected_column.column - 1).max(0);
            }
            KeyCode::ArrowRight => {
                selected_column.column = (selected_column.column + 1).min(GRID_WIDTH - 1);
            }
            _ => { return }
        }
    }

    // Update the color of the cells based on the selected column
    for (cell, mut sprite) in query.iter_mut() {
        if cell.winning_cell{
            sprite.color = WINNING_CELL_COLOR;
        }
        else if cell.column == selected_column.column {
            sprite.color = BRIGHT_CELL_SELECTION_COLOR; // Change to green for selected column
        } else if (cell.column + cell.row) % 2 == 0 {
            sprite.color = BRIGHT_CELL_COLOR;
        } else {
            sprite.color = DARK_CELL_COLOR;
        }
    }
}

fn extract_key_code(mut key_event: EventReader<KeyboardInput>, keys: Vec<KeyCode>) -> Option<KeyCode>{
    let mut key_code = None;
    for event in key_event.read(){
        if !event.state.is_pressed() { break }
        if keys.contains(&event.key_code){
            key_code = Some(event.key_code);
        }
    }
    return key_code;
}

fn process_occupy_slot(
    selected_column: Res<SelectedColumn>,
    mut active_player: ResMut<ActivePlayer>,
    key_event: EventReader<KeyboardInput>,
    mut query: Query<(&mut Slot, &mut Sprite)>,
) {

    if extract_key_code(key_event, vec![KeyCode::Space]).is_none(){
        return
    }

    let color = match &active_player.player{
        Player::Player1 => OCCUPIED_COLOR_PLAYER_1,
        Player::Player2 => OCCUPIED_COLOR_PLAYER_2,
    };

    for row in 0..GRID_HEIGHT{
        for (mut slot, mut sprite) in query.iter_mut(){
            if slot.column != selected_column.column || slot.player.is_some(){
                continue
        }
            if slot.row == row{
                slot.player = Some(active_player.player.clone());
                if active_player.player == Player::Player1{
                    active_player.player = Player::Player2;
                }
                else{
                    active_player.player = Player::Player1
                }
                sprite.color = color;
                return
            }
        }
    }
}

fn get_player(slots_query: &Query<(&Slot)>, column: &i8, row: &i8) -> Option<Player>{
    slots_query.iter()
        .find(|slot| &slot.column == column && &slot.row == row)
        .map(|slot| slot.player.clone())?
}


fn check_game_for_win_condition(query: Query<(&Slot)>, mut event_writer: EventWriter<GameOverEvent>){
    for (column, row) in iproduct!(0..GRID_WIDTH, 0..GRID_HEIGHT){
        let positions = get_winning_positions(&query, column, row);
        if let Some(winning_positions) = positions {
            event_writer.send(GameOverEvent{winning_positions});
            break
        }
    }
}


fn process_win_condition(mut event_reader: EventReader<GameOverEvent>,  mut query: Query<(&mut Cell, &mut Sprite)>,) {
    for event in event_reader.read() {
        for (column, row) in &event.winning_positions {
            for (mut cell, mut sprite) in query.iter_mut() {
                if cell.check_coordinates(column, row) {
                    cell.winning_cell = true;
                    sprite.color = WINNING_CELL_COLOR;
                }
            }
        }
    }
}

fn setup_board_game(mut commands: Commands, query: Query<&Window>, selected_column: ResMut<SelectedColumn>) {
    let screen_width: f32;
    let screen_height: f32;

    if let Ok(primary_window) = query.get_single() {
        screen_width = primary_window.resolution.width();
        screen_height = primary_window.resolution.height();
    } else {
        panic!("Failed to find the primary window!");
    }

    let cell_size = CELL_SIZE.min(screen_width / GRID_WIDTH as f32).min(screen_height / GRID_HEIGHT as f32);
    // Calculate the total width and height of the grid
    let grid_width = GRID_WIDTH as f32 * cell_size;
    let grid_height = GRID_HEIGHT as f32 * cell_size;

    // Center offset to align the grid on the screen
    let x_offset = -grid_width / 2.0;
    let y_offset = -grid_height / 2.0;

    for row in 0..GRID_HEIGHT {
        for column in 0..GRID_WIDTH {
            // Alternate colors between black and white
            let color = if column == selected_column.column {
                Color::srgb(0., 128., 0.)
            } else if (column + row) % 2 == 0 {
                BRIGHT_CELL_COLOR
            } else {
                DARK_CELL_COLOR
            };

            // Spawn a cell
            commands.spawn((
                Sprite {
                    color,
                    custom_size: Some(Vec2::splat(cell_size)),
                    ..default()
                },
                Transform::from_translation(Vec3::new(
                    column as f32 * cell_size + x_offset,
                    row as f32 * cell_size + y_offset,
                    0.0,
                )),
                Cell::new(column as i8, row as i8),
            ));

            // Spawn a slot:
            commands.spawn((
                Sprite {
                    color: UNOCCUPIED_SLOT_COLOR,
                    custom_size: Some(Vec2::splat(cell_size * 0.5)),
                    ..default()
                },
                Transform::from_translation(Vec3::new(
                    column as f32 * cell_size + x_offset,
                    row as f32 * cell_size + y_offset,
                    1.0,
                )),
                Slot::new(column as i8, row as i8),
            ));
        }
    }
}
