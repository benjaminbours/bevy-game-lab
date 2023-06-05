use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    prelude::{
        default, shape, App, Assets, Camera, Camera2d, Camera2dBundle, Camera3d, Camera3dBundle,
        Color, Commands, Component, Handle, Image, Mesh, PbrBundle, Plugin, PointLight,
        PointLightBundle, Query, ResMut, StandardMaterial, Transform, UiCameraConfig, Vec2, Vec3,
    },
    reflect::TypeUuid,
    render::{
        camera::RenderTarget,
        mesh::MeshVertexBufferLayout,
        render_resource::{
            AsBindGroup, Extent3d, RenderPipelineDescriptor, ShaderRef,
            SpecializedMeshPipelineError, TextureDescriptor, TextureDimension, TextureFormat,
            TextureUsages,
        },
        texture::BevyDefault,
        view::RenderLayers,
    },
    sprite::{Material2d, Material2dKey, Material2dPlugin, MaterialMesh2dBundle},
    window::Window,
};

use crate::entities::player::Player;

/// Marks the first camera cube (rendered to a texture.)
#[derive(Component)]
pub struct PointLightPlayer;

#[derive(Component)]
pub struct MainCamera;

/// It is generally encouraged to set up post processing effects as a plugin
pub struct PostProcessPlugin;

/// Our custom post processing material
#[derive(AsBindGroup, TypeUuid, Clone)]
#[uuid = "bc2f08eb-a0fb-43f1-a908-54871ea597d5"]
pub struct PostProcessingMaterial {
    #[uniform(0)]
    pub light_position: Vec2,
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
    #[texture(6)]
    #[sampler(7)]
    color_texture: Handle<Image>,
}

impl Material2d for PostProcessingMaterial {
    fn vertex_shader() -> ShaderRef {
        "volumetric_light.vert".into()
    }

    fn fragment_shader() -> ShaderRef {
        "volumetric_light.frag".into()
    }

    // Bevy assumes by default that vertex shaders use the "vertex" entry point
    // and fragment shaders use the "fragment" entry point (for WGSL shaders).
    // GLSL uses "main" as the entry point, so we must override the defaults here
    fn specialize(
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayout,
        _key: Material2dKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        descriptor.vertex.entry_point = "main".into();
        descriptor.fragment.as_mut().unwrap().entry_point = "main".into();
        Ok(())
    }
}

impl Plugin for PostProcessPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(Material2dPlugin::<PostProcessingMaterial>::default())
            .add_startup_system(setup);
    }
}

fn setup(
    mut commands: Commands,
    windows: Query<&Window>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut post_processing_materials: ResMut<Assets<PostProcessingMaterial>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    // This assumes we only have a single window
    let window = windows.single();

    let size = Extent3d {
        width: window.resolution.physical_width(),
        height: window.resolution.physical_height(),
        ..default()
    };

    // This is the texture that will be rendered to.
    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::bevy_default(),
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };

    // fill image.data with zeroes
    image.resize(size);

    let image_handle = images.add(image);

    // let cube_handle = meshes.add(Mesh::from(shape::Cube { size: 4.0 }));
    let cube_handle = meshes.add(Mesh::from(shape::UVSphere {
        radius: 2.0,
        sectors: 100,
        stacks: 100,
    }));
    let cube_material_handle = materials.add(StandardMaterial {
        base_color: Color::hex("ffffff").unwrap(),
        reflectance: 0.02,
        unlit: true,
        ..default()
    });

    // The cube that will be rendered to the texture.
    commands.spawn((
        Player,
        PbrBundle {
            mesh: cube_handle,
            material: cube_material_handle,
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
            ..default()
        },
    ));

    // light
    commands.spawn((
        PointLightPlayer,
        PointLightBundle {
            point_light: PointLight {
                intensity: 1500.0,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
            ..default()
        },
    ));

    // Main camera, first to render
    commands.spawn((
        MainCamera,
        Camera3dBundle {
            camera_3d: Camera3d {
                clear_color: ClearColorConfig::Custom(Color::BLACK),
                ..default()
            },
            camera: Camera {
                target: RenderTarget::Image(image_handle.clone()),
                // order: 2,
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 50.0))
                .looking_at(Vec3::default(), Vec3::Y),
            // transform: Transform::from_xyz(-2.0, 2.5, 100.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        UiCameraConfig { show_ui: false },
    ));

    // This specifies the layer used for the post processing camera, which will be attached to the post processing camera and 2d quad.
    let post_processing_pass_layer = RenderLayers::layer((RenderLayers::TOTAL_LAYERS - 1) as u8);

    let quad_handle: Handle<Mesh> = meshes.add(Mesh::from(shape::Quad::new(Vec2::new(
        size.width as f32,
        size.height as f32,
    ))));

    // This material has the texture that has been rendered.
    let material_handle = post_processing_materials.add(PostProcessingMaterial {
        light_position: Vec2 { x: 0.5, y: 0.5 },
        exposure: 0.18,
        decay: 0.95,
        density: 0.8,
        weight: 0.4,
        samples: 100,
        // TODO: Rename into source image
        color_texture: image_handle,
    });

    // Post processing 2d quad, with material using the render texture done by the main camera, with a custom shader.
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: quad_handle.into(),
            material: material_handle,
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 1.5),
                ..default()
            },
            ..default()
        },
        post_processing_pass_layer,
    ));

    // The post-processing pass camera.
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                // renders after the first main camera which has default value: 0.
                order: 1,
                ..default()
            },
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::Custom(Color::hex("090611").unwrap()),
                ..default()
            },
            ..Camera2dBundle::default()
        },
        post_processing_pass_layer,
    ));
}
