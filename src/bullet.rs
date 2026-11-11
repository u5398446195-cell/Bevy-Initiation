use bevy::prelude::*;
use crate::player::Player;

pub struct BulletPlugin;


impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
           app.add_systems(Update, (spawn_bullet, move_bullets));
}}


#[derive(Component)]
pub struct Bullet;

fn spawn_bullet(
     keys: Res<Input<KeyCode>>,
     mut commands: Commands,
     mut meshes: ResMut<Assets<Mesh>>,
     mut materials: ResMut<Assets<StandardMaterial>>,
     player_q: Query<&Transform, With<Player>>, ){

if keys.just_pressed(KeyCode::R) { 
    let player = match player_q.get_single() {
        Ok(p) => p,
        Err(_) => return,
    };
    
    let pos = Vec3::new(0.0, 2.0, -1.0) + player.translation; // position du cube 

    commands.spawn((
        PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),  // la forme : cube de 1 de côté
        material: materials.add(Color::rgb(0.0, 0.8, 0.8).into()), // la couleur : rouge (r, g, b de 0 à 1)
        transform: Transform::from_xyz(pos.x, pos.y, pos.z), // la position
        ..default() },
        Bullet,    
    ));
}

}

fn move_bullets(
    mut bullet_q: Query<&mut Transform, With<Bullet>>,
    time: Res<Time>,
){
let direction = Vec3::new(0.0, 0.0, -1.0);
for mut transform in bullet_q.iter_mut(){
    transform.translation += direction * 10.0 * time.delta_seconds();
}

}