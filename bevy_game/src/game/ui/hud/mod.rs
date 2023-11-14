use bevy::prelude::*;

mod systems;
mod components;
mod styles;

use crate::AppState;

use systems::layout::*;
use systems::interactions::*;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Game), spawn_hud)
            .add_systems(OnExit(AppState::Game), despawn_hud)
        ;
    }
}