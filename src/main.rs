use bevy::prelude::*;
use bevy_editor_pls::prelude::*;

const LEFT_WALL: f32 = -400.;
const RIGHT_WALL: f32 = 400.;
// y coordinates
const BOTTOM_WALL: f32 = -300.;
const TOP_WALL: f32 = 300.;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Logic Projects Tutorial".into(),
                        resolution: (800.0, 600.0).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .register_type::<Movable>()
        .register_type::<Velocity>()
        .add_plugins(EditorPlugin::default())
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, player_movement_system)
        .add_systems(Update, draw_lines_system)
        .run();
}

#[derive(Component)]
struct Player;

#[derive(Component, Debug, Reflect)]
struct Movable {
    rotation_speed: f32,
    acceleration: f32,
    max_speed: f32,
}

#[derive(Component, Default, Reflect)]
struct Velocity {
    linear_velocity: Vec2,
    angular_velocity: f32,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Game assets
    let rocket_handle = asset_server.load("rocket.png");
    // Camera
    commands.spawn(Camera2dBundle::default());

    // Player
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(50.0, 50.0)),
                ..default()
            },
            texture: rocket_handle.clone(),
            ..default()
        },
        Player {},
        Movable {
            rotation_speed: 4.0,
            acceleration: 3.0,
            max_speed: 360.0,
        },
        Velocity::default(),
        Name::new("Player"),
    ));
}

fn player_movement_system(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&Movable, &mut Velocity, &mut Transform), With<Player>>,
) {
    let (ship_movement, mut velocity, mut ship_transform) = query.single_mut();
    let mut rotation_factor: f32 = 0.0;
    let mut acceleration_factor: f32 = 0.0;
    let facing_towards = ship_transform.local_y().xy();

    // handle inputs
    if keyboard_input.pressed(KeyCode::KeyA) {
        rotation_factor += 1.0;
    };

    if keyboard_input.pressed(KeyCode::KeyD) {
        rotation_factor -= 1.0;
    };

    if keyboard_input.pressed(KeyCode::KeyW) {
        acceleration_factor += 1.0;
    };

    // rotate ship
    ship_transform.rotate_z(rotation_factor * ship_movement.rotation_speed * time.delta_seconds());

    // accelerate ship by calculating new linear velocity
    velocity.linear_velocity += facing_towards * acceleration_factor * ship_movement.acceleration;

    // prevent the ship from moving too fast for gameplay purposes
    velocity.linear_velocity = velocity
        .linear_velocity
        .clamp_length(0.0, ship_movement.max_speed);

    // update ship position
    ship_transform.translation += velocity.linear_velocity.extend(0.0) * time.delta_seconds();

    // keep ship inside bounds by wraping it around
    if ship_transform.translation.x < LEFT_WALL {
        ship_transform.translation.x = RIGHT_WALL;
    }
    if ship_transform.translation.x > RIGHT_WALL {
        ship_transform.translation.x = LEFT_WALL;
    }
    if ship_transform.translation.y < BOTTOM_WALL {
        ship_transform.translation.y = TOP_WALL;
    }
    if ship_transform.translation.y > TOP_WALL {
        ship_transform.translation.y = BOTTOM_WALL;

        //draw lines
        // player_movement_system(time, keyboard_input, query);
    }
}

fn draw_lines_system(query: Query<(&Velocity, &Transform), With<Player>>, mut gizmos: Gizmos) {
    for (velocity, transform) in &query {
        let position = transform.translation;
        // let facing = transform.local_y();
        let start = position.xy();
        gizmos.line_2d(start, start + velocity.linear_velocity * 10.0, Color::RED);
        // gizmos.line_2d(start, start + facing.xy() * 25.0, Color::GREEN);
    }
}
