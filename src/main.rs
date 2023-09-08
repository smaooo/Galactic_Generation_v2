use bevy::{
    pbr::{wireframe::WireframePlugin, MaterialPipeline, MaterialPipelineKey},
    prelude::*,
    reflect::{TypePath, TypeUuid},
    render::{
        mesh::MeshVertexBufferLayout,
        render_resource::{
            AsBindGroup, RenderPipelineDescriptor, ShaderRef, SpecializedMeshPipelineError,
        },
    },
};
use galactic_generation_v2::systems::*;

#[derive(AsBindGroup, Clone, TypeUuid, TypePath)]
#[uuid = "4ee9c363-1124-4113-890e-199d81b00281"]
struct CustomMaterial {
    #[uniform(0)]
    color: Color,
    #[texture(1)]
    #[sampler(2)]
    color_texture: Option<Handle<Image>>,
    #[texture(3)]
    #[sampler(4)]
    normal_map_texture: Option<Handle<Image>>,
    alpha_mode: AlphaMode,
}

impl Material for CustomMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders/flip_tex.vert".into()
    }
    fn fragment_shader() -> ShaderRef {
        "shaders/flip_tex.frag".into()
    }
    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }
    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayout,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        descriptor.vertex.entry_point = "main".into();
        descriptor.fragment.as_mut().unwrap().entry_point = "main".into();
        Ok(())
    }
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            WireframePlugin,
            MaterialPlugin::<CustomMaterial>::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, input_handler)
        .add_systems(Update, draw_gizmos)
        .run();
}
