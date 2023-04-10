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
    ui::{FinishedGame, GameElapsedTime, HasEnteredControlRoom, TextFeed},
};

#[derive(Component)]
pub struct KitchenLevel;

pub fn despawn_kitchen(mut commands: Commands, query: Query<Entity, With<KitchenLevel>>) {
    for entity in &query {
        if commands.get_entity(entity).is_some() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn spawn_kitchen(
    mut commands: Commands,
    level_assets: Res<LevelAssets>,
    mut materials: ResMut<Assets<SkyBoxMaterial>>,
    texture_assets: Res<TextureAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut fps_controller: Query<&mut FpsController>,
    mut text_feed: ResMut<TextFeed>,
    mut game_time: ResMut<GameElapsedTime>,
    time: Res<Time>,
    has_entered_control_room: Res<HasEnteredControlRoom>,
    mut finished_game: ResMut<FinishedGame>,
) {
    if game_time.0.is_none() {
        game_time.0 = Some(time.elapsed_seconds());
    }
    if has_entered_control_room.0 {
        finished_game.0 .0 = true;
        finished_game.0 .1 = time.elapsed_seconds();
        text_feed.push("You did it! Nice work.");
    } else {
        //text_feed.push("Escape the kitchen");
    }
    let mut fps_controller = fps_controller.get_single_mut().unwrap();
    fps_controller.gravity = crate::character_controller::GRAVITY;
    // SKYBOX
    commands
        .spawn(MaterialMeshBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1000.0 })),
            material: materials.add(SkyBoxMaterial {
                env_texture: Some(texture_assets.hilly_terrain_01_puresky.clone()),
                uv_offset: vec2(0.3, 0.0),
                brightness: 1.0,
                contrast: 1.0,
            }),
            ..default()
        })
        .insert(KitchenLevel);
    let env_settings = EnvSettings {
        env_spec: 0.1,
        env_diff: 0.1,
        emit_mult: 1.0,
    };
    commands
        .spawn(SceneBundle {
            scene: level_assets.kitchen_curtains.clone(),
            ..default()
        })
        .insert(KitchenLevel);
    commands
        .spawn(SceneBundle {
            scene: level_assets.kitchen_dust.clone(),
            ..default()
        })
        .insert(SetLightShaftMaterial(LightShaftsMaterial {
            color: vec3(0.9, 0.8, 0.5),
            shaft: 1.0,
            dust: 1.0,
            dust_size: 1.0,
            dust_qty_sub: 0.0,
            dust_speed: 1.0,
        }))
        .insert(KitchenLevel);
    commands
        .spawn(SceneBundle {
            scene: level_assets.kitchen_props.clone(),
            ..default()
        })
        .insert(AddTrimeshPhysics)
        .insert(env_settings)
        .insert(KitchenLevel);
    commands
        .spawn(SceneBundle {
            scene: level_assets.kitchen_room.clone(),
            ..default()
        })
        .insert(AddTrimeshPhysics)
        .insert(env_settings)
        .insert(KitchenLevel);
    commands
        .spawn(SceneBundle {
            scene: level_assets.kitchen_stovetopclock.clone(),
            ..default()
        })
        .insert(AddTrimeshPhysics)
        .insert(env_settings)
        .insert(KitchenLevel);
    commands
        .spawn(SceneBundle {
            scene: level_assets.kitchen_wallpaper_trim.clone(),
            ..default()
        })
        .insert(env_settings)
        .insert(KitchenLevel);
}
