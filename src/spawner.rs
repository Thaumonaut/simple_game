use std::time::Duration;
use bevy::prelude::*;
use bevy_rapier2d::{prelude::*, na::Dynamic};
use bevy::sprite::MaterialMesh2dBundle;
use rand::Rng;
use crate::player_movement::Player;

pub struct SpawnerPlugin;

impl Plugin for SpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawner_setup)
            .add_systems(Update, (spawn_pickup, display_events))
            .insert_resource(SpawnTimer{timer: Timer::from_seconds(2.5, TimerMode::Once)});

    }
}

fn spawner_setup (
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Box::default().into()).into(),
            material: materials.add(ColorMaterial::from(Color::hex("#f3aac5").unwrap())),
            transform: Transform::from_translation(Vec3::new(0., 300., 0.)).with_scale(Vec3::splat(20.)),
            ..default()
        },
        Name::from("Spawner"),
        Spawner {
            health: 5,
        },

    ));
}

fn spawn_pickup (
    mut commands: Commands,
    spawner: Query<&Transform, With<Spawner>>,
    mut timer_config: ResMut<SpawnTimer>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    timer_config.timer.tick(time.delta());
    let transform = spawner.single();
    if timer_config.timer.just_finished() {
        let mut rng = rand::thread_rng();
        timer_config.timer.set_duration(Duration::from_secs(rng.gen_range(1..5)));
        timer_config.timer.reset();

        let handle = asset_server.load("octopus/Idle.png");
        let atlas = TextureAtlas::from_grid(handle, Vec2::new(48., 48.), 4, 1, None, None);
        let atlas_handle = texture_atlases.add(atlas);

        commands.spawn((
                SpriteSheetBundle{
                    texture_atlas: atlas_handle,
                    transform: Transform::from_translation(transform.translation).with_scale(Vec3::splat(1.)),
                    ..default()
                },
                RigidBody::Dynamic,
                Sensor,
                Collider::cuboid(24., 24.),
                ActiveEvents::COLLISION_EVENTS,
            ));
    }
}

fn display_events(
    mut collision_events: EventReader<CollisionEvent>,
    mut contact_force_events: EventReader<ContactForceEvent>
) {
    for collision_event in collision_events.iter() {
        println!("Collision event: {:?}", collision_event);
    }
    for collision_event in contact_force_events.iter() {
        println!("Collision event: {:?}", collision_event);
    }

}

#[derive(Component)]
pub struct Spawner {
    pub health: u16,
}

#[derive(Resource)]
pub struct SpawnTimer{
    timer: Timer
}