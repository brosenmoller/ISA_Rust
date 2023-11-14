pub mod events;
mod systems;

mod game;
mod main_menu;

use game::GamePlugin;
use main_menu::MainMenuPlugin;
use systems::*;

use bevy::prelude::*;

pub const SIZE_MULTIPLIER: f32 = 4.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_state::<AppState>()
        .add_plugins(GamePlugin)
        .add_plugins(MainMenuPlugin)
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, exit_game)
        .add_systems(Update, handle_game_over)
        .add_systems(Update, transistion_to_game_state)
        .add_systems(Update, transistion_to_main_menu_state)
        .run();
}

#[derive(States, Eq, PartialEq, Debug, Clone, Copy, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}