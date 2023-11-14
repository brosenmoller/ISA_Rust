use bevy::prelude::*;

use crate::game::ui::hud::components::*;

pub fn spawn_hud(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    let hud_entity = build_hud(&mut commands, &asset_server);

    
}

pub fn despawn_hud(
    mut commands: Commands,
    hud_query: Query<Entity, With<HUD>>
) {
    if let Ok(hud_entiy) = hud_query.get_single() {
        commands.entity(hud_entiy).despawn_recursive();
    }
}

pub fn build_hud(
    commands: &mut Commands, 
    asset_server: &Res<AssetServer>
) -> Entity {
    let hud_entity = commands.spawn(
        (
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Start,
                    justify_content: JustifyContent::Start,
                    row_gap: Val::Px(8.0),
                    ..default()
                },
                ..default()
            }, 
            HUD {},
        )
    )
    
    
    .id();

    return hud_entity;
}
