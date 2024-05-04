use bevy::prelude::*;
use bevy_editor_pls::prelude::*;
use bevy_prototype_lyon::{draw, prelude::*, shapes::Line};

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
                }),)
        .add_plugins(EditorPlugin::default())
        .add_plugins(ShapePlugin)
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .insert_resource(Msaa::Sample4)
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, (player_movement_system, draw_lines_system))
        .run();
}

#[derive(Component)]
struct Player {}

#[derive(Component, Debug)]
struct Movable {
    rotation_speed: f32,
    acceleration: f32,
    speed: f32,
}

#[derive(Component)]
struct AccelerationText {}

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
            rotation_speed: 3.0,
            acceleration: 10.0,
            speed: 0.0,
        },
    ));
}

fn player_movement_system(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Movable, &mut Transform), With<Player>>,
) {
    let (mut ship_movement, mut ship_transform) = query.single_mut();
    let mut rotation_factor: f32 = 0.0;
    let mut acceleration_factor: f32 = 0.0;
    let facing_towards: Direction3d = ship_transform.local_x();
    let movement_direction: Vec3 = ship_transform.translation;
    let current_rotation: Quat = ship_transform.rotation;

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

    if keyboard_input.just_pressed(KeyCode::Space) {
        println!("- - -\nfacing towards {:?}, movement direction {:?}, speed {:?}, current rotation {:?}", facing_towards, movement_direction, ship_movement.speed, current_rotation);
    };

    // rotate ship
    ship_transform.rotate_z(rotation_factor * ship_movement.rotation_speed * time.delta_seconds());

    // accelerate ship
    ship_transform.translation += (movement_direction * ship_movement.speed
        + facing_towards * acceleration_factor * ship_movement.acceleration)
        * time.delta_seconds();

    // keep ship inside bounds
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
    enum Lines {
        
    } 
    player_movement_system(time, keyboard_input, query)
    }
}

fn draw_lines_system(start_point: Vec2, end_point: Vec2, mut commands: Commands) {
    let shape = Line(start_point, end_point);
    commands.spawn((
        GeometryBuilder::build_as(&shape),
        DrawMode::Stroke {
            outline_mode: StrokeMode::new(Color::WHITE, 10.0),
        },
        Transform::default(),
    ));
}