use std::{f32::consts::FRAC_PI_6};

use bevy::{prelude::*, sprite::collide_aabb::collide, utils::HashSet};

use crate::{components::{Player, Laser, Pos, PlayerDirection, Enemy, EnemyCount, ExplosionTimer}, constants::{PLAYER_PNG, LASER_PNG, LASER_SOUND, ENEMY_SIZE, LASER_SIZE, ENEMY_COLLIDE_SOUND, EXPLOSION_PNG}};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
       app.add_startup_system_to_stage(StartupStage::Startup, player_init_system)
       .insert_resource(Pos(0f32))
       .add_system(laser_movement_system)
       .add_system(keyboard_system)
       .add_system(laser_collide_system)
       .add_system(explosion_system);
    }
}

fn player_init_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window: Res<Windows>,
) {
    let player: Handle<Image> = asset_server.load(PLAYER_PNG);

    let primary_window = window.primary();

    let (_, window_height) = (primary_window.width(), primary_window.height());

    commands.spawn_bundle(SpriteBundle {
        texture: player,
        transform: Transform {
            translation: Vec3::new(0., -window_height / 2. + 75. / 2., 1.),
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
    audio: Res<Audio>,
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
            } else if keyboard.pressed(KeyCode::Right) {
                if window_width > t.translation.x {
                    t.translation.x += 1. / 60. * 300.;
                }
            } else if keyboard.pressed(KeyCode::Left) {
                if -window_width < t.translation.x {
                    t.translation.x -= 1. / 60. * 300.;
                }
            } else if keyboard.just_pressed(KeyCode::Space) {
                let laser: Handle<Image> = asset_server.load(LASER_PNG);

                let (x, y, z) = (t.translation.x, t.translation.y, t.translation.z);

                let mut mod_x = x;

                if pos.0 < 0. {
                    mod_x += 28.;
                } else if pos.0 > 0. {
                    mod_x -= 28.;
                }

                let laser_sound: Handle<AudioSource> = asset_server.load(LASER_SOUND);

                commands.spawn_bundle(SpriteBundle {
                    texture: laser,
                    transform: Transform {
                        translation: Vec3::new(mod_x, y + 50., z),
                        rotation: Quat::from_rotation_z(pos.0),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Laser)
                .insert(PlayerDirection::from(pos.0));
                
                audio.play(laser_sound);

            }
        },
        Err(_) => {}
    }
}

fn laser_movement_system(
    mut commands: Commands,
    mut laser_query: Query<(&mut Transform, Entity, &PlayerDirection), With<Laser>>,
    window: Res<Windows>,
) {
    let primary_win = window.primary();

    let (win_width, win_height) = (primary_win.width(), primary_win.height());

    for (mut laser_transform, entity, direction) in laser_query.iter_mut() {
        if laser_transform.translation.y < win_height || laser_transform.translation.x < win_width {

            match direction {
                PlayerDirection::Up => laser_transform.translation.y += 1. * 20.,
                PlayerDirection::Left => {
                    laser_transform.translation.y += 1. * 20.;
                    laser_transform.translation.x += 1. * 20.;
                },
                PlayerDirection::Right => {
                    laser_transform.translation.y += 1. * 20.;
                    laser_transform.translation.x -= 1.  * 20.;
                }
            }
        } else {
            commands.entity(entity).despawn();
        }
    }
}

fn laser_collide_system(
    mut commands: Commands,
    laser_query: Query<(&Transform, Entity), With<Laser>>,
    enemy_query: Query<(&Transform, Entity), With<Enemy>>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
    mut enemy_count: ResMut<EnemyCount>,
    mut texture_atlas: ResMut<Assets<TextureAtlas>>
) {
    let mut despawned_entities: HashSet<Entity> = HashSet::new();
    for (laser, laser_entity) in laser_query.iter() {
        if despawned_entities.contains(&laser_entity) {
            continue;
        }

        for (&e_transform, enemy_entity) in enemy_query.iter() {
            if despawned_entities.contains(&laser_entity) || despawned_entities.contains(&enemy_entity) {
                continue;
            }

            let collision = collide(laser.translation,
                Vec2::new(LASER_SIZE.0 * 0.5, LASER_SIZE.1 * 0.5),
                e_transform.translation,
                Vec2::new(ENEMY_SIZE.0 * 0.5, ENEMY_SIZE.1 * 0.5)
            );

            match collision {
                Some(_) => {
                    let audio_handle: Handle<AudioSource> = asset_server.load(ENEMY_COLLIDE_SOUND);
                    audio.play(audio_handle);

                    commands.entity(enemy_entity).despawn();
                    commands.entity(laser_entity).despawn();

                    enemy_count.0 -= 1;
                    despawned_entities.insert(enemy_entity);
                    despawned_entities.insert(laser_entity);

                    let texture: Handle<Image> = asset_server.load(EXPLOSION_PNG);
                    let atlas = TextureAtlas::from_grid(texture, Vec2::new(64., 64.), 4, 4);
                    let texture_atlas_handle = texture_atlas.add(atlas);

                    commands.spawn_bundle(SpriteSheetBundle {
                        texture_atlas: texture_atlas_handle,
                        transform: Transform {
                            translation: e_transform.translation,
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(ExplosionTimer {
                        timer: Timer::from_seconds(0.05, true)
                    });

                },
                None => {},
            }
        }
    }
}

fn explosion_system(
    mut commands: Commands,
    mut query: Query<(Entity, &mut ExplosionTimer, &mut TextureAtlasSprite)>,
    time: Res<Time>
) {
    for (entity, mut explosion_timer, mut sprite) in query.iter_mut() {
        explosion_timer.timer.tick(time.delta());

        if explosion_timer.timer.just_finished() {
            sprite.index += 1;
            if sprite.index >= 16 {
                commands.entity(entity).despawn();
            }
        }
    }
}