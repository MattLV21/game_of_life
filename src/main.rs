use bevy::prelude::*;
use std::collections::HashMap;
mod game_of_life;
use game_of_life::{LivingCell, ConwaysGame};

#[derive(Resource)]
pub struct ConwaysGameResource(pub ConwaysGame);

const WINDOW_WIDTH: f32 = 640.0; // Width of the window
const WINDOW_HEIGHT: f32 = 480.0; // Height of the window
const CELL_SIZE: f32 = 10.;
const CELL_MARGIN: f32 = 1.5;

fn main() {
    App::new()
    .add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Rust Game of Life".into(),
                    resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                    ..default()
                }),
                ..default()
            })
            .build(),
    )
    .add_systems(Startup, setup)
    .add_systems(Update, (clear_sprites.before(update), update.after(clear_sprites)))
    .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    let mut cells: Vec<LivingCell> = Vec::new();
    for x in -5..5 {
        for y in -5..5 {
            cells.push(LivingCell::new(x, y));
        }
    }
    let game = ConwaysGameResource(ConwaysGame::new(cells));
    
    for cell in game.0.get_living_cells().iter() {
        commands.spawn( (
            Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(CELL_SIZE, CELL_SIZE)), // Width and height
                ..default()
            },
            Transform::from_xyz((cell.x * (CELL_SIZE + CELL_MARGIN) as i32) as f32, (cell.y * (CELL_SIZE + CELL_MARGIN) as i32) as f32, 0.0), // Position
        ));
    }

    commands.insert_resource(game);
}

fn clear_sprites(mut commands: Commands, query: Query<Entity, With<Sprite>>) {
    // TODO
    // move entity to cell position and despawn left overs
    // if too few, spawn new ones
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

fn update(mut commands: Commands, _conways_game: Res<ConwaysGameResource>, _time: Res<Time>,) {
    // TODO
    // calcualte the next genaration
    // update conways game based to new genaration
    for cell in _conways_game.0.get_living_cells().iter() {
        commands.spawn( (
            Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(CELL_SIZE, CELL_SIZE)), // Width and height
                ..default()
            },
            Transform::from_xyz((cell.x * (CELL_SIZE + CELL_MARGIN) as i32) as f32, (cell.y * (CELL_SIZE + CELL_MARGIN) as i32) as f32, 0.0), // Position
        ));
    } 
}