use bevy::scene::prelude;

use bevy::{prelude::*};

use crate::components::Enemy;
use crate::constants::ENEMY_PNG;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_startup_system_to_stage(
            StartupStage::Startup,
            enemy_spawn_system
        );
    }
}

fn enemy_spawn_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    // window: Res<Windows>
){
    let enemy_handle = asset_server.load(ENEMY_PNG);
    commands.spawn_bundle(SpriteBundle {
        texture: enemy_handle,
        transform: Transform {
            translation: Vec3::new(0., 0., 0.),
            scale: Vec3::new(0.5, 0.5, 0.),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(Enemy);
}