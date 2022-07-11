use bevy::core::FixedTimestep;

use bevy::sprite::collide_aabb::collide;
use bevy::{prelude::*};
use rand::{thread_rng, Rng};



use crate::components::{Enemy, EnemyDirection, EnemyCount, EnemyLaser, Player, ExplosionTimer, PlayerCount, Pos};
use crate::constants::{ENEMY_PNG, ENEMY_LASER_PNG, ENEMY_LASER_SIZE, PLAYER_SIZE, EXPLOSION_PNG};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(EnemyCount(0))
        .add_system_set(
            SystemSet::new()
            .with_run_criteria(FixedTimestep::step(2.0))
            .with_system(enemy_spawn_system)
        )
        .add_system(enemy_movement_system)
        .add_system_set(
            SystemSet::new()
            .with_run_criteria(FixedTimestep::step(2.5))
            .with_system(enemy_laser_spawn_system)
        )
        .add_system(enemy_laser_movement_system)
        .add_system(laser_collide_system);
    }
}

fn enemy_spawn_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window: Res<Windows>,
    mut enemy_count: ResMut<EnemyCount>
){
    if enemy_count.0 < 2 {
        let primary_window = window.get_primary().unwrap();
        let width = primary_window.width();
    
        let width = (width / 2.) - 100.;
    
        let mut rnd = thread_rng();
        let random_x = rnd.gen_range(-width..width);
    
        let enemy_handle = asset_server.load(ENEMY_PNG);
    
        // commands.insert_resource(EnemyDirection::from(width));
        
        commands.spawn_bundle(SpriteBundle {
            texture: enemy_handle,
            transform: Transform {
                translation: Vec3::new(random_x, 0., 1.),
                scale: Vec3::new(0.5, 0.5, 0.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Enemy)
        .insert(EnemyDirection::from(width));
    
        enemy_count.0 += 1;
    }

}

fn enemy_movement_system(
    mut query: Query<(&mut Transform, &mut EnemyDirection), With<Enemy>>,
    window: Res<Windows>,
    time: Res<Time>,
){
    let random_y = thread_rng().gen_range(0..1) as f32;
    let y = time.seconds_since_startup().cos() as f32;

    let primary_window = window.get_primary().unwrap();
    let width = primary_window.width() / 2.;
    let height = primary_window.height() / 2.;

    for (mut enemy_tf, mut dir) in query.iter_mut() {
            match *dir {
                EnemyDirection::Left => {
                    enemy_tf.translation.x += 1.
                },
                EnemyDirection::Right => {
                    enemy_tf.translation.x -= 1.
                },
            }
            if enemy_tf.translation.y < height {
                enemy_tf.translation.y += y + random_y;
            }

            if enemy_tf.translation.x > width {
                *dir = EnemyDirection::Right;
            } else if enemy_tf.translation.x < -width {
                *dir = EnemyDirection::Left;
            }
    }
}

fn enemy_laser_spawn_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<(&Transform, &EnemyDirection), With<Enemy>>,
) {
    let enemy_laser_handle = asset_server.load(ENEMY_LASER_PNG);

    for (enemy_tf, _) in query.iter() {
        commands.spawn_bundle(SpriteBundle {
            texture: enemy_laser_handle.clone(),
            transform: Transform {
                translation: Vec3::new(enemy_tf.translation.x, enemy_tf.translation.y, 1.),
                scale: Vec3::new(0.3, 0.3, 1.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(EnemyLaser);
    }
}

fn enemy_laser_movement_system(
    mut laser_query: Query<(&mut Transform, Entity), With<EnemyLaser>>,
) {
    for (mut enemy_laser, _) in laser_query.iter_mut() {
        enemy_laser.translation.y -= 5.;
    }
}

fn laser_collide_system(
    mut commands: Commands,
    laser_query: Query<(&Transform, Entity), With<EnemyLaser>>,
    player_query: Query<(&Transform, Entity), With<Player>>,
    mut texture_atlas: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
    mut player_count: ResMut<PlayerCount>,
    mut pos: ResMut<Pos>,
) {
    for (laser_tf, laser_entity) in laser_query.iter() {
        
        match player_query.get_single() {
            Ok((player_tf, player_entity)) => {
                let collision = collide(
                    laser_tf.translation,
                    Vec2::new(ENEMY_LASER_SIZE.0 * 0.3, ENEMY_LASER_SIZE.1 * 0.3),
                    player_tf.translation,
                    Vec2::new(PLAYER_SIZE.0 * 0.6, PLAYER_SIZE.1 * 0.6)
                );
                if let Some(_) = collision {
                    let asset: Handle<Image> = asset_server.load(EXPLOSION_PNG);
                    let atlas = TextureAtlas::from_grid(asset, Vec2::new(64., 64.), 4, 4);
                    let atlas_handle = texture_atlas.add(atlas);

                    commands.spawn_bundle(SpriteSheetBundle {
                        texture_atlas: atlas_handle,
                        transform: Transform {
                            translation: player_tf.translation,
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(ExplosionTimer {
                        timer: Timer::from_seconds(0.05, true)
                    });

                    commands.entity(laser_entity).despawn();
                    commands.entity(player_entity).despawn();

                    player_count.0 -= 1;
                    pos.0 = 0f32;
                }
            },
            Err(_) => {},
        }
    }
}

