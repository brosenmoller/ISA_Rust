use bevy::prelude::*;

pub mod components;
mod systems;

use crate::AppState;

use self::systems::*;

use super::SimulationState;

pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE: f32 = 16.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Game), spawn_player)

            .add_systems(OnExit(AppState::Game), despawn_player)

            .add_systems(Update, 
                (
                    (player_movement, confine_player_movement).chain(),
                    player_hit_enemy,
                    player_hit_star,
                )
                .run_if(in_state(AppState::Game))
                .run_if(in_state(SimulationState::Running))
            )
        ;
    }
}