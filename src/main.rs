use bevy::prelude::*;
mod game_of_life;
use game_of_life::{Cell, ConwaysGame};

#[derive(Resource)]
pub struct ConwaysGameResource(pub ConwaysGame);
#[derive(Resource)]
pub struct Last(f32);

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

    let mut cells: Vec<Cell> = Vec::new();
    for x in -20..20 {
        for y in -20..20 {
            cells.push(Cell::new(x, y));
        }
    }
    let game = ConwaysGameResource(ConwaysGame::new(cells));
    let last = Last(0.0);
    
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
    commands.insert_resource(last);
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

fn update(mut commands: Commands, mut conways_game: ResMut<ConwaysGameResource>, time: Res<Time>, mut last_time: ResMut<Last>, keyboard_input: Res<ButtonInput<KeyCode>>) {
    // TODO
    // calcualte the next genaration
    // update conways game based to new genaration
    let l_time = last_time.0;
    let game = &mut conways_game.0;
    if keyboard_input.just_pressed(KeyCode::Space) {
    }
    if time.elapsed_secs() > l_time + 1.0 {
        let map = game.calculate_next_gen_map();
        game.apply_gen_from_map(&map);
        
        last_time.0 = time.elapsed_secs();
    }

    // commands.resour;
    // conways_game.0 = next_gen;

    for cell in game.get_living_cells().iter() {
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