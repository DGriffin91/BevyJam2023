use bevy::{math::vec2, prelude::*};
use bevy_fps_controller::controller::FpsController;

use crate::{
    assets::{LevelAssets, TextureAssets},
    materials::skybox::SkyBoxMaterial,
    pbr_material::EnvSettings,
    physics::AddTrimeshPhysics,
    ui::TextFeed,
    units::EnemySpawns,
};

#[derive(Component)]
pub struct BF1Level;

pub fn despawn_bf1(mut commands: Commands, query: Query<Entity, With<BF1Level>>) {
    for entity in &query {
        if commands.get_entity(entity).is_some() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn spawn_bf1(
    mut commands: Commands,
    level_assets: Res<LevelAssets>,
    mut materials: ResMut<Assets<SkyBoxMaterial>>,
    texture_assets: Res<TextureAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut fps_controller: Query<&mut FpsController>,
    mut text_feed: ResMut<TextFeed>,
) {
    text_feed.push("We need you to eliminate the security drones in each sector as fast as you can. Teleporters are setup to bring you from one sector to the next... an unfortunate side effect is that they sometimes teleport you to the wrong coordinates.");
    let mut fps_controller = fps_controller.get_single_mut().unwrap();
    fps_controller.gravity = crate::character_controller::GRAVITY;
    // SKYBOX
    commands
        .spawn(MaterialMeshBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 10000.0 })),
            material: materials.add(SkyBoxMaterial {
                env_texture: Some(texture_assets.belfast_sunset_puresky.clone()),
                uv_offset: vec2(0.0, 0.0),
                brightness: 0.1,
                contrast: 1.8,
            }),
            ..default()
        })
        .insert(BF1Level);
    let env_settings = EnvSettings {
        env_spec: 0.5,
        env_diff: 0.5,
        emit_mult: 1.0,
    };
    commands
        .spawn(SceneBundle {
            scene: level_assets.bf1_start.clone(),
            ..default()
        })
        .insert(AddTrimeshPhysics)
        .insert(env_settings)
        .insert(BF1Level);
    commands
        .spawn(SceneBundle {
            scene: level_assets.bf1_mid.clone(),
            ..default()
        })
        .insert(AddTrimeshPhysics)
        .insert(env_settings)
        .insert(BF1Level);
    commands
        .spawn(SceneBundle {
            scene: level_assets.bf1_down.clone(),
            ..default()
        })
        .insert(AddTrimeshPhysics)
        .insert(env_settings)
        .insert(BF1Level);
    commands
        .spawn(SceneBundle {
            scene: level_assets.bf1_blinds.clone(),
            ..default()
        })
        .insert(env_settings)
        .insert(BF1Level);
    commands
        .spawn(SceneBundle {
            scene: level_assets.bf1_lights.clone(),
            ..default()
        })
        .insert(BF1Level);
    commands
        .spawn(SceneBundle {
            scene: level_assets.bf1_enemy_spawns.clone(),
            ..default()
        })
        .insert(BF1Level)
        .insert(EnemySpawns);
}
