use bevy::{
    app::App,
    pbr::{MaterialPipeline, MaterialPipelineKey},
    prelude::{
        default, shape, AlphaMode, Assets, Camera3dBundle, Color, ColorMaterial, Commands,
        Component, Handle, Image, Input, KeyCode, Material, MaterialMeshBundle, MaterialPlugin,
        Mesh, PbrBundle, Plugin, PointLight, PointLightBundle, Query, Res, ResMut, Resource,
        StandardMaterial, Transform, Vec2, Vec3, With,
    },
    reflect::TypeUuid,
    render::{
        camera::ScalingMode,
        mesh::MeshVertexBufferLayout,
        render_resource::{
            AsBindGroup, RenderPipelineDescriptor, ShaderRef, SpecializedMeshPipelineError,
        },
    },
    sprite::{MaterialMesh2dBundle, Sprite, SpriteBundle},
    time::{Time, Timer, TimerMode},
    DefaultPlugins,
};

mod entities;
mod post_processing;
mod systems;

use entities::player::Player;
use post_processing::PostProcessSettings;
use systems::{gravity::gravity, move_player::move_player};

use crate::post_processing::PostProcessPlugin;

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app
            // .insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_startup_systems((setup, spawn_player, spawn_floor))
            .add_systems((
                move_player,
                //  gravity
            ));
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut camera_bundle = Camera3dBundle::default();
    // camera_bundle.projection.scaling_mode = ScalingMode::FixedVertical(10.);
    camera_bundle.transform = Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y);
    commands.spawn((camera_bundle, PostProcessSettings { intensity: 10., other: 10. }));
    // commands.spawn(MaterialMesh2dBundle {
    //     mesh: meshes.add(Mesh::from(shape::Circle::new(1.0))).into(),
    //     transform: Transform::default().with_scale(Vec3::splat(128.)),
    //     material: materials.add(ColorMaterial::from(Color::PURPLE)),
    //     ..default()
    // });
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

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Player,
        MaterialMeshBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            // material: materials.add(CustomMaterial {
            //     // color: Color::BLUE,
            //     light_position: Vec2 { x: 0.5, y: 0.5 },
            //     exposure: 0.18,
            //     decay: 0.95,
            //     density: 0.8,
            //     weight: 0.4,
            //     samples: 50,
            //     // color_texture: Some(asset_server.load("branding/icon.png")),
            //     alpha_mode: AlphaMode::Blend,
            // }),
            transform: Transform::from_xyz(0.0, 1.0, 0.0),
            ..default()
        },
    ));
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}

// This is the struct that will be passed to your shader
#[derive(AsBindGroup, Clone, TypeUuid)]
#[uuid = "4ee9c363-1124-4113-890e-199d81b00281"]
pub struct CustomMaterial {
    #[uniform(0)]
    light_position: Vec2,
    #[uniform(1)]
    exposure: f32,
    #[uniform(2)]
    decay: f32,
    #[uniform(3)]
    density: f32,
    #[uniform(4)]
    weight: f32,
    #[uniform(5)]
    samples: i32,
    // #[uniform(5)]
    // samples: i32,
    // #[texture(1)]
    // #[sampler(2)]
    // color_texture: Option<Handle<Image>>,
    alpha_mode: AlphaMode,
}

/// The Material trait is very configurable, but comes with sensible defaults for all methods.
/// You only need to implement functions for features that need non-default behavior. See the Material api docs for details!
/// When using the GLSL shading language for your shader, the specialize method must be overridden.
impl Material for CustomMaterial {
    fn vertex_shader() -> ShaderRef {
        "volumetric_light.vert".into()
    }

    fn fragment_shader() -> ShaderRef {
        "volumetric_light.frag".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }

    // Bevy assumes by default that vertex shaders use the "vertex" entry point
    // and fragment shaders use the "fragment" entry point (for WGSL shaders).
    // GLSL uses "main" as the entry point, so we must override the defaults here
    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayout,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        descriptor.vertex.entry_point = "main".into();
        descriptor.fragment.as_mut().unwrap().entry_point = "main".into();
        Ok(())
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PostProcessPlugin)
        .add_plugin(MaterialPlugin::<CustomMaterial>::default())
        .add_plugin(SetupPlugin)
        .run();
}
