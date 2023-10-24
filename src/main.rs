mod player_movement;
mod spawner;

use std::borrow::Cow;
// use std::collections::HashMap;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::prelude::*;
use bevy::core_pipeline::clear_color::ClearColorConfig;
// use bevy::render::camera::ScalingMode;
use bevy_rapier2d::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use crate::player_movement::Player;


// TODO: Add collision and maybe physics?

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set( WindowPlugin {
                primary_window: Some(Window {
                    title: "Simple Game".into(),
                    resolution: (1280.0, 720.0).into(),
                    resizable: false,
                    ..default()
                }),
                ..default()
            })
        )
        .add_plugins((
            WorldInspectorPlugin::new(),
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
            RapierDebugRenderPlugin::default()))
        .add_plugins((
            player_movement::PlayerPlugin,
            spawner::SpawnerPlugin,
        ))
        .add_systems(Startup, (setup_camera, setup_physics, setup))
        .add_systems(Update, (animation, bevy::window::close_on_esc))
        .run()
}

fn setup_camera (mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(Color::rgb(0.2, 0.2, 0.2)),
            ..default()
        },
        // projection: OrthographicProjection {
        //     scaling_mode: ScalingMode::Fixed
        //     {
        //         width: 1000.,
        //         height: 1000.,
        //     },
        //
        //     ..default()
        // },
        ..default()
    });
}

fn setup_physics (
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    let entity_id = commands.spawn((
        Collider::cuboid(1.0, 0.5),
        RigidBody::Fixed,
        // TransformBundle::from(Transform::from_xyz(0.0, -300.0, 0.0)),
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Box::default().into()).into(),
            material: materials.add(ColorMaterial::from(Color::rgb(1.0,1.0,1.0))),
            transform: Transform::from_translation(Vec3::new(0.0, -330.0, 0.0))
            .with_scale(Vec3::new(660.0, 25.0, 1.0)),
            ..default()
        }
       )).id();

       println!("Id: {:?}", entity_id);
}

// fn animation_setup (
//     mut commands: Commands,
//     asset_server: Res<AssetServer>,
//     mut texture_atlases: ResMut<Assets<TextureAtlas>>,
// ) {
//     let player_animations = AnimationNode { position: 0};
//     let swordfish_texture_handle = asset_server.load_folder("sword_fish");
//     let swordfish_texture_handle = swordfish_texture_handle.unwrap();
//
//     let mut texture_atlas_handles: Vec<Handle<TextureAtlas>> = Vec::new();
//
//     for texture in swordfish_texture_handle
//     {
//         let typed_texture = texture.typed::<Image>();
//         let texture_atlas = TextureAtlas::from_grid(typed_texture, Vec2::new(48., 48.), 4, 1, None, None);
//         let atlas_handle = texture_atlases.add(texture_atlas);
//         texture_atlas_handles.push(atlas_handle);
//     }
//
//     let current_state = SwordFishStates::Walk;
//     let current_texture = texture_atlas_handles[current_state as usize].clone();
//     let texture_atlas_sprite = TextureAtlasSprite::new(1);
// }

fn setup (
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
   // TODO: Move animation loading into its own function and create idle animation that can be triggered when the player is not moving the character

    let player_animations = AnimationNode { position: 0};
    let swordfish_texture_handle = asset_server.load_folder("sword_fish");
    let swordfish_texture_handle = swordfish_texture_handle.unwrap();

    let mut texture_atlas_handles: Vec<Handle<TextureAtlas>> = Vec::new();

    for texture in swordfish_texture_handle
    {
        let typed_texture = texture.typed::<Image>();
        let texture_atlas = TextureAtlas::from_grid(typed_texture, Vec2::new(48., 48.), 4, 1, None, None);
        let atlas_handle = texture_atlases.add(texture_atlas);
        texture_atlas_handles.push(atlas_handle);
    }

    let current_texture = texture_atlas_handles[4].clone();
    let texture_atlas_sprite = TextureAtlasSprite::new(1);

    //println!("{:?}", texture_atlases);

    commands.spawn((
        SpriteSheetBundle {
            // transform: Transform::from_scale(Vec3::splat(2.0)),
            transform: Transform {
                scale: Vec3::splat(2.0),
                translation: Vec3::new(0.0, 0.0, 2.0),
                ..default()
            },
            texture_atlas: current_texture,
            sprite: texture_atlas_sprite,
            ..default()
        },
        Collider::cuboid(24., 12.),
        RigidBody::Dynamic,
        ColliderMassProperties::Mass(1000.0),
        GravityScale(5.0),
        Damping {
            linear_damping: 1.5,
            angular_damping: 1.0
        },
        Friction {
            coefficient: 0.2,
            ..default()
        },
        Name::new(Cow::from("Character")),
        Velocity::default(),
        LockedAxes::ROTATION_LOCKED,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        player_animations,
        Player {
            health: 5,
            speed: 1000.
        }
    ));
}
// #[derive(Resource)]
// enum SwordFishStates {
//     Attack,
//     Death ,
//     Hurt ,
//     Idle,
//     Walk,
// }

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn animation (
    the_time: Res<Time>,
    mut query: Query<(
        &mut TextureAtlasSprite,
        &mut AnimationTimer,
        &mut AnimationNode
    )>

) {
    for (mut tex, mut timer, mut frame) in &mut query {

        timer.tick(the_time.delta());
        if timer.just_finished() {
            frame.position = if frame.position < 3 {
                frame.position + 1
            } else {
                0
            };
            tex.index = frame.position;
        }
    }
}

// fn apply_forces (
//     mut ext_impulses: Query<&mut ExternalImpulse>,
//     input: Res<Input<KeyCode>>
// ) {
//     for mut ext_impulse in ext_impulses.iter_mut() {
//         if input.pressed(KeyCode::Space) {
//             println!("Jump!");
//         }
//             ext_impulse.impulse = Vec2::new(1.0, 1000.0);
//     }
// }

#[derive(Component)]
pub struct AnimationNode {
    position: usize,
    // states: HashMap<String, AnimationState>,
}

// #[derive(Component)]
// pub struct AnimationState {
//     path: String,
//     dimensions: Vec2,
//     start_frame: usize,
//     current_frame: usize,
//     end_frame: usize
// }

// impl Default for AnimationState {
//     fn default() -> AnimationState {
//         AnimationState { path: String::new(), dimensions: Vec2 { x: 0.0, y: 0.0 }, start_frame: 0, current_frame: 0, end_frame: 0 }
//     }
// }