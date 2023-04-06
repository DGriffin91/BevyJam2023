use bevy::{
    gltf::GltfExtras,
    pbr::{
        MaterialPipeline, MaterialPipelineKey, StandardMaterialFlags, PBR_PREPASS_SHADER_HANDLE,
    },
    prelude::*,
    reflect::{FromReflect, Reflect, TypeUuid},
    render::{
        extract_component::ExtractComponent,
        mesh::MeshVertexBufferLayout,
        render_asset::RenderAssets,
        render_resource::{
            AsBindGroup, AsBindGroupShaderType, Face, RenderPipelineDescriptor, ShaderRef,
            ShaderType, SpecializedMeshPipelineError, TextureFormat,
        },
    },
    utils::HashMap,
};

use crate::assets::TextureAssets;
use crate::util::all_children;

#[derive(AsBindGroup, Reflect, FromReflect, Debug, Clone, TypeUuid)]
#[uuid = "d8393d59-19b7-46e1-9ae2-d38f35c734ae"]
#[bind_group_data(CustomStandardMaterialKey)]
#[uniform(0, CustomStandardMaterialUniform)]
#[reflect(Default, Debug)]
pub struct CustomStandardMaterial {
    pub base_color: Color,
    #[texture(1)]
    #[sampler(2)]
    pub base_color_texture: Option<Handle<Image>>,
    pub emissive: Color,
    #[texture(3)]
    #[sampler(4)]
    pub emissive_texture: Option<Handle<Image>>,
    pub perceptual_roughness: f32,
    pub metallic: f32,
    #[texture(5)]
    #[sampler(6)]
    pub metallic_roughness_texture: Option<Handle<Image>>,
    #[doc(alias = "specular_intensity")]
    pub reflectance: f32,
    #[texture(9)]
    #[sampler(10)]
    pub normal_map_texture: Option<Handle<Image>>,
    pub flip_normal_map_y: bool,
    #[texture(7)]
    #[sampler(8)]
    pub detail_texture: Option<Handle<Image>>,
    pub double_sided: bool,
    #[reflect(ignore)]
    pub cull_mode: Option<Face>,
    pub unlit: bool,
    pub fog_enabled: bool,
    pub alpha_mode: AlphaMode,
    pub depth_bias: f32,
    pub grass: bool,
    pub env_spec: f32,
    pub env_diff: f32,
}

impl Default for CustomStandardMaterial {
    fn default() -> Self {
        CustomStandardMaterial {
            base_color: Color::rgb(1.0, 1.0, 1.0),
            base_color_texture: None,
            emissive: Color::BLACK,
            emissive_texture: None,
            perceptual_roughness: 0.5,
            metallic: 0.0,
            metallic_roughness_texture: None,
            reflectance: 0.5,
            detail_texture: None,
            normal_map_texture: None,
            flip_normal_map_y: false,
            double_sided: false,
            cull_mode: Some(Face::Back),
            unlit: false,
            fog_enabled: true,
            alpha_mode: AlphaMode::Opaque,
            depth_bias: 0.0,
            grass: false,
            env_diff: 1.0,
            env_spec: 1.0,
        }
    }
}

impl From<Color> for CustomStandardMaterial {
    fn from(color: Color) -> Self {
        CustomStandardMaterial {
            base_color: color,
            alpha_mode: if color.a() < 1.0 {
                AlphaMode::Blend
            } else {
                AlphaMode::Opaque
            },
            ..Default::default()
        }
    }
}

impl From<Handle<Image>> for CustomStandardMaterial {
    fn from(texture: Handle<Image>) -> Self {
        CustomStandardMaterial {
            base_color_texture: Some(texture),
            ..Default::default()
        }
    }
}

#[derive(Clone, Default, ShaderType)]
pub struct CustomStandardMaterialUniform {
    pub base_color: Vec4,
    pub emissive: Vec4,
    pub roughness: f32,
    pub metallic: f32,
    pub reflectance: f32,
    pub flags: u32,
    pub alpha_cutoff: f32,
    pub env_spec: f32,
    pub env_diff: f32,
}

impl AsBindGroupShaderType<CustomStandardMaterialUniform> for CustomStandardMaterial {
    fn as_bind_group_shader_type(
        &self,
        images: &RenderAssets<Image>,
    ) -> CustomStandardMaterialUniform {
        let mut flags = StandardMaterialFlags::NONE;
        if self.base_color_texture.is_some() {
            flags |= StandardMaterialFlags::BASE_COLOR_TEXTURE;
        }
        if self.emissive_texture.is_some() {
            flags |= StandardMaterialFlags::EMISSIVE_TEXTURE;
        }
        if self.metallic_roughness_texture.is_some() {
            flags |= StandardMaterialFlags::METALLIC_ROUGHNESS_TEXTURE;
        }
        //if self.occlusion_texture.is_some() {
        //    flags |= StandardMaterialFlags::OCCLUSION_TEXTURE;
        //}
        if self.double_sided {
            flags |= StandardMaterialFlags::DOUBLE_SIDED;
        }
        if self.unlit {
            flags |= StandardMaterialFlags::UNLIT;
        }
        if self.fog_enabled {
            flags |= StandardMaterialFlags::FOG_ENABLED;
        }
        let has_normal_map = self.normal_map_texture.is_some();
        if has_normal_map {
            if let Some(texture) = images.get(self.normal_map_texture.as_ref().unwrap()) {
                match texture.texture_format {
                    // All 2-component unorm formats
                    TextureFormat::Rg8Unorm
                    | TextureFormat::Rg16Unorm
                    | TextureFormat::Bc5RgUnorm
                    | TextureFormat::EacRg11Unorm => {
                        flags |= StandardMaterialFlags::TWO_COMPONENT_NORMAL_MAP;
                    }
                    _ => {}
                }
            }
            if self.flip_normal_map_y {
                flags |= StandardMaterialFlags::FLIP_NORMAL_MAP_Y;
            }
        }
        let mut alpha_cutoff = 0.5;
        match self.alpha_mode {
            AlphaMode::Opaque => flags |= StandardMaterialFlags::ALPHA_MODE_OPAQUE,
            AlphaMode::Mask(c) => {
                alpha_cutoff = c;
                flags |= StandardMaterialFlags::ALPHA_MODE_MASK;
            }
            AlphaMode::Blend => flags |= StandardMaterialFlags::ALPHA_MODE_BLEND,
            AlphaMode::Premultiplied => flags |= StandardMaterialFlags::ALPHA_MODE_PREMULTIPLIED,
            AlphaMode::Add => flags |= StandardMaterialFlags::ALPHA_MODE_ADD,
            AlphaMode::Multiply => flags |= StandardMaterialFlags::ALPHA_MODE_MULTIPLY,
        };

        CustomStandardMaterialUniform {
            base_color: self.base_color.as_linear_rgba_f32().into(),
            emissive: self.emissive.as_linear_rgba_f32().into(),
            roughness: self.perceptual_roughness,
            metallic: self.metallic,
            reflectance: self.reflectance,
            flags: flags.bits(),
            alpha_cutoff,
            env_spec: self.env_spec,
            env_diff: self.env_diff,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CustomStandardMaterialKey {
    grass: bool,
    normal_map: bool,
    cull_mode: Option<Face>,
    depth_bias: i32,
}

impl From<&CustomStandardMaterial> for CustomStandardMaterialKey {
    fn from(material: &CustomStandardMaterial) -> Self {
        CustomStandardMaterialKey {
            grass: material.grass,
            normal_map: material.normal_map_texture.is_some(),
            cull_mode: material.cull_mode,
            depth_bias: material.depth_bias as i32,
        }
    }
}

impl Material for CustomStandardMaterial {
    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayout,
        key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        if let Some(fragment) = descriptor.fragment.as_mut() {
            if key.bind_group_data.normal_map {
                fragment
                    .shader_defs
                    .push("STANDARDMATERIAL_NORMAL_MAP".into());
            }
            if key.bind_group_data.grass {
                fragment.shader_defs.push("GRASS".into());
            }
        }
        descriptor.primitive.cull_mode = key.bind_group_data.cull_mode;
        if let Some(label) = &mut descriptor.label {
            *label = format!("pbr_{}", *label).into();
        }
        if let Some(depth_stencil) = descriptor.depth_stencil.as_mut() {
            depth_stencil.bias.constant = key.bind_group_data.depth_bias;
        }
        Ok(())
    }

    fn prepass_fragment_shader() -> ShaderRef {
        PBR_PREPASS_SHADER_HANDLE.typed().into()
    }

    fn fragment_shader() -> ShaderRef {
        "shaders/pbr.wgsl".into()
    }

    #[inline]
    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }

    #[inline]
    fn depth_bias(&self) -> f32 {
        self.depth_bias
    }
}

#[derive(Component, Default, Clone, Copy)]
pub struct EnvSettings {
    pub env_spec: f32,
    pub env_diff: f32,
}

pub fn swap_standard_material(
    mut commands: Commands,
    //mut material_events: EventReader<AssetEvent<StandardMaterial>>,
    entites: Query<(Entity, &Handle<StandardMaterial>)>,
    standard_materials: Res<Assets<StandardMaterial>>,
    mut custom_materials: ResMut<Assets<CustomStandardMaterial>>,
    extras: Query<&GltfExtras>,
    parent: Query<&Parent>,
    texture_assets: Res<TextureAssets>,
) {
    let mut converted = Vec::new();
    for (entity, handle) in entites.iter() {
        let mat = standard_materials.get(handle).unwrap();
        converted.push(handle.clone());
        let mut custom_mat = CustomStandardMaterial {
            base_color: mat.base_color,
            base_color_texture: mat.base_color_texture.clone(),
            emissive: mat.emissive,
            emissive_texture: mat.emissive_texture.clone(),
            perceptual_roughness: mat.perceptual_roughness,
            metallic: mat.metallic,
            metallic_roughness_texture: mat.metallic_roughness_texture.clone(),
            reflectance: mat.reflectance,
            normal_map_texture: mat.normal_map_texture.clone(),
            flip_normal_map_y: mat.flip_normal_map_y,
            detail_texture: Some(texture_assets.detail.clone()),
            double_sided: mat.double_sided,
            cull_mode: mat.cull_mode,
            unlit: mat.unlit,
            fog_enabled: mat.fog_enabled,
            alpha_mode: mat.alpha_mode,
            depth_bias: mat.depth_bias,
            grass: false,
            env_spec: 1.0,
            env_diff: 1.0,
        };

        if let Ok(parent) = parent.get(entity) {
            if let Ok(extras) = extras.get(**parent) {
                if let Ok(fields) = serde_json::from_str::<HashMap<String, String>>(&extras.value) {
                    if let Some(alpha) = fields.get("AlphaMode") {
                        let alpha = alpha.to_lowercase();
                        // Set on object properties
                        // string with ex: AlphaMode add
                        if alpha == "premultiplied" {
                            custom_mat.alpha_mode = AlphaMode::Premultiplied;
                        } else if alpha == "multiply" {
                            custom_mat.alpha_mode = AlphaMode::Multiply;
                        } else if alpha == "add" {
                            custom_mat.alpha_mode = AlphaMode::Add;
                        } else if alpha == "blend" {
                            custom_mat.alpha_mode = AlphaMode::Blend;
                        }
                    }
                }
            }
        }
        let mut ecmds = commands.entity(entity);
        ecmds.remove::<Handle<StandardMaterial>>();
        ecmds.insert(custom_materials.add(custom_mat));
    }
}

#[derive(Component)]
pub struct SetGrassMaterial;

pub fn setup_grass_mats(
    mut commands: Commands,
    scene_entities: Query<Entity, With<SetGrassMaterial>>,
    children_query: Query<&Children>,
    mat_handles: Query<&Handle<CustomStandardMaterial>>,
    mut custom_materials: ResMut<Assets<CustomStandardMaterial>>,
) {
    for entity in scene_entities.iter() {
        let mut found = false;
        if let Ok(children) = children_query.get(entity) {
            all_children(children, &children_query, &mut |entity| {
                if let Ok(mat_h) = mat_handles.get_component(entity) {
                    let mut mat = custom_materials.get_mut(mat_h).unwrap();
                    mat.grass = true;
                    found = true;
                }
            });
            if found {
                commands.entity(entity).remove::<SetGrassMaterial>();
            }
        }
    }
}

pub fn setup_env_settings(
    mut commands: Commands,
    scene_entities: Query<(Entity, &EnvSettings)>,
    children_query: Query<&Children>,
    mat_handles: Query<&Handle<CustomStandardMaterial>>,
    mut custom_materials: ResMut<Assets<CustomStandardMaterial>>,
) {
    for (entity, env_settings) in scene_entities.iter() {
        let mut found = false;
        if let Ok(children) = children_query.get(entity) {
            all_children(children, &children_query, &mut |entity| {
                if let Ok(mat_h) = mat_handles.get_component(entity) {
                    let mut mat = custom_materials.get_mut(mat_h).unwrap();
                    mat.env_spec = env_settings.env_spec;
                    mat.env_diff = env_settings.env_diff;
                    found = true;
                }
            });
            if found {
                commands.entity(entity).remove::<SetGrassMaterial>();
            }
        }
    }
}
/*
#[derive(Component)]
pub struct CurtainSetBlend;

pub fn setup_curtains(
    mut commands: Commands,
    scene_entities: Query<Entity, With<CurtainSetBlend>>,
    children_query: Query<&Children>,
    mat_handles: Query<&Handle<CustomStandardMaterial>>,
    mut custom_materials: ResMut<Assets<CustomStandardMaterial>>,
) {
    for entity in scene_entities.iter() {
        let mut found = false;
        if let Ok(children) = children_query.get(entity) {
            all_children(children, &children_query, &mut |entity| {
                if let Ok(mat_h) = mat_handles.get_component(entity) {
                    let mut mat = custom_materials.get_mut(mat_h).unwrap();
                    mat.alpha_mode = AlphaMode::Premultiplied;
                    found = true;
                }
            });
            if found {
                commands.entity(entity).remove::<CurtainSetBlend>();
            }
        }
    }
}
*/
