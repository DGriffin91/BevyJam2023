use bevy::prelude::*;
use bevy_fps_controller::controller::FpsController;

use crate::{
    assets::LevelAssets, pbr_material::EnvSettings, physics::AddTrimeshPhysics, ui::TextFeed,
    units::EnemySpawns,
};

#[derive(Component)]
pub struct BFA1Level;

pub fn despawn_bfa1(mut commands: Commands, query: Query<Entity, With<BFA1Level>>) {
    for entity in &query {
        if commands.get_entity(entity).is_some() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn spawn_bfa1(
    mut commands: Commands,
    level_assets: Res<LevelAssets>,
    //mut materials: ResMut<Assets<SkyBoxMaterial>>,
    //texture_assets: Res<TextureAssets>,
    //mut meshes: ResMut<Assets<Mesh>>,
    mut fps_controller: Query<&mut FpsController>,
    mut text_feed: ResMut<TextFeed>,
) {
    text_feed.push("Alright, take out those drones to unlock the teleporter.");
    let mut fps_controller = fps_controller.get_single_mut().unwrap();
    fps_controller.gravity = crate::character_controller::GRAVITY;
    let env_settings = EnvSettings {
        env_spec: 0.5,
        env_diff: 0.5,
        emit_mult: 1.0,
    };
    commands
        .spawn(SceneBundle {
            scene: level_assets.bfa_bfa1.clone(),
            ..default()
        })
        .insert(AddTrimeshPhysics)
        .insert(env_settings)
        .insert(BFA1Level);
    commands
        .spawn(SceneBundle {
            scene: level_assets.bfa1_enemy_spawns.clone(),
            ..default()
        })
        .insert(BFA1Level)
        .insert(EnemySpawns);
}

// --------------------------------------
// --------------------------------------
// --------------------------------------

#[derive(Component)]
pub struct BFA2Level;

pub fn despawn_bfa2(mut commands: Commands, query: Query<Entity, With<BFA2Level>>) {
    for entity in &query {
        if commands.get_entity(entity).is_some() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn spawn_bfa2(
    mut commands: Commands,
    level_assets: Res<LevelAssets>,
    //mut materials: ResMut<Assets<SkyBoxMaterial>>,
    //texture_assets: Res<TextureAssets>,
    //mut meshes: ResMut<Assets<Mesh>>,
    mut fps_controller: Query<&mut FpsController>,
    mut text_feed: ResMut<TextFeed>,
) {
    text_feed.push("Back to business.");
    let mut fps_controller = fps_controller.get_single_mut().unwrap();
    fps_controller.gravity = crate::character_controller::GRAVITY;
    let env_settings = EnvSettings {
        env_spec: 0.5,
        env_diff: 0.5,
        emit_mult: 1.0,
    };
    commands
        .spawn(SceneBundle {
            scene: level_assets.bfa_bfa2.clone(),
            ..default()
        })
        .insert(AddTrimeshPhysics)
        .insert(env_settings)
        .insert(BFA2Level);
    commands
        .spawn(SceneBundle {
            scene: level_assets.bfa2_enemy_spawns.clone(),
            ..default()
        })
        .insert(BFA2Level)
        .insert(EnemySpawns);
}

// --------------------------------------
// --------------------------------------
// --------------------------------------

#[derive(Component)]
pub struct BFA3Level;

pub fn despawn_bfa3(mut commands: Commands, query: Query<Entity, With<BFA3Level>>) {
    for entity in &query {
        if commands.get_entity(entity).is_some() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn spawn_bfa3(
    mut commands: Commands,
    level_assets: Res<LevelAssets>,
    //mut materials: ResMut<Assets<SkyBoxMaterial>>,
    //texture_assets: Res<TextureAssets>,
    //mut meshes: ResMut<Assets<Mesh>>,
    mut fps_controller: Query<&mut FpsController>,
    mut text_feed: ResMut<TextFeed>,
) {
    text_feed.push("");
    let mut fps_controller = fps_controller.get_single_mut().unwrap();
    fps_controller.gravity = crate::character_controller::GRAVITY;
    let env_settings = EnvSettings {
        env_spec: 0.5,
        env_diff: 0.5,
        emit_mult: 1.0,
    };
    commands
        .spawn(SceneBundle {
            scene: level_assets.bfa_bfa3.clone(),
            ..default()
        })
        .insert(AddTrimeshPhysics)
        .insert(env_settings)
        .insert(BFA3Level);
    commands
        .spawn(SceneBundle {
            scene: level_assets.bfa3_enemy_spawns.clone(),
            ..default()
        })
        .insert(BFA3Level)
        .insert(EnemySpawns);
}
