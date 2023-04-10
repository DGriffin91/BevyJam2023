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
pub struct BathroomLevel;

pub fn despawn_bathroom(mut commands: Commands, query: Query<Entity, With<BathroomLevel>>) {
    for entity in &query {
        if commands.get_entity(entity).is_some() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn spawn_bathroom(
    mut commands: Commands,
    level_assets: Res<LevelAssets>,
    //mut materials: ResMut<Assets<SkyBoxMaterial>>,
    //texture_assets: Res<TextureAssets>,
    //mut meshes: ResMut<Assets<Mesh>>,
    mut fps_controller: Query<&mut FpsController>,
    mut text_feed: ResMut<TextFeed>,
) {
    text_feed.push("Ugh.");
    let mut fps_controller = fps_controller.get_single_mut().unwrap();
    fps_controller.gravity = crate::character_controller::GRAVITY;
    let env_settings = EnvSettings {
        env_spec: 0.2,
        env_diff: 0.2,
        emit_mult: 1.0,
    };
    commands
        .spawn(SceneBundle {
            scene: level_assets.bathroom_clockcoords.clone(),
            ..default()
        })
        .insert(env_settings)
        .insert(BathroomLevel);
    commands
        .spawn(SceneBundle {
            scene: level_assets.bathroom_props.clone(),
            ..default()
        })
        .insert(AddTrimeshPhysics)
        .insert(env_settings)
        .insert(BathroomLevel);
    commands
        .spawn(SceneBundle {
            scene: level_assets.bathroom_structure.clone(),
            ..default()
        })
        .insert(AddTrimeshPhysics)
        .insert(env_settings)
        .insert(BathroomLevel);
    commands
        .spawn(SceneBundle {
            scene: level_assets.bathroom_dust.clone(),
            ..default()
        })
        .insert(SetLightShaftMaterial(LightShaftsMaterial {
            color: vec3(1.0, 1.0, 1.0),
            shaft: 1.0,
            dust: 1.0,
            dust_size: 1.0,
            dust_qty_sub: 0.0,
            dust_speed: 1.0,
        }))
        .insert(BathroomLevel);
}
