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
        app.add_plugin(MaterialPlugin::<SkyBoxMaterial>::default())
            .add_startup_system(add_skybox);
    }
}

fn add_skybox(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<SkyBoxMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 100000.0 })),
        material: materials.add(SkyBoxMaterial {
            env_texture: Some(asset_server.load("../../temp_assets/kloppenheim_05_puresky_2k.exr")),
        }),
        ..default()
    });
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
    #[texture(0)]
    #[sampler(1)]
    pub env_texture: Option<Handle<Image>>,
}
