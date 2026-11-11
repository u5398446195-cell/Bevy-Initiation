use bevy::prelude::*;
use std::time::Duration;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, (start_idle, player_movement, player_animation));
    }
}

#[derive(Component)]
pub struct Player;

// Ce que fait le perso en ce moment (sert à choisir l'animation)
#[derive(Component, Default, PartialEq, Clone, Copy)]
pub enum PlayerState {
    #[default]
    Idle,
    Run,
    Attack,
}

// Durée restante de l'attaque en cours
#[derive(Component, Default)]
struct AttackTimer(Timer);

// Les animations du .glb, chargées une seule fois.
// Le numéro = l'ordre des animations dans le fichier (76 au total), par exemple :
//   36 Idle · 48 Running_A · 49 Running_B · 72 Walking_A · 62 Spellcast_Shoot
//   60 Spellcast_Long · 22 Cheer · 38 Jump_Full_Long · 27 Dodge_Backward · 23 Death_A
#[derive(Resource)]
struct PlayerAnimations {
    idle: Handle<AnimationClip>,
    run: Handle<AnimationClip>,
    attack: Handle<AnimationClip>,
}

// Mage du pack KayKit Adventurers (Kay Lousberg, licence CC0)
const MODEL: &str = "kaykit_mage/Mage.glb";

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(PlayerAnimations {
        idle: asset_server.load(format!("{MODEL}#Animation36")),
        run: asset_server.load(format!("{MODEL}#Animation48")),
        attack: asset_server.load(format!("{MODEL}#Animation62")),
    });

    commands.spawn((
        SceneBundle {
            scene: asset_server.load(format!("{MODEL}#Scene0")),
            // les pieds du modèle sont à y = 0 ; il mesure ~2,2 unités avec le chapeau,
            // on le réduit un peu pour garder la même taille que l'ancien perso
            transform: Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::splat(0.75)),
            ..default()
        },
        Player,
        PlayerState::default(),
        AttackTimer::default(),
    ));
}

// L'AnimationPlayer n'est pas sur l'entité Player : Bevy le crée sur un enfant
// quand la scène du .glb a fini de charger. Dès qu'il apparaît, on lance « idle ».
fn start_idle(
    animations: Res<PlayerAnimations>,
    mut players: Query<&mut AnimationPlayer, Added<AnimationPlayer>>,
) {
    for mut player in &mut players {
        player.play(animations.idle.clone_weak()).repeat();
    }
}

fn player_movement(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut player_q: Query<(&mut Transform, &mut PlayerState, &mut AttackTimer), With<Player>>,
    cam_q: Query<&Transform, (With<Camera3d>, Without<Player>)>,
) {
    let Ok(cam) = cam_q.get_single() else { return };

    for (mut transform, mut state, mut attack) in player_q.iter_mut() {
      

        // R = attaque
        if keys.just_pressed(KeyCode::R) {
            state.set_if_neq(PlayerState::Attack);
            attack.0 = Timer::from_seconds(1.0, TimerMode::Once);
            continue;
        }

        let mut direction = Vec3::ZERO;

if transform.translation.z < 7.0 {
        if keys.pressed(KeyCode::S) { direction += cam.back();     direction.y = 0.0; }
}

    
if transform.translation.z > -7.0 {
         if keys.pressed(KeyCode::Z) { direction += cam.forward();     direction.y = 0.0; }
}

if transform.translation.x > -7.0 {
   if keys.pressed(KeyCode::Q) { direction += cam.left();     direction.y = 0.0; }
}

if transform.translation.x < 7.0 {
     
        if keys.pressed(KeyCode::D) { direction += cam.right();    direction.y = 0.0; }
}


        let direction = direction.normalize_or_zero();

        if direction == Vec3::ZERO {
            state.set_if_neq(PlayerState::Idle);
            continue;
        }

        state.set_if_neq(PlayerState::Run);
        transform.translation += direction * 2.0 * time.delta_seconds();

        // Le perso se tourne (en douceur) vers là où il court.
        // Un modèle glTF regarde vers +Z, et looking_at oriente -Z vers la cible :
        // on vise donc le point derrière lui pour que son visage soit dans la direction.
        let target = Transform::from_translation(transform.translation)
            .looking_at(transform.translation - direction, Vec3::Y)
            .rotation;
        let t = (10.0 * time.delta_seconds()).min(1.0);
        transform.rotation = transform.rotation.slerp(target, t);
    }
}

// Change d'animation seulement quand l'état change (sinon elle redémarre à chaque image).
// C'est pour ça qu'on écrit l'état avec set_if_neq : il ne marque « changé » que si la valeur change vraiment.
fn player_animation(
    animations: Res<PlayerAnimations>,
    state_q: Query<&PlayerState, (With<Player>, Changed<PlayerState>)>,
    mut anim_q: Query<&mut AnimationPlayer>,
) {
    let Ok(state) = state_q.get_single() else { return };
    let Ok(mut anim) = anim_q.get_single_mut() else { return };

    let fade = Duration::from_millis(200); // fondu entre deux animations
    match state {
        PlayerState::Idle => {
            anim.play_with_transition(animations.idle.clone_weak(), fade).repeat();
        }
        PlayerState::Run => {
            anim.play_with_transition(animations.run.clone_weak(), fade).repeat();
        }
        PlayerState::Attack => {
            anim.play_with_transition(animations.attack.clone_weak(), fade);
            println!("Attack animation started");
        }
    }
}
