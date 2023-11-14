use bevy::{prelude::*, window::PrimaryWindow};
use rand::random;


use super::{components::*, resources::EnemySpawnTimer, NUMBER_OF_ENEMIES, ENEMY_SPEED, ENEMY_SIZE };

use crate::SIZE_MULTIPLIER;

pub fn spawn_enemies(
    mut commands: Commands, 
    window_query: Query<&Window, With<PrimaryWindow>>, 
    asset_server: Res<AssetServer>
) {
    let window = window_query.get_single().unwrap();

    for _ in 0..NUMBER_OF_ENEMIES {
        let random_position = Vec2 {
            x: random::<f32>() * window.width(),
            y: random::<f32>() * window.height(),
        };

        commands.spawn((
            SpriteBundle {
                transform:  Transform::default()
                    .with_translation(Vec3::new(random_position.x, random_position.y, 0.0))
                    .with_scale(Vec3::splat(SIZE_MULTIPLIER)),
                texture: asset_server.load("sprites/Car_1.png"),
                ..default()
            },
            Enemy { direction: Vec2::new(random::<f32>(), random::<f32>()).normalize() }
        ));
    }
}

pub fn despawn_enemies(
    mut commands: Commands,
    enemy_query: Query<Entity, With<Enemy>>
) {
    for enemy_entity in enemy_query.iter() {
        commands.entity(enemy_entity).despawn();
    }
}


pub fn enemy_movement(
    mut enemy_query: Query<(&mut Transform, &Enemy)>,
    time: Res<Time>,
) {
    for (mut transform, enemy) in enemy_query.iter_mut() {
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);
        transform.translation += direction * ENEMY_SPEED * time.delta_seconds();
    }
}

pub fn update_enemy_direction(
    mut enemy_query: Query<(&Transform, &mut Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    let half_player_size = (ENEMY_SIZE * SIZE_MULTIPLIER) / 2.0;
    let x_min = 0.0 + half_player_size;
    let x_max = window.width() - half_player_size;
    let y_min = 0.0 + half_player_size;
    let y_max = window.height() - half_player_size;

    for (transform, mut enemy) in enemy_query.iter_mut(){
        let translation = transform.translation;
        
        if translation.x < x_min || translation.x > x_max {
            enemy.direction.x *= -1.0;
        }
        if translation.y < y_min || translation.y > y_max {
            enemy.direction.y *= -1.0;
        }
    }
}

pub fn confine_enemies(
    mut enemy_query: Query<&mut Transform, With<Enemy>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    let half_player_size = (ENEMY_SIZE * SIZE_MULTIPLIER) / 2.0;
    let x_min = 0.0 + half_player_size;
    let x_max = window.width() - half_player_size;
    let y_min = 0.0 + half_player_size;
    let y_max = window.height() - half_player_size;

    for mut transform in enemy_query.iter_mut(){
        let mut translation = transform.translation;
        
        if translation.x < x_min {
            translation.x = x_min;
        }
        else if translation.x > x_max {
            translation.x = x_max;
        }

        if translation.y < y_min {
            translation.y = y_min;
        }
        else if translation.y > y_max {
            translation.y = y_max;
        }

        transform.translation = translation;
    }
}

pub fn tick_enemy_spawn_timer(mut enemy_spawn_timer: ResMut<EnemySpawnTimer>, time: Res<Time>){
    enemy_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_enemies_over_time(
    mut commands: Commands, 
    window_query: Query<&Window, With<PrimaryWindow>>, 
    asset_server: Res<AssetServer>,
    enemy_spawn_timer: Res<EnemySpawnTimer>
) {
    if enemy_spawn_timer.timer.finished() {
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
                texture: asset_server.load("sprites/Car_1.png"),
                ..default()
            },
            Enemy { direction: Vec2::new(random::<f32>(), random::<f32>()).normalize() }
        ));
    }
}