use bevy::{
    math::{vec2, vec3},
    prelude::*,
};
use bevy_fps_controller::controller::FpsController;

use crate::{
    assets::{LevelAssets, TextureAssets},
    light_shafts::{LightShaftsMaterial, SetLightShaftMaterial},
    materials::skybox::SkyBoxMaterial,
    pbr_material::{EnvSettings, SetGrassMaterial},
    physics::AddTrimeshPhysics,
    plant_material::{PlantsMaterial, SetPlantsMaterial},
    ui::TextFeed,
};

#[derive(Component)]
pub struct HousesLevel;

pub fn despawn_houses(mut commands: Commands, query: Query<Entity, With<HousesLevel>>) {
    for entity in &query {
        if commands.get_entity(entity).is_some() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn spawn_houses(
    mut commands: Commands,
    level_assets: Res<LevelAssets>,
    mut materials: ResMut<Assets<SkyBoxMaterial>>,
    texture_assets: Res<TextureAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut fps_controller: Query<&mut FpsController>,
    mut text_feed: ResMut<TextFeed>,
) {
    text_feed.push("");
    let mut fps_controller = fps_controller.get_single_mut().unwrap();
    fps_controller.gravity = crate::character_controller::GRAVITY;
    // SKYBOX
    commands
        .spawn(MaterialMeshBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1000.0 })),
            material: materials.add(SkyBoxMaterial {
                env_texture: Some(texture_assets.quarry_04_puresky.clone()),
                uv_offset: vec2(0.5, -0.06),
                brightness: 0.003,
                contrast: 1.0,
            }),
            ..default()
        })
        .insert(HousesLevel);
    let env_settings = EnvSettings {
        env_spec: 0.1,
        env_diff: 0.1,
        emit_mult: 1.0,
    };
    commands
        .spawn(SceneBundle {
            scene: level_assets.houses_clock_package.clone(),
            ..default()
        })
        .insert(AddTrimeshPhysics)
        .insert(env_settings)
        .insert(HousesLevel);
    commands
        .spawn(SceneBundle {
            scene: level_assets.houses_dust.clone(),
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
        .insert(HousesLevel);
    commands
        .spawn(SceneBundle {
            scene: level_assets.houses_grass3d.clone(),
            ..default()
        })
        .insert(AddTrimeshPhysics)
        .insert(env_settings)
        .insert(SetGrassMaterial)
        .insert(HousesLevel);
    commands
        .spawn(SceneBundle {
            scene: level_assets.houses_houses.clone(),
            ..default()
        })
        .insert(AddTrimeshPhysics)
        .insert(env_settings)
        .insert(HousesLevel);
    commands
        .spawn(SceneBundle {
            scene: level_assets.houses_houses2.clone(),
            ..default()
        })
        .insert(AddTrimeshPhysics)
        .insert(env_settings)
        .insert(HousesLevel);
    commands
        .spawn(SceneBundle {
            scene: level_assets.houses_props.clone(),
            ..default()
        })
        .insert(AddTrimeshPhysics)
        .insert(env_settings)
        .insert(HousesLevel);
    commands
        .spawn(SceneBundle {
            scene: level_assets.houses_structure.clone(),
            ..default()
        })
        .insert(AddTrimeshPhysics)
        .insert(env_settings)
        .insert(HousesLevel);
    commands
        .spawn(SceneBundle {
            scene: level_assets.houses_fake.clone(),
            ..default()
        })
        .insert(SetPlantsMaterial(PlantsMaterial {}))
        .insert(env_settings)
        .insert(HousesLevel);
    commands
        .spawn(SceneBundle {
            scene: level_assets.houses_landscape.clone(),
            ..default()
        })
        //.insert(SetPlantsMaterial(PlantsMaterial {}))
        .insert(env_settings)
        .insert(HousesLevel);
    commands
        .spawn(SceneBundle {
            scene: level_assets.houses_lights.clone(),
            ..default()
        })
        .insert(HousesLevel);

    //for x in 0..8 {
    //    for z in 0..8 {
    //        commands
    //            .spawn(SceneBundle {
    //                scene: level_assets.tree_burnt.clone(),
    //                transform: Transform::from_xyz(x as f32 * 2.0 - 20.0, 0.0, z as f32 * 2.0),
    //                ..default()
    //            })
    //            .insert(env_settings)
    //            .insert(HousesLevel)
    //            .insert(SetPlantsMaterial(PlantsMaterial {}));
    //    }
    //}
}
