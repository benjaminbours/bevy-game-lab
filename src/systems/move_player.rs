use bevy::prelude::{
    Assets, Camera, GlobalTransform, Handle, Input, KeyCode, Or, Query, Res, ResMut, Transform,
    Vec2, With,
};

use crate::{
    entities::player::Player,
    post_processing::{MainCamera, PointLightPlayer, PostProcessingMaterial},
};

pub fn move_player(
    keys: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, Or<(With<Player>, With<PointLightPlayer>)>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut custom_materials: Query<&mut Handle<PostProcessingMaterial>>,
    mut assets_custom_materials: ResMut<Assets<PostProcessingMaterial>>,
) {
    let (camera, camera_global_transform) = camera_query.single();
    let custom_material_handle = custom_materials.single();
    let mut custom_material = assets_custom_materials
        .get_mut(custom_material_handle)
        .unwrap();
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
        let shader_light_position = camera
            .world_to_ndc(camera_global_transform, transform.translation)
            .unwrap();
        custom_material.light_position = Vec2::new(
            (shader_light_position.x + 1.) / 2.,
            (shader_light_position.y + 1.) / 2.,
        );
    }
}
