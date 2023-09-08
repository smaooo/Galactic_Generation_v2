use bevy::{
    math::*,
    pbr::{
        wireframe::{Wireframe, WireframeConfig, WireframePlugin},
        MaterialPipeline, MaterialPipelineKey,
    },
    prelude::*,
    reflect::{TypePath, TypeUuid},
    render::{
        color,
        mesh::Indices,
        mesh::MeshVertexBufferLayout,
        render_resource::PrimitiveTopology,
        render_resource::{
            AsBindGroup, RenderPipelineDescriptor, ShaderRef, SpecializedMeshPipelineError,
        },
    },
};
use galactic_generation_v2::procedural_meshes::*;

#[derive(Component)]
struct CustomUV;

#[derive(Component)]
struct MousePos {
    prev_pos: Vec2,
}

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
impl Default for MousePos {
    fn default() -> Self {
        Self {
            prev_pos: Vec2::ZERO,
        }
    }
}

impl MousePos {
    fn calculate_delta(&mut self, current_pos: Vec2) -> Vec2 {
        let delta = self.prev_pos - current_pos;
        delta
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
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    // mut custom_materials: ResMut<Assets<CustomMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut wireframe_config: ResMut<WireframeConfig>,
) {
    wireframe_config.global = false;

    let color_texture = asset_server.load("base-map.png");
    let normal_texture = asset_server.load("normal-map.png");

    let mesh = meshes.add(generate_grid(4));
    commands.spawn((
        MaterialMeshBundle {
            mesh,
            material: materials.add(StandardMaterial {
                base_color_texture: Some(color_texture.clone()),
                normal_map_texture: Some(normal_texture.clone()),
                double_sided: true,
                ..default()
            }),

            ..default()
        },
        Wireframe,
    ));

    let camera_and_light_transform =
        Transform::from_xyz(1.0, 1.0, 4.0).looking_at(Vec3::new(1.0, 1.0, 0.0), Vec3::Y);

    // Camera in 3D space.
    commands.spawn((
        Camera3dBundle {
            transform: camera_and_light_transform,
            ..default()
        },
        MousePos::default(),
    ));

    // Light up the scene.
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1000.0,
            range: 100.0,
            ..default()
        },
        transform: camera_and_light_transform,
        ..default()
    });
}

fn input_handler(
    mouse_input: Res<Input<MouseButton>>,
    mut windows: Query<&mut Window>,
    mut query: Query<&mut Transform, With<Camera>>,
    mut mouse_pos: Query<&mut MousePos>,
    time: Res<Time>,
) {
    let window = windows.single_mut();
    if mouse_input.just_pressed(MouseButton::Left) {
        if let Some(position) = window.cursor_position() {
            mouse_pos.single_mut().as_mut().prev_pos = position;
        }
    }
    if mouse_input.pressed(MouseButton::Left) {
        if let Some(position) = window.cursor_position() {
            for mut camera in &mut query {
                let delta = mouse_pos.single_mut().calculate_delta(position);

                if delta.x.abs() > 0.1 || delta.y.abs() > 0.1 {
                    let mut rotation = delta.x * Vec3::X + delta.y * Vec3::Y;
                    rotation = Vec3::normalize(rotation);
                    rotation *= time.delta_seconds();

                    camera.rotate_around(
                        Vec3::ZERO,
                        Quat::from_euler(EulerRot::YXZ, rotation.x, rotation.y, 0.0),
                    );
                }
            }
        }
    }
}
