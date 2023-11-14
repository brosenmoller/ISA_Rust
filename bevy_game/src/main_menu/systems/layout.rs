use bevy::prelude::*;

use crate::main_menu::{components::*, styles::*};

pub fn spawn_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    let main_menu_entity = build_main_menu(&mut commands, &asset_server);

    
}

pub fn despawn_main_menu(
    mut commands: Commands,
    main_menu_query: Query<Entity, With<MainMenu>>
) {
    if let Ok(main_menu_entiy) = main_menu_query.get_single() {
        commands.entity(main_menu_entiy).despawn_recursive();
    }
}

pub fn build_main_menu(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>
) -> Entity {
    let main_menu_entity = commands.spawn(
        (
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    column_gap: Val::Px(8.0),
                    ..default()
                },
                ..default()
            }, 
            MainMenu {},
        )
    )
    .with_children(|parent: &mut ChildBuilder|{
        // == Title ==
        parent.spawn(
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    height: Val::Px(300.0),
                    width: Val::Px(120.0),
                    ..default()
                },
                ..default()
            }
        )
            .with_children(|parent: &mut ChildBuilder| {
                // Image 1
                parent.spawn(
                    ImageBundle {
                        style: get_image_style(),
                        image: asset_server.load("sprites/Car_3.png").into(),
                        ..default()
                    }
                );
                
                // Text
                parent.spawn(TextBundle {
                    text: get_default_text(asset_server, "Bevy Game", 64.0),
                    ..default()
                });

                // Image 2
                parent.spawn(
                    ImageBundle {
                        style: get_image_style(),
                        image: asset_server.load("sprites/Car_2.png").into(),
                        ..default()
                    }
                );

            })
        ;

        // == Play Button ==
        parent.spawn(
            (
                ButtonBundle {
                    style: get_button_style(),
                    background_color: NORMAL_BUTTON_COLOR.into(),
                    ..default()
                },
                PlayButton {},
            )
        )
            .with_children(|parent: &mut ChildBuilder| {
                parent.spawn(TextBundle {
                    text: get_default_text(asset_server, "Play", 32.0),
                    ..default()
                });
            })
        ;

        // == Quit Button ==
        parent.spawn(
            (
                ButtonBundle {
                    style: get_button_style(),
                    background_color: NORMAL_BUTTON_COLOR.into(),
                    ..default()
                },
                QuitButton {},
            )
        )
            .with_children(|parent: &mut ChildBuilder| {
                parent.spawn(TextBundle {
                    text: get_default_text(asset_server, "Quit", 32.0),
                    ..default()
                });
            })
        ;


    })
    .id();

    return main_menu_entity;
}