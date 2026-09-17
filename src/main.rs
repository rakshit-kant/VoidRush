use bevy::{input::keyboard::key, prelude::*};

// represents the playable spaceship
#[derive(component)]
struct spaceship;

// defines the velocity of the asteroids
// #[derive(component)]
// struct velocity(vec2);

fn setup(mut commands: commands, asset_server: res<assetserver>) {
    // spawns the 2d camera
    commands.spawn(camera2d);

    // spawn background
    commands.spawn((
        sprite {
            image: asset_server.load("background.png"),
            ..default()
        },
        transform::from_xyz(0.0, 0.0, -1.0), // kept behind the spaceship via z axis
    ));

    // spawns spaceship
    commands.spawn((
        spaceship,
        // velocity(vec2::zero),
        sprite {
            image: asset_server.load("spaceship.png"),
            ..default()
        },
        // idk what this does, so apparently chatgpt told me that it just gets multiplied to the xyz
        // values like x * splat or something like that
        transform::from_xyz(0.0, -250.0, 0.0).with_scale(vec3::splat(2.0)),
    )); // omg! look at these brackets
}

fn move_spaceship() {
    const spaceship_speed: f32 = 400.0;
}

// exits the application if escape key is pressed as i am too lazy to touch my mouse... one of the
// reasons i use lazyvim
fn exit_on_esc(keyboard_input: res<buttoninput<key>>, mut app_exit_events: messagewriter<appexit>) {
    if keyboard_input.just_pressed(key::escape) {
        app_exit_events.write(appexit::success);
    }
}

fn main() {
    // if you need to read this... you are lazier than me as you can't read the above comments
    app::new()
        .add_plugins(defaultplugins)
        .add_systems(update, exit_on_esc)
        .add_systems(startup, setup)
        .run();
}
