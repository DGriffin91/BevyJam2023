use bevy::{
    math::{vec2, vec3},
    prelude::*,
};
use bevy_fps_controller::controller::FpsController;
use bevy_rapier3d::prelude::Velocity;

use crate::{
    assets::{LevelAssets, TextureAssets},
    light_shafts::{LightShaftsMaterial, SetLightShaftMaterial},
    materials::skybox::SkyBoxMaterial,
    pbr_material::EnvSettings,
    physics::AddTrimeshPhysics,
};

#[derive(Component)]
pub struct ShowerLevel;

pub fn spawn_player_shower(mut query: Query<(&mut Transform, &mut Velocity), With<FpsController>>) {
    for (mut transform, mut velocity) in &mut query {
        velocity.linvel = Vec3::ZERO;
        transform.translation = vec3(0.0, 0.0, 0.0);
    }
}

pub fn despawn_shower(mut commands: Commands, query: Query<Entity, With<ShowerLevel>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn spawn_shower(
    mut commands: Commands,
    level_assets: Res<LevelAssets>,
    mut materials: ResMut<Assets<SkyBoxMaterial>>,
    texture_assets: Res<TextureAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
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
