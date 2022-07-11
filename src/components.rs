use bevy::{prelude::Component, core::Timer};

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Laser;

#[derive(Component)]
pub struct EnemyLaser;

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
#[derive(Component, Debug)]
pub enum EnemyDirection {
    Left,
    Right
}

impl From<f32> for EnemyDirection {
    fn from(d: f32) -> Self {
        if d < 0. {
            return EnemyDirection::Right;
        }
        EnemyDirection::Left
    }
}

pub struct EnemyCount(pub u32);


#[derive(Component, Debug)]
pub struct ExplosionTimer {
    pub timer: Timer 
}

#[derive(Component, Debug)]
pub struct Explosion;