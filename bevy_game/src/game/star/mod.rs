use bevy::prelude::*;

pub mod components;
pub mod resources;
mod systems;

pub const NUMBER_OF_STARS: usize = 4;
pub const STAR_SIZE: f32 = 14.0;

use crate::AppState;
use super::SimulationState;

use self::resources::*;
use self::systems::*;


pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<StarSpawnTimer>()
            
            .add_systems(OnEnter(AppState::Game), spawn_stars)
            .add_systems(OnExit(AppState::Game), despawn_stars)

            .add_systems(Update, 
                (
                    tick_star_spawn_timer, 
                    spawn_stars_over_time
                )
                .run_if(in_state(AppState::Game))
                .run_if(in_state(SimulationState::Running))
            )
        ;
    }
}