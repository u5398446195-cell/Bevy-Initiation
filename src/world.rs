use bevy::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin{
    fn build(&self, app: &mut App){
        // lumière d'ambiance pour que le perso ne soit pas tout noir côté ombre
        app.insert_resource(AmbientLight { color: Color::WHITE, brightness: 0.4 })
            .add_systems(Startup, (spawn_light, spawn_floor, spawn_wall));
    }
}



fn spawn_wall(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
){
   let walltexture: Handle<Image> = asset_server.load("brique.png");

    let wall = PbrBundle {
    mesh: meshes.add(Mesh::from(shape::Box::new(15.0, 10.0, 0.0))),
    transform: Transform::from_xyz(0.0, 5.0, -5.0),
    material: materials.add(StandardMaterial {
        base_color_texture: Some(walltexture),
        ..default()
    }),
    ..default()
};

    commands.spawn(wall);
}


fn spawn_light(mut commands: Commands){
    let light = PointLightBundle {
        point_light: PointLight{
            intensity: 2000.0,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 5.0, 0.0),
        ..default()
    };

    commands.spawn(light);
}

fn spawn_floor(
     mut commands: Commands,
     mut meshes: ResMut<Assets<Mesh>>,
     mut materials :  ResMut<Assets<StandardMaterial>>
){
    let floor = PbrBundle{
        mesh: meshes.add(Mesh::from(shape::Plane::from_size(15.0))),
        material: materials.add(Color::DARK_GREEN.into()),
        ..default()
    };

    commands.spawn(floor);
}