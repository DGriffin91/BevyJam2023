use bevy::{math::vec2, prelude::*};
use bevy_fps_controller::controller::FpsController;

use crate::{
    assets::{LevelAssets, TextureAssets},
    materials::{pbr_material::SetGrassMaterial2, skybox::SkyBoxMaterial},
    pbr_material::EnvSettings,
    physics::AddTrimeshPhysics,
    ui::TextFeed,
};

#[derive(Component)]
pub struct BFStartLevel;

pub fn despawn_bfstart(mut commands: Commands, query: Query<Entity, With<BFStartLevel>>) {
    for entity in &query {
        if commands.get_entity(entity).is_some() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn spawn_bfstart(
    mut commands: Commands,
    level_assets: Res<LevelAssets>,
    mut materials: ResMut<Assets<SkyBoxMaterial>>,
    texture_assets: Res<TextureAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut fps_controller: Query<&mut FpsController>,
    mut text_feed: ResMut<TextFeed>,
) {
    text_feed.push("Hey, we need you to infiltrate the facility and quickly eliminate the security drones in each sector. Take the teleporters from one sector to the next to get to the control room …an unfortunate side effect is that they might transport you to the wrong coordinates.");
    let mut fps_controller = fps_controller.get_single_mut().unwrap();
    fps_controller.gravity = crate::character_controller::GRAVITY;
    // SKYBOX
    commands
        .spawn(MaterialMeshBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1000.0 })),
            material: materials.add(SkyBoxMaterial {
                env_texture: Some(texture_assets.belfast_sunset_puresky.clone()),
                uv_offset: vec2(0.0, 0.0),
                brightness: 0.1,
                contrast: 1.8,
            }),
            ..default()
        })
        .insert(BFStartLevel);
    let env_settings = EnvSettings {
        env_spec: 0.1,
        env_diff: 0.1,
        emit_mult: 1.0,
    };
    commands
        .spawn(SceneBundle {
            scene: level_assets.bf_start_building.clone(),
            ..default()
        })
        .insert(AddTrimeshPhysics)
        .insert(env_settings)
        .insert(BFStartLevel);
    commands
        .spawn(SceneBundle {
            scene: level_assets.bf_start_grass.clone(),
            ..default()
        })
        .insert(AddTrimeshPhysics)
        .insert(env_settings)
        .insert(BFStartLevel)
        .insert(SetGrassMaterial2);
    commands
        .spawn(SceneBundle {
            scene: level_assets.bf_start_rocks.clone(),
            ..default()
        })
        .insert(AddTrimeshPhysics)
        .insert(env_settings)
        .insert(BFStartLevel);
}
