use bevy::prelude::*;
use bevy_fps_controller::controller::FpsController;

use crate::{
    assets::LevelAssets,
    pbr_material::EnvSettings,
    physics::AddTrimeshPhysics,
    ui::{HasEnteredControlRoom, TextFeed},
};

#[derive(Component)]
pub struct ControlRoomLevel;

pub fn despawn_control_room(mut commands: Commands, query: Query<Entity, With<ControlRoomLevel>>) {
    for entity in &query {
        if commands.get_entity(entity).is_some() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn spawn_control_room(
    mut commands: Commands,
    level_assets: Res<LevelAssets>,
    //mut materials: ResMut<Assets<SkyBoxMaterial>>,
    //texture_assets: Res<TextureAssets>,
    //mut meshes: ResMut<Assets<Mesh>>,
    mut fps_controller: Query<&mut FpsController>,
    mut text_feed: ResMut<TextFeed>,
    mut has_entered_control_room: ResMut<HasEnteredControlRoom>,
) {
    has_entered_control_room.0 = true;
    text_feed
        .push("Nice. Quick, jump into the control system core to disrupt the security network.");
    let mut fps_controller = fps_controller.get_single_mut().unwrap();
    fps_controller.gravity = crate::character_controller::GRAVITY;
    let env_settings = EnvSettings {
        env_spec: 0.2,
        env_diff: 0.2,
        emit_mult: 1.0,
    };
    commands
        .spawn(SceneBundle {
            scene: level_assets.controlroom_counter.clone(),
            ..default()
        })
        .insert(AddTrimeshPhysics)
        .insert(env_settings)
        .insert(ControlRoomLevel);
    commands
        .spawn(SceneBundle {
            scene: level_assets.controlroom_props.clone(),
            ..default()
        })
        .insert(AddTrimeshPhysics)
        .insert(env_settings)
        .insert(ControlRoomLevel);
    commands
        .spawn(SceneBundle {
            scene: level_assets.controlroom_structure.clone(),
            ..default()
        })
        .insert(AddTrimeshPhysics)
        .insert(env_settings)
        .insert(ControlRoomLevel);
}
