mod player;
mod constants;
mod components;
mod enemy;

use bevy::{prelude::*};
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
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    // commands.insert_resource(Pos(0f32));

}
