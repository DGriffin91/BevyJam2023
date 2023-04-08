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

pub struct SkyBoxPlugin;
impl Plugin for SkyBoxPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(MaterialPlugin::<SkyBoxMaterial>::default());
    }
}

impl Material for SkyBoxMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/skybox.wgsl".into()
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
#[uuid = "36612dc3-a023-423b-af6a-51b1a63e1a95"]
pub struct SkyBoxMaterial {
    #[uniform(0)]
    pub uv_offset: Vec2,
    #[uniform(0)]
    pub brightness: f32,
    #[uniform(0)]
    pub contrast: f32,
    #[texture(1)]
    #[sampler(2)]
    pub env_texture: Option<Handle<Image>>,
}
