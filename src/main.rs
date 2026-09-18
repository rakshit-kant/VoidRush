use bevy::{input::keyboard::KeyCode, prelude::*};

// represents the playable spaceship
#[derive(Component)]
struct Spaceship;

// defines the velocity of the asteroids
// #[derive(Component)]
// struct Velocity(Vec2);

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // spawns the 2d camera
    commands.spawn(Camera2d);

    // spawn background
    commands.spawn((
        Sprite {
            image: asset_server.load("background.png"),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, -1.0), // kept behind the spaceship via z axis
    ));

    // spawns spaceship
    commands.spawn((
        Spaceship,
        // velocity(Vec2::ZERO),
        Sprite {
            image: asset_server.load("spaceship.png"),
            ..default()
        },
        // idk what this does, so apparently chatgpt told me that it just gets multiplied to the xyz
        // values like x * splat or something like that
        Transform::from_xyz(0.0, -250.0, 0.0).with_scale(Vec3::splat(2.0)),
    )); // omg! look at these brackets
}

fn move_spaceship(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut spaceship: Single<&mut Transform, With<Spaceship>>,
    time: Res<Time>,
) {
    const SPACESHIP_SPEED: f32 = 400.0;

    let mut direction = Vec2::ZERO;

    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        direction.x -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::ArrowRight) {
        direction.x += 1.0;
    }

    if keyboard_input.pressed(KeyCode::ArrowUp) {
        direction.y += 1.0;
    }

    if keyboard_input.pressed(KeyCode::ArrowDown) {
        direction.y -= 1.0;
    }

    if direction != Vec2::ZERO {
        direction = direction.normalize();
    }

    spaceship.translation += direction.extend(0.0) * SPACESHIP_SPEED * time.delta_secs();
}

// exits the application if escape key is pressed as i am too lazy to touch my mouse... one of the
// reasons i use lazyvim
fn exit_on_esc(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut app_exit_events: MessageWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_events.write(AppExit::Success);
    }
}

fn main() {
    // if you need to read this... you are lazier than me as you can't read the above comments
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, (move_spaceship, exit_on_esc))
        .add_systems(Startup, setup)
        .run();
}
