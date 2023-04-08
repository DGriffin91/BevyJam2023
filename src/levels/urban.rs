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
pub struct UrbanLevel;

pub fn spawn_player_urban(mut query: Query<(&mut Transform, &mut Velocity), With<FpsController>>) {
    for (mut transform, mut velocity) in &mut query {
        velocity.linvel = Vec3::ZERO;
        transform.translation = vec3(0.0, 0.0, 0.0);
    }
}

pub fn despawn_urban(mut commands: Commands, query: Query<Entity, With<UrbanLevel>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn spawn_urban(
    mut commands: Commands,
    level_assets: Res<LevelAssets>,
    mut materials: ResMut<Assets<SkyBoxMaterial>>,
    texture_assets: Res<TextureAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    // SKYBOX
    commands
        .spawn(MaterialMeshBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1000.0 })),
            material: materials.add(SkyBoxMaterial {
                env_texture: Some(texture_assets.quarry_04_puresky.clone()),
                uv_offset: vec2(0.0, 0.0),
                brightness: 0.008,
                contrast: 2.0,
            }),
            ..default()
        })
        .insert(UrbanLevel);
    let env_settings = EnvSettings {
        env_spec: 0.1,
        env_diff: 0.1,
        emit_mult: 1.0,
    };
    commands
        .spawn(SceneBundle {
            scene: level_assets.urban_far_away_buildings.clone(),
            ..default()
        })
        .insert(env_settings)
        .insert(UrbanLevel);
    commands
        .spawn(SceneBundle {
            scene: level_assets.urban_props.clone(),
            ..default()
        })
        .insert(AddTrimeshPhysics)
        .insert(env_settings)
        .insert(UrbanLevel);
    commands
        .spawn(SceneBundle {
            scene: level_assets.urban_structure.clone(),
            ..default()
        })
        .insert(AddTrimeshPhysics)
        .insert(env_settings)
        .insert(UrbanLevel);
    commands
        .spawn(SceneBundle {
            scene: level_assets.urban_surrounding_buildings.clone(),
            ..default()
        })
        .insert(AddTrimeshPhysics)
        .insert(env_settings)
        .insert(UrbanLevel);
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
            dust_speed: 1.0,
        }))
        .insert(UrbanLevel);
}
