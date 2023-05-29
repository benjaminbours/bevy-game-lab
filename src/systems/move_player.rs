use bevy::{
    app::App,
    prelude::{
        default, shape, Assets, Camera3dBundle, Color, ColorMaterial, Commands, Component, Input,
        KeyCode, Mesh, PbrBundle, Plugin, PointLight, PointLightBundle, Query, Res, ResMut,
        Resource, StandardMaterial, Transform, Vec2, Vec3, With,
    },
    render::camera::ScalingMode,
    sprite::{MaterialMesh2dBundle, Sprite, SpriteBundle},
    time::{Time, Timer, TimerMode},
    DefaultPlugins,
};

use crate::entities::player::Player;

pub fn move_player(
    keys: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    let mut direction = Vec2::ZERO;
    // if keys.any_pressed([KeyCode::Up, KeyCode::W]) {
    //     direction.y += 1.;
    // }
    // if keys.any_pressed([KeyCode::Down, KeyCode::S]) {
    //     direction.y -= 1.;
    // }
    if keys.any_pressed([KeyCode::Right, KeyCode::D]) {
        direction.x += 1.;
    }
    if keys.any_pressed([KeyCode::Left, KeyCode::A]) {
        direction.x -= 1.;
    }
    if direction == Vec2::ZERO {
        return;
    }

    let move_speed = 0.13;
    let move_delta = (direction * move_speed).extend(0.);

    for mut transform in player_query.iter_mut() {
        transform.translation += move_delta;
    }
}
