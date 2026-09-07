use bevy::{input::keyboard::Key, prelude::*};

// Represents the Playable Spaceship
#[derive(Component)]
struct Spaceship;

// Defines the Velocity of the Asteroids
// #[derive(Component)]
// struct Velocity(Vec2);

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawns the 2D Camera
    commands.spawn(Camera2d);

    // Spawn Background
    commands.spawn((
        Sprite {
            image: asset_server.load("background.png"),
            ..default()
        },
        // Idk what this does
        Transform::from_xyz(0.0, 0.0, -1.0), // Kept behind the spaceship via Z axis
    ));

    // Spawns Spaceship
    commands.spawn((
        Spaceship,
        // Velocity(Vec2::ZERO),
        Sprite {
            image: asset_server.load("spaceship.png"),
            ..default()
        },
        // Idk what this does
        Transform::from_xyz(0.0, -250.0, 0.0).with_scale(Vec3::splat(2.0)),
    )); // OMG! Look at these Brackets
}

// Exits the Application if Escape Key is pressed as I am too lazy to touch my Mouse... One of the
// Reasons I use Lazyvim
fn exit_on_esc(keyboard_input: Res<ButtonInput<Key>>, mut app_exit_events: MessageWriter<AppExit>) {
    if keyboard_input.just_pressed(Key::Escape) {
        app_exit_events.write(AppExit::Success);
    }
}

fn main() {
    // If you need to Read this... You are lazier than me as You can't read the Above Comments
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, exit_on_esc)
        .add_systems(Startup, setup)
        .run();
}
