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

pub struct PlantsPlugin;
impl Plugin for PlantsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(MaterialPlugin::<PlantsMaterial>::default())
            .add_system(setup_plants);
    }
}

#[derive(Component)]
pub struct SetPlantsMaterial(pub PlantsMaterial);

pub fn setup_plants(
    mut commands: Commands,
    scene_entities: Query<(Entity, &SetPlantsMaterial)>,
    children_query: Query<&Children>,
    mat_handles: Query<&Handle<CustomStandardMaterial>>,
    mut custom_materials: ResMut<Assets<CustomStandardMaterial>>,
    mut light_shaft_materials: ResMut<Assets<PlantsMaterial>>,
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
                commands.entity(entity).remove::<SetPlantsMaterial>();
            }
        }
    }
}

impl Material for PlantsMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/plants.wgsl".into()
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
}

#[derive(AsBindGroup, Debug, Clone, TypeUuid)]
#[uuid = "146058e4-92a5-4c30-ad79-6a7ec31fb260"]
pub struct PlantsMaterial {}
