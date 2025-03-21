use bevy::prelude::*;

const WINDOW_WIDTH: f32 = 640.0; // Width of the window
const WINDOW_HEIGHT: f32 = 480.0; // Height of the window

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
    .add_systems(Update, update)
    .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn update(_time: Res<Time>,) {

}