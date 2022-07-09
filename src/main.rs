mod player;
mod constants;
mod components;
mod enemy;

use bevy::{prelude::*};
use constants::{BACKGROUND_PNG, GAME_SOUND};
use enemy::EnemyPlugin;
use player::PlayerPlugin;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(start_up_system)
        .add_plugin(PlayerPlugin)
        .add_plugin(EnemyPlugin)
        .run();
}

fn start_up_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>
) {
    let background_handle = asset_server.load(BACKGROUND_PNG);
    let game_sound: Handle<AudioSource> = asset_server.load(GAME_SOUND);

    commands.insert_resource(WindowDescriptor {
        width: 900.,
        height: 600.,
        title: "Space Maruaders".to_string(),
        ..Default::default()
    });

    audio.play_with_settings(game_sound, PlaybackSettings {
        repeat: true,
        volume: 0.3,
        ..Default::default()
    });

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(SpriteBundle {
        texture: background_handle,
        transform: Transform {
            scale: Vec3::new(10., 10., 1.),
            ..Default::default()
        },
        ..Default::default()
    });
    // commands.insert_resource(Pos(0f32));

}
