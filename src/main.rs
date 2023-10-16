use std::collections::HashMap;
use bevy::prelude::*;
use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_rapier2d::prelude::*;


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
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1.0),
            RapierDebugRenderPlugin::default()))
        .add_systems(Startup, (setup_camera, setup_physics, setup))
        .add_systems(Update, (animation, bevy::window::close_on_esc, player_input, apply_forces))
        .run()
}

fn setup_camera (mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(Color::rgb(0.2, 0.2, 0.2)),
            ..default()
        },
        ..default()
    });
}

fn setup_physics (
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    commands.spawn((
        Collider::cuboid(640.0, 25.0),
        RigidBody::Fixed,
        TransformBundle::from(Transform::from_xyz(0.0, -360.0, 0.0)),
        // MaterialMesh2dBundle {
        //     mesh: meshes.add(shape::Box::default().into()).into(),
        //     material: materials.add(ColorMaterial::from(Color::rgb(1.0,1.0,1.0))),
        //     transform: Transform::from_translation(Vec3::new(0.0, -360.0, 0.0)).with_scale(Vec3::new(640.0, 50.0, 1.0)),
        //     ..default()
        // }
       ));
}

fn setup (
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
   // TODO: Move animation loading into its own function and create idle animation that can be triggered when the player is not moving the character

    let mut player_animations = AnimationNode { position: 0, states: HashMap::new()};
    player_animations.states.insert(String::from("running"), AnimationState { path: String::from("Test Running Animation.png"), dimensions: Vec2::new(35.0, 75.0), start_frame: 0, current_frame: 0, end_frame: 4 });

    let texture_handle = asset_server.load("Test Running Animation.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(35.0, 75.0), 4, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);


    let texture_atlas_sprite = TextureAtlasSprite::new(1);

    commands.spawn((
        SpriteSheetBundle {
            // transform: Transform::from_scale(Vec3::splat(1.0)),
            transform: Transform {
                scale: Vec3::splat(1.0),
                translation: Vec3::new(0.0, -290.0, 0.0),
                ..default()
            },
            texture_atlas: texture_atlas_handle,
            sprite: texture_atlas_sprite,
            ..default()
        },
        Collider::cuboid(17.5, 37.5),
        RigidBody::Dynamic,
        Restitution::coefficient(0.5),
        ColliderMassProperties::Density(20.0),
        GravityScale(30.0),
        Damping {
            linear_damping: 1.0,
            angular_damping: 1.0
        },
        Velocity::default(),
        LockedAxes::ROTATION_LOCKED,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        player_animations
    ));
}

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

fn player_input (
    mut query: Query<(
        &mut  Transform,
        &mut TextureAtlasSprite
    )>,
    mut velocities: Query<&mut Velocity>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {

    for mut vel in velocities.iter_mut() {
        if input.just_pressed(KeyCode::Space) {
            vel.linvel.y = 800.0;
        }
        if input.pressed(KeyCode::D) {
            vel.linvel.x = 500.0;
        } else if input.pressed(KeyCode::A) {
            vel.linvel.x = -500.0;
        }
    }

    for (mut controller, mut sprite) in &mut query {

        // let mut translate = Vec2::ZERO;
        // let speed = 500.0;
        //
        if input.pressed(KeyCode::D) {
            sprite.flip_x = false;
            //translate.x =  speed * time.delta_seconds();
        }
        if input.pressed(KeyCode::A) {
            sprite.flip_x = true;
            //translate.x =  -speed * time.delta_seconds();
        }

        //controller.translation += Vec3::new(translate.x, translate.y, 0.0);

    }
}

fn apply_forces (
    mut ext_impulses: Query<&mut ExternalImpulse>,
    input: Res<Input<KeyCode>>
) {
    for mut ext_impulse in ext_impulses.iter_mut() {
        if input.just_pressed(KeyCode::Space) {
            println!("Jump!");
        }
            ext_impulse.impulse = Vec2::new(1.0, 1000.0);
    }
}

#[derive(Component)]
pub struct AnimationNode {
    position: usize,
    states: HashMap<String, AnimationState>,
}

#[derive(Component)]
pub struct AnimationState {
    path: String,
    dimensions: Vec2,
    start_frame: usize,
    current_frame: usize,
    end_frame: usize
}

impl Default for AnimationState {
    fn default() -> AnimationState {
        AnimationState { path: String::new(), dimensions: Vec2 { x: 0.0, y: 0.0 }, start_frame: 0, current_frame: 0, end_frame: 0 }
    }
}

#[derive(Component)]
pub struct Player {
    health: usize,
    speed: f32,
}