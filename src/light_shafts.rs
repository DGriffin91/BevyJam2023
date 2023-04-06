use bevy::{
    pbr::{MaterialPipeline, MaterialPipelineKey},
    prelude::*,
    reflect::TypeUuid,
    render::{
        mesh::MeshVertexBufferLayout,
        render_resource::{
            AsBindGroup, RenderPipelineDescriptor, ShaderRef, SpecializedMeshPipelineError,
        },
    },
};

use crate::{pbr_material::CustomStandardMaterial, util::all_children};

pub struct LightShaftsPlugin;
impl Plugin for LightShaftsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(MaterialPlugin::<LightShaftsMaterial>::default())
            .add_system(setup_light_shafts);
    }
}

#[derive(Component)]
pub struct SetLightShaftMaterial(pub LightShaftsMaterial);

pub fn setup_light_shafts(
    mut commands: Commands,
    scene_entities: Query<(Entity, &SetLightShaftMaterial)>,
    children_query: Query<&Children>,
    mat_handles: Query<&Handle<CustomStandardMaterial>>,
    mut custom_materials: ResMut<Assets<CustomStandardMaterial>>,
    mut light_shaft_materials: ResMut<Assets<LightShaftsMaterial>>,
) {
    for (entity, light_shaft) in scene_entities.iter() {
        let mut found = false;
        if let Ok(children) = children_query.get(entity) {
            all_children(children, &children_query, &mut |entity| {
                if let Ok(mat_h) = mat_handles.get_component(entity) {
                    let mut mat = custom_materials.get_mut(mat_h).unwrap();
                    mat.alpha_mode = AlphaMode::Premultiplied;
                    found = true;
                }
                commands
                    .entity(entity)
                    .insert(light_shaft_materials.add(light_shaft.0.clone()));
                commands
                    .entity(entity)
                    .remove::<Handle<CustomStandardMaterial>>();
            });
            if found {
                commands.entity(entity).remove::<SetLightShaftMaterial>();
            }
        }
    }
}

impl Material for LightShaftsMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/light_shafts.wgsl".into()
    }

    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayout,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        descriptor.primitive.cull_mode = None;
        Ok(())
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Add
    }
}

#[derive(AsBindGroup, Debug, Clone, TypeUuid)]
#[uuid = "146058e4-91a5-4c30-ad79-6a7ec31fb260"]
pub struct LightShaftsMaterial {
    #[uniform(0)]
    pub color: Vec3,
    #[uniform(0)]
    pub shaft: f32,
    #[uniform(0)]
    pub dust: f32,
    #[uniform(0)]
    pub dust_size: f32,
    #[uniform(0)]
    pub dust_qty_sub: f32,
}
