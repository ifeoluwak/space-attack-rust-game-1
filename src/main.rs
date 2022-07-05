mod player;
mod constants;
mod components;

use bevy::{prelude::*};
use components::Pos;
use player::PlayerPlugin;




fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(start_up_system)
        .add_plugin(PlayerPlugin)
        .run();
}

fn start_up_system(
    mut commands: Commands,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    // commands.insert_resource(Pos(0f32));

}
