use bevy::{math::vec3, prelude::*};
use bevy_fps_controller::controller::FpsController;

use crate::{
    assets::LevelAssets,
    light_shafts::{LightShaftsMaterial, SetLightShaftMaterial},
    pbr_material::EnvSettings,
    physics::AddTrimeshPhysics,
    ui::TextFeed,
};

#[derive(Component)]
pub struct ShowerLevel;

pub fn despawn_shower(mut commands: Commands, query: Query<Entity, With<ShowerLevel>>) {
    for entity in &query {
        if commands.get_entity(entity).is_some() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn spawn_shower(
    mut commands: Commands,
    level_assets: Res<LevelAssets>,
    //mut materials: ResMut<Assets<SkyBoxMaterial>>,
    //texture_assets: Res<TextureAssets>,
    //mut meshes: ResMut<Assets<Mesh>>,
    mut fps_controller: Query<&mut FpsController>,
    mut text_feed: ResMut<TextFeed>,
) {
    text_feed.push(
        "Oops, that’s…a shower. At least you’re clean. Hurry up and get back to the facility.",
    );
    let mut fps_controller = fps_controller.get_single_mut().unwrap();
    fps_controller.gravity = crate::character_controller::GRAVITY;
    let env_settings = EnvSettings {
        env_spec: 0.1,
        env_diff: 0.1,
        emit_mult: 1.0,
    };
    commands
        .spawn(SceneBundle {
            scene: level_assets.shower_props.clone(),
            ..default()
        })
        .insert(AddTrimeshPhysics)
        .insert(env_settings)
        .insert(ShowerLevel);
    commands
        .spawn(SceneBundle {
            scene: level_assets.shower_structure.clone(),
            ..default()
        })
        .insert(AddTrimeshPhysics)
        .insert(env_settings)
        .insert(ShowerLevel);
    commands
        .spawn(SceneBundle {
            scene: level_assets.shower_clock.clone(),
            ..default()
        })
        .insert(env_settings)
        .insert(ShowerLevel);
    commands
        .spawn(SceneBundle {
            scene: level_assets.shower_dust.clone(),
            ..default()
        })
        .insert(SetLightShaftMaterial(LightShaftsMaterial {
            color: vec3(0.9, 0.8, 0.5),
            shaft: 2.0,
            dust: 2.0,
            dust_size: 1.2,
            dust_qty_sub: -0.05,
            dust_speed: 100.0,
        }))
        .insert(ShowerLevel);
}
