use bevy::{prelude::*, window::PrimaryWindow};
use rand::random;

use crate::SIZE_MULTIPLIER;
use super::{NUMBER_OF_STARS, resources::StarSpawnTimer, components::Star};

pub fn spawn_stars(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>
) {
    let window = window_query.get_single().unwrap();

    for _ in 0..NUMBER_OF_STARS {
        let random_position = Vec2 {
            x: random::<f32>() * window.width(),
            y: random::<f32>() * window.height(),
        };

        commands.spawn((
            SpriteBundle {
                transform:  Transform::default()
                    .with_translation(Vec3::new(random_position.x, random_position.y, 0.0))
                    .with_scale(Vec3::splat(SIZE_MULTIPLIER)),
                texture: asset_server.load("sprites/ScoreBoard.png"),
                ..default()
            },
            Star { },
        ));
    }
}

pub fn despawn_stars(
    mut commands: Commands,
    star_query: Query<Entity, With<Star>>
) {
    for star_entity in star_query.iter() {
        commands.entity(star_entity).despawn();
    }
}

pub fn tick_star_spawn_timer(mut star_spawn_timer: ResMut<StarSpawnTimer>, time: Res<Time>){
    star_spawn_timer.timer.tick(time.delta());
}


pub fn spawn_stars_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    star_spawn_timer: Res<StarSpawnTimer>
) {
    if star_spawn_timer.timer.finished() {
        let window = window_query.get_single().unwrap();
        let random_position = Vec2 {
            x: random::<f32>() * window.width(),
            y: random::<f32>() * window.height(),
        };

        commands.spawn((
            SpriteBundle {
                transform:  Transform::default()
                    .with_translation(Vec3::new(random_position.x, random_position.y, 0.0))
                    .with_scale(Vec3::splat(SIZE_MULTIPLIER)),
                texture: asset_server.load("sprites/ScoreBoard.png"),
                ..default()
            },
            Star { },
        ));
    }
}