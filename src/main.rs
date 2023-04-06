#![allow(clippy::too_many_arguments, clippy::type_complexity)]

mod assets;
mod character_controller;
mod editor;
mod light_shafts;
mod pbr_material;
mod physics;
mod skybox;
mod util;

use assets::{AssetProcPlugin, LevelAssets, TextureAssets};
use bevy_asset_loader::prelude::{LoadingState, LoadingStateAppExt};
use character_controller::CharacterController;
use editor::GameEditorPlugin;
use light_shafts::{LightShaftsPlugin, SetLightShaftMaterial};
use pbr_material::{
    setup_env_settings, setup_grass_mats, swap_standard_material, CustomStandardMaterial,
    SetGrassMaterial,
};
use physics::{AddTrimeshPhysics, PhysicsStuff};

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    math::vec3,
    pbr::{CascadeShadowConfigBuilder, DirectionalLightShadowMap},
    prelude::*,
    render::pipelined_rendering::PipelinedRenderingPlugin,
    window::PresentMode,
};
use skybox::SkyBoxPlugin;

use crate::{light_shafts::LightShaftsMaterial, pbr_material::EnvSettings};

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum GameLoading {
    #[default]
    AssetLoading,
    Loaded,
}

fn main() {
    App::new()
        .add_state::<GameLoading>()
        .add_loading_state(
            LoadingState::new(GameLoading::AssetLoading).continue_to_state(GameLoading::Loaded),
        )
        .add_collection_to_loading_state::<_, TextureAssets>(GameLoading::AssetLoading)
        .add_collection_to_loading_state::<_, LevelAssets>(GameLoading::AssetLoading)
        .insert_resource(AmbientLight {
            color: Color::BLACK,
            brightness: 0.0,
        })
        .insert_resource(DirectionalLightShadowMap { size: 4096 })
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    watch_for_changes: true,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        present_mode: PresentMode::AutoVsync,
                        ..default()
                    }),
                    ..default()
                }), //.disable::<PipelinedRenderingPlugin>(),
        )
        .add_plugin(MaterialPlugin::<CustomStandardMaterial>::default())
        //.add_plugin(GameEditorPlugin)
        .add_plugin(PhysicsStuff)
        .add_plugin(CharacterController)
        .add_plugin(SkyBoxPlugin)
        .add_plugin(LightShaftsPlugin)
        .add_plugin(AssetProcPlugin)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_system(spawn_houses.in_schedule(OnEnter(GameLoading::Loaded)))
        .add_systems((
            swap_standard_material.run_if(in_state(GameLoading::Loaded)),
            setup_grass_mats.run_if(in_state(GameLoading::Loaded)),
            setup_env_settings.run_if(in_state(GameLoading::Loaded)),
        ))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, level_assets: Res<LevelAssets>) {
    //commands.spawn(DirectionalLightBundle {
    //    directional_light: DirectionalLight {
    //        shadows_enabled: true,
    //        ..default()
    //    },
    //    // This is a relatively small scene, so use tighter shadow
    //    // cascade bounds than the default for better quality.
    //    // We also adjusted the shadow map to be larger since we're
    //    // only using a single cascade.
    //    cascade_shadow_config: CascadeShadowConfigBuilder {
    //        num_cascades: 1,
    //        maximum_distance: 100.0,
    //        ..default()
    //    }
    //    .into(),
    //    ..default()
    //});

    //    commands
    //        .spawn(SceneBundle {
    //            scene: asset_server.load("levels/kitchen/ExpKitchen.blend_Room.gltf#Scene0"),
    //            ..default()
    //        })
    //        .insert(AddTrimeshPhysics);
    //    commands
    //        .spawn(SceneBundle {
    //            scene: asset_server.load("levels/kitchen/ExpKitchen.blend_Props.gltf#Scene0"),
    //            ..default()
    //        })
    //        .insert(AddTrimeshPhysics);
    //    commands
    //        .spawn(SceneBundle {
    //            scene: asset_server.load("levels/kitchen/ExpKitchen.blend_Ceiling.gltf#Scene0"),
    //            ..default()
    //        })
    //        .insert(AddTrimeshPhysics);
    //    commands.spawn(SceneBundle {
    //        scene: asset_server.load("levels/kitchen/ExpKitchen.blend_Wallpaper_Trim.gltf#Scene0"),
    //        ..default()
    //    });
    //    commands.spawn(SceneBundle {
    //        scene: asset_server.load("levels/kitchen/ExpKitchen.blend_Curtains.gltf#Scene0"),
    //        ..default()
    //    });
    //    commands
    //        .spawn(SceneBundle {
    //            scene: asset_server.load("levels/kitchen/ExpKitchen.Dust.gltf#Scene0"),
    //            ..default()
    //        })
    //        .insert(SetLightShaftMaterial);

    //--------------------
    //--------------------
    //--------------------

    //commands
    //    .spawn(SceneBundle {
    //        scene: asset_server
    //            .load("../../temp_assets/Bathroom/ExpBathroom.blend_Structure.gltf#Scene0"),
    //        ..default()
    //    })
    //    .insert(AddTrimeshPhysics);
    //commands
    //    .spawn(SceneBundle {
    //        scene: asset_server
    //            .load("../../temp_assets/Bathroom/ExpBathroom.blend_Props.gltf#Scene0"),
    //        ..default()
    //    })
    //    .insert(AddTrimeshPhysics);
    //commands
    //    .spawn(SceneBundle {
    //        scene: asset_server
    //            .load("../../temp_assets/Bathroom/ExpBathroom.blend_Dust.gltf#Scene0"),
    //        ..default()
    //    })
    //    .insert(SetLightShaftMaterial);

    //commands
    //    .spawn(SceneBundle {
    //        scene: asset_server.load("../../temp_assets/grass.gltf#Scene0"),
    //        ..default()
    //    })
    //    .insert(SetGrassMaterial)
    //    .insert(AddTrimeshPhysics);
}

fn spawn_urban(mut commands: Commands, level_assets: Res<LevelAssets>) {
    let env_settings = EnvSettings {
        env_spec: 0.1,
        env_diff: 0.1,
    };
    commands
        .spawn(SceneBundle {
            scene: level_assets.urban_far_away_buildings.clone(),
            ..default()
        })
        .insert(env_settings);
    commands
        .spawn(SceneBundle {
            scene: level_assets.urban_props.clone(),
            ..default()
        })
        .insert(AddTrimeshPhysics)
        .insert(env_settings);
    commands
        .spawn(SceneBundle {
            scene: level_assets.urban_structure.clone(),
            ..default()
        })
        .insert(AddTrimeshPhysics)
        .insert(env_settings);
    commands
        .spawn(SceneBundle {
            scene: level_assets.urban_surrounding_buildings.clone(),
            ..default()
        })
        .insert(AddTrimeshPhysics)
        .insert(env_settings);
    commands
        .spawn(SceneBundle {
            scene: level_assets.urban_dust.clone(),
            ..default()
        })
        .insert(SetLightShaftMaterial(LightShaftsMaterial {
            color: vec3(0.28, 0.25, 0.15),
            shaft: 0.3,
            dust: 0.8,
            dust_size: 2.5,
            dust_qty_sub: 0.03,
        }));
}

fn spawn_houses(mut commands: Commands, level_assets: Res<LevelAssets>) {
    let env_settings = EnvSettings {
        env_spec: 0.1,
        env_diff: 0.1,
    };
    commands
        .spawn(SceneBundle {
            scene: level_assets.houses_clock_package.clone(),
            ..default()
        })
        .insert(AddTrimeshPhysics)
        .insert(env_settings);
    commands
        .spawn(SceneBundle {
            scene: level_assets.houses_dust.clone(),
            ..default()
        })
        .insert(SetLightShaftMaterial(LightShaftsMaterial {
            color: vec3(0.28, 0.25, 0.15),
            shaft: 0.3,
            dust: 0.8,
            dust_size: 2.5,
            dust_qty_sub: 0.03,
        }));
    commands
        .spawn(SceneBundle {
            scene: level_assets.houses_grass3d.clone(),
            ..default()
        })
        .insert(AddTrimeshPhysics)
        .insert(env_settings)
        .insert(SetGrassMaterial);
    commands
        .spawn(SceneBundle {
            scene: level_assets.houses_houses.clone(),
            ..default()
        })
        .insert(AddTrimeshPhysics)
        .insert(env_settings);
    commands
        .spawn(SceneBundle {
            scene: level_assets.houses_houses2.clone(),
            ..default()
        })
        .insert(AddTrimeshPhysics)
        .insert(env_settings);
    commands
        .spawn(SceneBundle {
            scene: level_assets.houses_houses2.clone(),
            ..default()
        })
        .insert(AddTrimeshPhysics)
        .insert(env_settings);
    commands
        .spawn(SceneBundle {
            scene: level_assets.houses_props.clone(),
            ..default()
        })
        .insert(AddTrimeshPhysics)
        .insert(env_settings);
    commands
        .spawn(SceneBundle {
            scene: level_assets.houses_structure.clone(),
            ..default()
        })
        .insert(AddTrimeshPhysics)
        .insert(env_settings);
}
