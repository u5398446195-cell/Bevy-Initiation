use bevy::prelude::*;

mod player;
mod camera;
mod world;
mod bullet;

use player::PlayerPlugin;
use camera::CameraPlugin;
use world::WorldPlugin;
use bullet::BulletPlugin;

fn main() {
    App::new().add_plugins((DefaultPlugins, PlayerPlugin, CameraPlugin, WorldPlugin, BulletPlugin)).run();
}



