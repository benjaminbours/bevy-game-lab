use bevy::{
    app::App,
    prelude::{
        default, shape, Assets, Color, Commands, Mesh, PbrBundle, Plugin, ResMut, StandardMaterial,
    },
    DefaultPlugins,
};

mod entities;
mod post_processing;
mod systems;

use entities::player::Player;
use systems::{gravity::gravity, move_player::move_player};

use crate::post_processing::PostProcessPlugin;

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app
            // .insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_startup_system(spawn_floor)
            .add_systems((
                move_player,
                //  gravity
            ));
    }
}

fn spawn_floor(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(500.0).into()),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PostProcessPlugin)
        .add_plugin(SetupPlugin)
        .run();
}
