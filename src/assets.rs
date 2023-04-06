use std::num::NonZeroU8;

use bevy::{
    prelude::*,
    render::{
        render_resource::{AddressMode, FilterMode, SamplerDescriptor},
        texture::ImageSampler,
    },
};
use bevy_asset_loader::prelude::*;

use crate::GameLoading;

#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
    #[asset(path = "textures/detail.ktx2")]
    pub detail: Handle<Image>,
    #[asset(path = "environment_maps/quarry_04_puresky_2k.ktx2")]
    pub quarry_04_puresky: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct LevelAssets {
    // URBAN
    #[asset(path = "levels/urban/ExpUrban.blend_FarAwayBuildings.gltf#Scene0")]
    pub urban_far_away_buildings: Handle<Scene>,
    #[asset(path = "levels/urban/ExpUrban.blend_Props.gltf#Scene0")]
    pub urban_props: Handle<Scene>,
    #[asset(path = "levels/urban/ExpUrban.blend_Structure.gltf#Scene0")]
    pub urban_structure: Handle<Scene>,
    #[asset(path = "levels/urban/ExpUrban.blend_SurroundingBuildings.gltf#Scene0")]
    pub urban_surrounding_buildings: Handle<Scene>,
    #[asset(path = "levels/urban/urban_dust.gltf#Scene0")]
    pub urban_dust: Handle<Scene>,

    // HOUSES
    #[asset(path = "levels/houses/ExpHouses.blend_ClockPackage.gltf#Scene0")]
    pub houses_clock_package: Handle<Scene>,
    #[asset(path = "levels/houses/ExpHouses.blend_Grass3D.gltf#Scene0")]
    pub houses_grass3d: Handle<Scene>,
    #[asset(path = "levels/houses/ExpHouses.blend_Houses.gltf#Scene0")]
    pub houses_houses: Handle<Scene>,
    #[asset(path = "levels/houses/ExpHouses.blend_Houses2.gltf#Scene0")]
    pub houses_houses2: Handle<Scene>,
    #[asset(path = "levels/houses/ExpHouses.blend_Props.gltf#Scene0")]
    pub houses_props: Handle<Scene>,
    #[asset(path = "levels/houses/ExpHouses.blend_Structure.gltf#Scene0")]
    pub houses_structure: Handle<Scene>,
    #[asset(path = "levels/houses/houses_dust.gltf#Scene0")]
    pub houses_dust: Handle<Scene>,
}

pub struct AssetProcPlugin;
impl Plugin for AssetProcPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(make_detail_repeat.in_schedule(OnEnter(GameLoading::Loaded)));
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
