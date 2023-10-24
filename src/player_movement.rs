use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (player_input));
    }
}

fn player_input (
    mut query: Query<(
        &mut TextureAtlasSprite, With<Player>
    )>,
    mut velocities: Query<&mut Velocity>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {

    let delta = time.delta_seconds();

    let x_vel = 1000.;
    let y_vel = 500.;

    for mut vel in velocities.iter_mut() {
        if input.just_pressed(KeyCode::Space) {
            vel.linvel.y = y_vel;
        }

        if input.pressed(KeyCode::D) {
            vel.linvel.x += x_vel * delta;
            //vel.linvel.y = y_vel;
        } else if input.pressed(KeyCode::A) {
            vel.linvel.x += -x_vel * delta;
            //vel.linvel.y = y_vel;
        }
    }

    for mut sprite in &mut query {
        if input.pressed(KeyCode::D) {
            sprite.0.flip_x = false;
        }
        if input.pressed(KeyCode::A) {
            sprite.0.flip_x = true;
        }
    }
}

#[derive(Component)]
pub struct Player {
    pub health: usize,
    pub speed: f32,
}