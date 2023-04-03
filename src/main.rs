#![allow(clippy::too_many_arguments, clippy::type_complexity)]

mod editor;
mod pbr_material;

use bevy_basic_camera::{CameraController, CameraControllerPlugin};
use editor::GameEditorPlugin;
use pbr_material::{swap_standard_material, CustomStandardMaterial};
use std::f32::consts::*;

use bevy::{
    core_pipeline::tonemapping::Tonemapping,
    pbr::{CascadeShadowConfigBuilder, DirectionalLightShadowMap},
    prelude::*,
};
use bevy_editor_pls::prelude::*;

fn main() {
    App::new()
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0 / 5.0f32,
        })
        .insert_resource(DirectionalLightShadowMap { size: 4096 })
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            watch_for_changes: true,
            ..default()
        }))
        .add_plugin(MaterialPlugin::<CustomStandardMaterial>::default())
        .add_plugin(CameraControllerPlugin)
        .add_plugin(GameEditorPlugin)
        .add_startup_system(setup)
        .add_system(swap_standard_material)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            Camera3dBundle {
                transform: Transform::from_xyz(0.7, 0.7, 1.0)
                    .looking_at(Vec3::new(0.0, 0.3, 0.0), Vec3::Y),
                tonemapping: Tonemapping::TonyMcMapface,
                ..default()
            },
            EnvironmentMapLight {
                diffuse_map: asset_server.load("environment_maps/pisa_diffuse_rgb9e5_zstd.ktx2"),
                specular_map: asset_server.load("environment_maps/pisa_specular_rgb9e5_zstd.ktx2"),
            },
        ))
        .insert(CameraController {
            sensitivity: 0.5,
            ..default()
        });

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        // This is a relatively small scene, so use tighter shadow
        // cascade bounds than the default for better quality.
        // We also adjusted the shadow map to be larger since we're
        // only using a single cascade.
        cascade_shadow_config: CascadeShadowConfigBuilder {
            num_cascades: 1,
            maximum_distance: 1.6,
            ..default()
        }
        .into(),
        ..default()
    });
    commands.spawn(SceneBundle {
        scene: asset_server.load("../../temp_assets/CopyRoom.gltf#Scene0"),
        ..default()
    });
}
