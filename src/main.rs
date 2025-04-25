use bevy::{color::palettes::css::RED, ecs::query::QueryData, prelude::*};
mod game_of_life;
use game_of_life::{Cell, ConwaysGame};

#[derive(Resource)]
pub struct ConwaysGameResource(pub ConwaysGame);
#[derive(Resource)]
pub struct Last(f32);

#[derive(Resource)]
pub struct State(u8);

#[derive(Debug, Component, QueryData)]
pub struct Cam;

const WINDOW_WIDTH: f32 = 640.0; // Width of the window
const WINDOW_HEIGHT: f32 = 480.0; // Height of the window
const CELL_SIZE: f32 = 10.;
const CELL_MARGIN: f32 = 1.5;

const NORMAL_BUTTON: Color = Color::srgba(0.25, 0.25, 0.25, 0.5);
const HOVERED_BUTTON: Color = Color::srgba(0.30, 0.30, 0.30, 0.6);
const PRESSED_BUTTON: Color = Color::srgba(0.35, 0.35, 0.35, 0.7);

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
            .build()
    )
    .add_systems(Startup, setup)
    .add_systems(Update, (clear_sprites.before(update), update.after(clear_sprites)))
    .add_systems(Update, (button_system, camera_system))
    .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Camera2d, Cam));

    let mut cells: Vec<Cell> = Vec::new();
    for x in -5..5 {
        for y in -5..5 {
            cells.push(Cell::new(x, y));
        }
    }
    let game = ConwaysGameResource(ConwaysGame::new(cells));
    let last = Last(0.0);
    let state = State(0);

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
    commands.insert_resource(state);

    let play_image_handle = asset_server.load("icons/pause.png");

    commands.spawn((
        Node {
            display: Display::Flex,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            width: Val::Percent(100.0),
            height: Val::Px(50.0),
            ..default()
        },
    )).with_children(|parent| {
        parent.spawn((
            Button,
            Node {
                width: Val::Px(40.),
                height: Val::Px(40.),
                margin: UiRect::all(Val::Px(20.0)), 
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(NORMAL_BUTTON),
        )).with_children(|parent| {
            parent.spawn((
                ImageNode {
                    image: play_image_handle,
                    ..default()
                },
                Transform::from_scale(Vec3::splat(0.07)),
            ));
        });
    });
}

fn clear_sprites(
    mut commands: Commands, 
    query: Query<Entity, With<Sprite>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

fn update(
    mut commands: Commands, 
    mut conways_game: ResMut<ConwaysGameResource>, 
    time: Res<Time>, mut last_time: ResMut<Last>, 
    state: Res<State>
) {
    let l_time = last_time.0;
    let game = &mut conways_game.0;
    if state.0 != 0 && time.elapsed_secs() > l_time + 1.0 {
        let map = game.calculate_next_gen_map();
        game.apply_gen_from_map(&map);
        
        last_time.0 = time.elapsed_secs();
    }

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


fn button_system(
    mut interaction_query: Query<(
        &Interaction,
        &mut BackgroundColor,
        &mut BorderColor,
        &Children,
    ),
    (Changed<Interaction>, With<Button>),
    >,
    mut image_query: Query<&mut ImageNode>,
    asset_server: Res<AssetServer>,
    mut state: ResMut<State>,
) {
    let play_image_handle = asset_server.load("icons/play.png");
    let pause_image_handle = asset_server.load("icons/pause.png");

    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        let mut image  = image_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                state.0 = (state.0 + 1) % 2;
                if state.0 == 0 {
                    image.image = pause_image_handle.clone();
                } else {
                    image.image = play_image_handle.clone();
                }
                *color = PRESSED_BUTTON.into();
                border_color.0 = RED.into();
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}


fn camera_system(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Cam>>,
    mut _state: ResMut<State>,
) {
    // Define movement speed
    let speed = 500.0;
    
    /*
    if keyboard_input.just_pressed(KeyCode::Space) {
        state.0 = (state.0 + 1) % 2;
    }
     */

    // Iterate over all cameras
    for mut transform in query.iter_mut() {
        // Get the current position of the camera
        let mut movement = Vec3::ZERO;

        // Move left with 'A' or right with 'D'
        if keyboard_input.pressed(KeyCode::KeyA) {
            movement.x -= speed * time.delta_secs();
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            movement.x += speed * time.delta_secs();
        }

        // Move up with 'W' or down with 'S'
        if keyboard_input.pressed(KeyCode::KeyW) {
            movement.y += speed * time.delta_secs();
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            movement.y -= speed * time.delta_secs();
        }

        // Update the camera's transform position
        transform.translation += movement;
    }
}