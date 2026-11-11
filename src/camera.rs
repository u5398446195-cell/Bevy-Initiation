use bevy::prelude::*;
use crate::player::Player;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App){
        app.add_systems(Startup, spawn_camera).add_systems(Update, camera_follow);
    }
}

fn spawn_camera(mut commands: Commands){
    let camera = Camera3dBundle {
        transform: Transform::from_xyz(0.0, 2.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default() 
    };
    commands.spawn(camera);
}

fn camera_follow(
    player_q: Query<&Transform, With<Player>>,
    mut camera_q: Query<&mut Transform, (With<Camera3d>, Without<Player>)>
) {
    let player = match player_q.get_single() {
        Ok(p) => p,
        Err(_) => return,
    };

    let mut camera = match camera_q.get_single_mut() {
        Ok(c) => c,
        Err(_) => return,
    };

    camera.translation = player.translation + Vec3::new(0.0, 2.0, 10.0);
}