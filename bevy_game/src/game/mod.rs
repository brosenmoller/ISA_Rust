use bevy::prelude::*;

mod player;
pub mod enemy;
pub mod score;
pub mod star;
mod systems;
mod ui;

use enemy::EnemyPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;
use star::StarPlugin;
use systems::*;

use crate::{events::GameOver, AppState};

use self::ui::GameUIPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state::<SimulationState>()
            .add_event::<GameOver>()

            .add_systems(OnEnter(AppState::Game), pause_simulation)
            .add_systems(OnExit(AppState::Game), resume_simulation)

            .add_plugins(EnemyPlugin)
            .add_plugins(PlayerPlugin)
            .add_plugins(StarPlugin)
            .add_plugins(ScorePlugin)
            .add_plugins(GameUIPlugin)
            .add_systems(Update, 
                toggle_simulation.run_if(in_state(AppState::Game)))
        ;
    }
}

#[derive(States, Eq, PartialEq, Debug, Clone, Copy, Hash, Default)]
pub enum SimulationState {
    #[default] Running,
    Paused,
}