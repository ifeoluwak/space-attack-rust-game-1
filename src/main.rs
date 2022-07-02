use std::f32::consts::{FRAC_PI_6};


use bevy::prelude::*;


const PLAYER_PNG: &str = "player.png";
const LASER_PNG: &str = "laser.png";

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Laser;

#[derive(Debug)]
struct Pos(f32);



fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(start_up_system)
        .add_system(keyboard_system)
        .add_system(laser_movement_system)
        .run();
}

fn start_up_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window: Res<Windows>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // let pos = Posi(32f32);

    commands.insert_resource(Pos(0f32));

    let player: Handle<Image> = asset_server.load(PLAYER_PNG);

    let laser: Handle<Image> = asset_server.load(LASER_PNG);

    let primary_window = window.primary();

    let (_, window_height) = (primary_window.width(), primary_window.height());

    commands.spawn_bundle(SpriteBundle {
        texture: player,
        transform: Transform {
            translation: Vec3::new(0., -window_height / 2. + 75. / 2., 0.),
            scale: Vec3::new(0.5, 0.5, 0.),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(Player);
}

fn keyboard_system(
    keyboard: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
    mut pos: ResMut<Pos>,
    window: Res<Windows>,
    asset_server: Res<AssetServer>,
    mut commands: Commands
) {
    let primary_window = window.primary();

    let (window_width, _) = (primary_window.width(), primary_window.height());
    let window_width = window_width / 2. - 30.; // some padding by each side

        let player_transform = query.get_single_mut();
        match player_transform {
            Ok(mut t) => {
                if keyboard.just_pressed(KeyCode::RShift) && keyboard.just_pressed(KeyCode::Right) {
                    // rotate maximum of 30 degrees
                    if pos.0 >= 0f32 {
                        pos.0 -= FRAC_PI_6;
                        t.rotation = Quat::from_rotation_z(pos.0);
                    }
                    // return;
                } else if keyboard.just_pressed(KeyCode::RShift) && keyboard.just_pressed(KeyCode::Left) {
                    // rotate maximum of 30 degrees
                    if pos.0 <= 0f32 {
                        pos.0 += FRAC_PI_6;
                        t.rotation = Quat::from_rotation_z(pos.0);
                    }
                    // return;
                } else if keyboard.pressed(KeyCode::Right) {
                    if window_width > t.translation.x {
                        t.translation.x += 1. / 60. * 300.;
                    }
                    // return;
                } else if keyboard.pressed(KeyCode::Left) {
                    if -window_width < t.translation.x {
                        t.translation.x -= 1. / 60. * 300.;
                    }
                    // return
                } else if keyboard.just_pressed(KeyCode::Space) {
                    let laser: Handle<Image> = asset_server.load(LASER_PNG);

                    let (x, y, z) = (t.translation.x, t.translation.y, t.translation.z);

                    commands.spawn_bundle(SpriteBundle {
                        texture: laser,
                        transform: Transform {
                            translation: Vec3::new(x, y + 50., z),
                            rotation: Quat::from_rotation_z(pos.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(Laser);
                }
            },
            Err(_) => println!("xxxxxx")
        }


    // player_transform.
}

fn laser_movement_system(
    mut commands: Commands,
    mut query: Query<(&mut Transform, Entity), With<Laser>>,
    window: Res<Windows>
) {
    let primary_win = window.primary();

    let (win_width, win_height) = (primary_win.width(), primary_win.height());

    for (mut laser_transform, entity) in query.iter_mut() {
        // println!("{:?}", laser_transform.translation.x);
        println!("{:?}", entity);
        if laser_transform.translation.y < win_height || laser_transform.translation.x < win_width {
            laser_transform.translation.y += 1.;
            // laser_transform.translation.z += 1.;
            laser_transform.translation.x += 1.;
            // laser_transform.rotation = Quat
        } else {
            commands.entity(entity).despawn();
        }
    }
}