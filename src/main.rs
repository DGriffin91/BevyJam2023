#![allow(clippy::too_many_arguments, clippy::type_complexity)]

mod character_controller;
mod editor;
mod light_shafts;
mod pbr_material;
mod physics;
mod skybox;
mod util;

use character_controller::CharacterController;
use editor::GameEditorPlugin;
use light_shafts::{LightShaftsPlugin, SetLightShaftMaterial};
use pbr_material::{
    setup_curtains, swap_standard_material, CurtainSetBlend, CustomStandardMaterial,
};
use physics::{AddTrimeshPhysics, PhysicsStuff};

use bevy::{
    pbr::{CascadeShadowConfigBuilder, DirectionalLightShadowMap},
    prelude::*,
};
use skybox::SkyBoxPlugin;

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
        .add_plugin(GameEditorPlugin)
        .add_plugin(PhysicsStuff)
        .add_plugin(CharacterController)
        .add_plugin(SkyBoxPlugin)
        .add_plugin(LightShaftsPlugin)
        .add_startup_system(setup)
        .add_systems((swap_standard_material, setup_curtains))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
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
            maximum_distance: 100.0,
            ..default()
        }
        .into(),
        ..default()
    });
    commands
        .spawn(SceneBundle {
            scene: asset_server.load("../../temp_assets/CopyRoom.gltf#Scene0"),
            ..default()
        })
        .insert(AddTrimeshPhysics);
    commands
        .spawn(SceneBundle {
            scene: asset_server.load("../../temp_assets/curtain.gltf#Scene0"),
            ..default()
        })
        .insert(CurtainSetBlend);
    commands
        .spawn(SceneBundle {
            scene: asset_server.load("../../temp_assets/lightshafts.gltf#Scene0"),
            ..default()
        })
        .insert(SetLightShaftMaterial);
}
