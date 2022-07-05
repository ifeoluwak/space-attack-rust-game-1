use bevy::prelude::Component;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Laser;

#[derive(Debug)]
pub struct Pos(pub f32);

#[derive(Component)]
pub enum PlayerDirection {
    Left,
    Right,
    Up
}


impl From<f32> for PlayerDirection {
    fn from(float: f32) -> Self {
        if float == 0. {
            PlayerDirection::Up
        } else if float < 0. {
            PlayerDirection::Left
        } else {
            PlayerDirection::Right
        }
    }
}