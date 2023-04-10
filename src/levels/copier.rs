use bevy::{
    math::{vec2, vec3},
    prelude::*,
};
use bevy_fps_controller::controller::FpsController;

use crate::{
    assets::{LevelAssets, TextureAssets},
    light_shafts::{LightShaftsMaterial, SetLightShaftMaterial},
    materials::skybox::SkyBoxMaterial,
    pbr_material::EnvSettings,
    physics::AddTrimeshPhysics,
    ui::TextFeed,
};

#[derive(Component)]
pub struct CopierLevel;

pub fn despawn_copier(mut commands: Commands, query: Query<Entity, With<CopierLevel>>) {
    for entity in &query {
        if commands.get_entity(entity).is_some() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn spawn_copier(
    mut commands: Commands,
    level_assets: Res<LevelAssets>,
    mut materials: ResMut<Assets<SkyBoxMaterial>>,
    texture_assets: Res<TextureAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut fps_controller: Query<&mut FpsController>,
    mut text_feed: ResMut<TextFeed>,
) {
    text_feed.push("Whoops, you're in some random office building.");
    let mut fps_controller = fps_controller.get_single_mut().unwrap();
    fps_controller.gravity = crate::character_controller::GRAVITY;
    // SKYBOX
    commands
        .spawn(MaterialMeshBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1000.0 })),
            material: materials.add(SkyBoxMaterial {
                env_texture: Some(texture_assets.kloppenheim_05_puresky.clone()),
                uv_offset: vec2(0.0, 0.0),
                brightness: 1.0,
                contrast: 1.0,
            }),
            ..default()
        })
        .insert(CopierLevel);
    let env_settings = EnvSettings {
        env_spec: 0.2,
        env_diff: 0.2,
        emit_mult: 1.0,
    };
    commands
        .spawn(SceneBundle {
            scene: level_assets.copierroom_room.clone(),
            ..default()
        })
        .insert(AddTrimeshPhysics)
        .insert(env_settings)
        .insert(CopierLevel);
    commands
        .spawn(SceneBundle {
            scene: level_assets.copierroom_props.clone(),
            ..default()
        })
        .insert(AddTrimeshPhysics)
        .insert(env_settings)
        .insert(CopierLevel);
    commands
        .spawn(SceneBundle {
            scene: level_assets.copierroom_coordinatesclock.clone(),
            ..default()
        })
        .insert(env_settings)
        .insert(CopierLevel);
    commands
        .spawn(SceneBundle {
            scene: level_assets.copier_dust.clone(),
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
        .insert(CopierLevel);
}
