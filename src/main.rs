#![allow(clippy::too_many_arguments, clippy::type_complexity)]

mod assets;
mod character_controller;
mod editor;
mod levels;
mod materials;
mod physics;
mod player;
mod ui;
mod units;
mod util;

use assets::{AssetProcPlugin, LevelAssets, PropAssets, TextureAssets, UnitAssets};
use bevy_asset_loader::prelude::{LoadingState, LoadingStateAppExt};

use bevy_polyline::PolylinePlugin;
use character_controller::CharacterController;

use levels::{GameLevel, LevelsPlugin};
use light_shafts::LightShaftsPlugin;
use materials::{light_shafts, pbr_material, plant_material};
use pbr_material::{
    setup_env_settings, setup_grass_mats, swap_standard_material, CustomStandardMaterial,
};
use physics::PhysicsStuff;

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    pbr::DirectionalLightShadowMap,
    prelude::*,
    window::PresentMode,
};
use materials::skybox::SkyBoxPlugin;
use plant_material::PlantsPlugin;
use player::PlayerPlugin;
use rand_pcg::Pcg32;
use ui::GameUiPlugin;
use units::UnitsPlugin;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum GameLoading {
    #[default]
    AssetLoading,
    Loaded,
}

#[derive(Resource, Deref, DerefMut)]
pub struct GameRng(pub Pcg32);

impl Default for GameRng {
    fn default() -> Self {
        GameRng(Pcg32::new(0xcafef00dd15ea5e5, 0xa02bdbf7bb3c0a7))
    }
}

#[derive(Component)]
pub struct Health(f32);

fn main() {
    App::new()
        .insert_resource(GameRng::default())
        .add_state::<GameLoading>()
        .add_state::<GameLevel>()
        .insert_resource(Msaa::Off)
        .add_loading_state(
            LoadingState::new(GameLoading::AssetLoading).continue_to_state(GameLoading::Loaded),
        )
        .add_collection_to_loading_state::<_, TextureAssets>(GameLoading::AssetLoading)
        .add_collection_to_loading_state::<_, LevelAssets>(GameLoading::AssetLoading)
        .add_collection_to_loading_state::<_, UnitAssets>(GameLoading::AssetLoading)
        .add_collection_to_loading_state::<_, PropAssets>(GameLoading::AssetLoading)
        .insert_resource(AmbientLight {
            color: Color::BLACK,
            brightness: 0.0,
        })
        .insert_resource(DirectionalLightShadowMap { size: 1024 })
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
        .add_plugin(PlantsPlugin)
        .add_plugin(AssetProcPlugin)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(LevelsPlugin)
        .add_plugin(GameUiPlugin)
        .add_plugin(UnitsPlugin)
        .add_plugin(PolylinePlugin)
        .add_plugin(PlayerPlugin)
        .add_systems((
            swap_standard_material.run_if(in_state(GameLoading::Loaded)),
            setup_grass_mats.run_if(in_state(GameLoading::Loaded)),
            setup_env_settings.run_if(in_state(GameLoading::Loaded)),
        ))
        .run();
}
