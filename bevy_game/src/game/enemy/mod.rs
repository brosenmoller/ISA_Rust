use bevy::prelude::*;

pub mod components;
pub mod resources;
mod systems;

use crate::AppState;

use self::resources::*;
use self::systems::*;

use super::SimulationState;

pub const NUMBER_OF_ENEMIES: usize = 4;
pub const ENEMY_SPEED: f32 = 200.0;
pub const ENEMY_SIZE: f32 = 16.0;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<EnemySpawnTimer>()
            .add_systems(OnEnter(AppState::Game), spawn_enemies)
            .add_systems(OnExit(AppState::Game), despawn_enemies)
            .add_systems(Update, (
                enemy_movement,
                update_enemy_direction,
                confine_enemies,
                tick_enemy_spawn_timer,
                spawn_enemies_over_time,
            )
                .run_if(in_state(AppState::Game))
                .run_if(in_state(SimulationState::Running))
            )
        ;
    }
}
