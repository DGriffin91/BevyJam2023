use std::num::NonZeroU8;

use bevy::{
    prelude::*,
    render::{
        render_resource::{AddressMode, FilterMode, SamplerDescriptor},
        texture::ImageSampler,
    },
};
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::AudioSource;

use crate::GameLoading;

#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
    #[asset(path = "textures/detail.ktx2")]
    pub detail: Handle<Image>,
    #[asset(path = "environment_maps/quarry_04_puresky_2k.ktx2")]
    pub quarry_04_puresky: Handle<Image>,
    #[asset(path = "environment_maps/kloppenheim_05_puresky_2k.ktx2")]
    pub kloppenheim_05_puresky: Handle<Image>,
    #[asset(path = "environment_maps/hilly_terrain_01_puresky_2k.ktx2")]
    pub hilly_terrain_01_puresky: Handle<Image>,
    #[asset(path = "environment_maps/belfast_sunset_puresky_2k.ktx2")]
    pub belfast_sunset_puresky: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct LevelAssets {
    // URBAN
    #[asset(path = "levels/urban/expurban_farawaybuildings.gltf#Scene0")]
    pub urban_far_away_buildings: Handle<Scene>,
    #[asset(path = "levels/urban/expurban_props.gltf#Scene0")]
    pub urban_props: Handle<Scene>,
    #[asset(path = "levels/urban/expurban_structure.gltf#Scene0")]
    pub urban_structure: Handle<Scene>,
    #[asset(path = "levels/urban/expurban_surroundingbuildings.gltf#Scene0")]
    pub urban_surrounding_buildings: Handle<Scene>,
    #[asset(path = "levels/urban/urban_dust.gltf#Scene0")]
    pub urban_dust: Handle<Scene>,

    // HOUSES
    #[asset(path = "levels/houses/exphouses_clockpackage.gltf#Scene0")]
    pub houses_clock_package: Handle<Scene>,
    #[asset(path = "levels/houses/exphouses_grass3d.gltf#Scene0")]
    pub houses_grass3d: Handle<Scene>,
    #[asset(path = "levels/houses/exphouses_houses.gltf#Scene0")]
    pub houses_houses: Handle<Scene>,
    #[asset(path = "levels/houses/exphouses_houses2.gltf#Scene0")]
    pub houses_houses2: Handle<Scene>,
    #[asset(path = "levels/houses/exphouses_props.gltf#Scene0")]
    pub houses_props: Handle<Scene>,
    #[asset(path = "levels/houses/exphouses_structure.gltf#Scene0")]
    pub houses_structure: Handle<Scene>,
    #[asset(path = "levels/houses/houses_dust.gltf#Scene0")]
    pub houses_dust: Handle<Scene>,
    #[asset(path = "levels/houses/houses_landscape.gltf#Scene0")]
    pub houses_landscape: Handle<Scene>,
    #[asset(path = "levels/houses/houses_fake.gltf#Scene0")]
    pub houses_fake: Handle<Scene>,
    #[asset(path = "levels/houses/houses_lights.gltf#Scene0")]
    pub houses_lights: Handle<Scene>,

    // KITCHEN
    #[asset(path = "levels/kitchen/ExpKitchen.blend_Curtains.gltf#Scene0")]
    pub kitchen_curtains: Handle<Scene>,
    #[asset(path = "levels/kitchen/expkitchen_props.gltf#Scene0")]
    pub kitchen_props: Handle<Scene>,
    #[asset(path = "levels/kitchen/expkitchen_room.gltf#Scene0")]
    pub kitchen_room: Handle<Scene>,
    #[asset(path = "levels/kitchen/expkitchen_stovetopclock.gltf#Scene0")]
    pub kitchen_stovetopclock: Handle<Scene>,
    #[asset(path = "levels/kitchen/expkitchen_wallpaper_trim.gltf#Scene0")]
    pub kitchen_wallpaper_trim: Handle<Scene>,
    #[asset(path = "levels/kitchen/ExpKitchen.Dust.gltf#Scene0")]
    pub kitchen_dust: Handle<Scene>,

    // SHOWER
    #[asset(path = "levels/shower/expshower_props.gltf#Scene0")]
    pub shower_props: Handle<Scene>,
    #[asset(path = "levels/shower/expshower_structure.gltf#Scene0")]
    pub shower_structure: Handle<Scene>,
    #[asset(path = "levels/shower/expshower_clock.gltf#Scene0")]
    pub shower_clock: Handle<Scene>,
    #[asset(path = "levels/shower/shower_dust.gltf#Scene0")]
    pub shower_dust: Handle<Scene>,

    // BATHROOM
    #[asset(path = "levels/bathroom/expbathroom_clockcoords.gltf#Scene0")]
    pub bathroom_clockcoords: Handle<Scene>,
    #[asset(path = "levels/bathroom/expbathroom_props.gltf#Scene0")]
    pub bathroom_props: Handle<Scene>,
    #[asset(path = "levels/bathroom/expbathroom_structure.gltf#Scene0")]
    pub bathroom_structure: Handle<Scene>,
    #[asset(path = "levels/bathroom/bathroom_dust.gltf#Scene0")]
    pub bathroom_dust: Handle<Scene>,

    // COPIER
    #[asset(path = "levels/copier/expcopierroom_props.gltf#Scene0")]
    pub copierroom_props: Handle<Scene>,
    #[asset(path = "levels/copier/expcopierroom_room.gltf#Scene0")]
    pub copierroom_room: Handle<Scene>,
    #[asset(path = "levels/copier/copier_dust.gltf#Scene0")]
    pub copier_dust: Handle<Scene>,
    #[asset(path = "levels/copier/expcopierroom_coordinatesclock.gltf#Scene0")]
    pub copierroom_coordinatesclock: Handle<Scene>,

    // CONTROL ROOM
    #[asset(path = "levels/controlroom/expcontrolroom_counter.gltf#Scene0")]
    pub controlroom_counter: Handle<Scene>,
    #[asset(path = "levels/controlroom/expcontrolroom_props.gltf#Scene0")]
    pub controlroom_props: Handle<Scene>,
    #[asset(path = "levels/controlroom/expcontrolroom_structure.gltf#Scene0")]
    pub controlroom_structure: Handle<Scene>,

    // BF START
    #[asset(path = "levels/bf_start/expbf_start_building.gltf#Scene0")]
    pub bf_start_building: Handle<Scene>,
    #[asset(path = "levels/bf_start/expbf_start_grass.gltf#Scene0")]
    pub bf_start_grass: Handle<Scene>,
    #[asset(path = "levels/bf_start/expbf_start_rocks.gltf#Scene0")]
    pub bf_start_rocks: Handle<Scene>,

    // BF1
    #[asset(path = "levels/bf1/expbf1_start.gltf#Scene0")]
    pub bf1_start: Handle<Scene>,
    #[asset(path = "levels/bf1/expbf1_mid.gltf#Scene0")]
    pub bf1_mid: Handle<Scene>,
    #[asset(path = "levels/bf1/expbf1_down.gltf#Scene0")]
    pub bf1_down: Handle<Scene>,
    #[asset(path = "levels/bf1/expbf1_blinds.gltf#Scene0")]
    pub bf1_blinds: Handle<Scene>,
    #[asset(path = "levels/bf1/bf1_lights.gltf#Scene0")]
    pub bf1_lights: Handle<Scene>,
    #[asset(path = "levels/bf1/bf1_enemy_spawns.gltf#Scene0")]
    pub bf1_enemy_spawns: Handle<Scene>,

    // BFA1
    #[asset(path = "levels/bfa/expbfa_bfa1.gltf#Scene0")]
    pub bfa_bfa1: Handle<Scene>,
    #[asset(path = "levels/bfa/bfa1_enemy_spawns.gltf#Scene0")]
    pub bfa1_enemy_spawns: Handle<Scene>,

    // BFA2
    #[asset(path = "levels/bfa/expbfa_bfa2.gltf#Scene0")]
    pub bfa_bfa2: Handle<Scene>,
    #[asset(path = "levels/bfa/bfa2_enemy_spawns.gltf#Scene0")]
    pub bfa2_enemy_spawns: Handle<Scene>,

    // BFA3
    #[asset(path = "levels/bfa/expbfa_bfa3.gltf#Scene0")]
    pub bfa_bfa3: Handle<Scene>,
    #[asset(path = "levels/bfa/bfa3_enemy_spawns.gltf#Scene0")]
    pub bfa3_enemy_spawns: Handle<Scene>,

    // bfa_triggers
    #[asset(path = "levels/bfa/bfa_triggers.gltf#Scene0")]
    pub bfa_triggers: Handle<Scene>,
}

#[derive(AssetCollection, Resource)]
pub struct UnitAssets {
    #[asset(path = "units/unit1.gltf#Scene0")]
    pub unit1: Handle<Scene>,
    #[asset(path = "units/unit1.gltf#Animation0")]
    pub walk: Handle<AnimationClip>,
    #[asset(path = "units/unit1.gltf#Animation1")]
    pub idle: Handle<AnimationClip>,
    #[asset(path = "units/unit1.gltf#Animation2")]
    pub bob: Handle<AnimationClip>,
    #[asset(path = "units/unit1.gltf#Animation3")]
    pub bonk: Handle<AnimationClip>,
    #[asset(path = "units/unit1.gltf#Animation4")]
    pub fire: Handle<AnimationClip>,
    #[asset(path = "units/unit1.gltf#Animation5")]
    pub walk_lazy: Handle<AnimationClip>,
}

#[derive(AssetCollection, Resource)]
pub struct PropAssets {
    #[asset(path = "props/gun/expgun_gun.gltf#Scene0")]
    pub gun: Handle<Scene>,
    #[asset(path = "props/gun/gun_emit_part.gltf#Scene0")]
    pub gun_emit: Handle<Scene>,
    #[asset(path = "props/gun/flash.gltf#Scene0")]
    pub gun_flash: Handle<Scene>,

    #[asset(path = "props/projectile/projectile.gltf#Scene0")]
    pub projectile: Handle<Scene>,
    #[asset(path = "props/projectile/projectile_lite.gltf#Scene0")]
    pub projectile_lite: Handle<Scene>,
    #[asset(path = "props/projectile/projectile_lite_red.gltf#Scene0")]
    pub projectile_lite_red: Handle<Scene>,

    #[asset(path = "props/projectile/crosshair.gltf#Scene0")]
    pub crosshair: Handle<Scene>,
}
#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    // PLAYER GUN
    #[asset(path = "audio/playergun1.flac")]
    pub playergun1: Handle<AudioSource>,
    #[asset(path = "audio/playergun2.flac")]
    pub playergun2: Handle<AudioSource>,
    #[asset(path = "audio/playergun3.flac")]
    pub playergun3: Handle<AudioSource>,
    #[asset(path = "audio/playergun4.flac")]
    pub playergun4: Handle<AudioSource>,
    #[asset(path = "audio/playergun5.flac")]
    pub playergun5: Handle<AudioSource>,

    // ENEMY GUN
    #[asset(path = "audio/enemygun1.flac")]
    pub enemygun1: Handle<AudioSource>,
    #[asset(path = "audio/enemygun2.flac")]
    pub enemygun2: Handle<AudioSource>,
    #[asset(path = "audio/enemygun3.flac")]
    pub enemygun3: Handle<AudioSource>,
    #[asset(path = "audio/enemygun4.flac")]
    pub enemygun4: Handle<AudioSource>,
    #[asset(path = "audio/enemygun5.flac")]
    pub enemygun5: Handle<AudioSource>,

    // PLAYER HIT
    #[asset(path = "audio/playerhit1.flac")]
    pub playerhit1: Handle<AudioSource>,
    #[asset(path = "audio/playerhit2.flac")]
    pub playerhit2: Handle<AudioSource>,
    #[asset(path = "audio/playerhit3.flac")]
    pub playerhit3: Handle<AudioSource>,
    #[asset(path = "audio/playerhit4.flac")]
    pub playerhit4: Handle<AudioSource>,
    #[asset(path = "audio/playerhit5.flac")]
    pub playerhit5: Handle<AudioSource>,

    // EXPLODE
    #[asset(path = "audio/enemyexplode1.flac")]
    pub enemyexplode1: Handle<AudioSource>,
    #[asset(path = "audio/enemyexplode2.flac")]
    pub enemyexplode2: Handle<AudioSource>,
    #[asset(path = "audio/enemyexplode3.flac")]
    pub enemyexplode3: Handle<AudioSource>,
    #[asset(path = "audio/enemyexplode4.flac")]
    pub enemyexplode4: Handle<AudioSource>,
    #[asset(path = "audio/enemyexplode5.flac")]
    pub enemyexplode5: Handle<AudioSource>,
    #[asset(path = "audio/enemyexplode6.flac")]
    pub enemyexplode6: Handle<AudioSource>,

    // MUSIC
    #[asset(path = "audio/theme1.flac")]
    pub theme1: Handle<AudioSource>,
    #[asset(path = "audio/theme3.flac")]
    pub theme3: Handle<AudioSource>,
    #[asset(path = "audio/kitchen.flac")]
    pub kitchen: Handle<AudioSource>,
    #[asset(path = "audio/shower.flac")]
    pub shower: Handle<AudioSource>,
}

pub struct AssetProcPlugin;
impl Plugin for AssetProcPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((
            make_detail_repeat.in_schedule(OnEnter(GameLoading::Loaded)),
            make_env_repeat.in_schedule(OnEnter(GameLoading::Loaded)),
        ));
    }
}

fn make_detail_repeat(texture_assets: Res<TextureAssets>, mut images: ResMut<Assets<Image>>) {
    if let Some(mut detail) = images.get_mut(&texture_assets.detail) {
        detail.sampler_descriptor = ImageSampler::Descriptor(SamplerDescriptor {
            label: Some("detail"),
            address_mode_u: AddressMode::Repeat,
            address_mode_v: AddressMode::Repeat,
            address_mode_w: AddressMode::Repeat,
            mag_filter: FilterMode::Linear,
            min_filter: FilterMode::Linear,
            mipmap_filter: FilterMode::Linear,
            lod_min_clamp: 0.0,
            lod_max_clamp: std::f32::MAX,
            compare: None,
            anisotropy_clamp: NonZeroU8::new(8),
            border_color: None,
        })
    }
}

fn make_env_repeat(texture_assets: Res<TextureAssets>, mut images: ResMut<Assets<Image>>) {
    for handle in [
        texture_assets.hilly_terrain_01_puresky.clone(),
        texture_assets.kloppenheim_05_puresky.clone(),
        texture_assets.quarry_04_puresky.clone(),
        texture_assets.belfast_sunset_puresky.clone(),
    ] {
        if let Some(mut detail) = images.get_mut(&handle) {
            detail.sampler_descriptor = ImageSampler::Descriptor(SamplerDescriptor {
                label: Some("detail"),
                address_mode_u: AddressMode::Repeat,
                address_mode_v: AddressMode::ClampToEdge,
                address_mode_w: AddressMode::ClampToEdge,
                mag_filter: FilterMode::Linear,
                min_filter: FilterMode::Linear,
                mipmap_filter: FilterMode::Linear,
                lod_min_clamp: 0.0,
                lod_max_clamp: std::f32::MAX,
                compare: None,
                anisotropy_clamp: NonZeroU8::new(2),
                border_color: None,
            })
        }
    }
}
