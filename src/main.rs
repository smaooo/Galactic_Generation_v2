use bevy::{
    math::*,
    pbr::{
        wireframe::{Wireframe, WireframeConfig, WireframePlugin},
        MaterialPipeline, MaterialPipelineKey,
    },
    prelude::{
        shape::{Cube, Torus, UVSphere},
        *,
    },
    reflect::{TypePath, TypeUuid},
    render::{
        mesh::Indices,
        mesh::MeshVertexBufferLayout,
        render_resource::PrimitiveTopology,
        render_resource::{
            AsBindGroup, RenderPipelineDescriptor, ShaderRef, SpecializedMeshPipelineError,
        },
    },
};

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

fn generate_quad() -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![
            Vec3::ZERO.to_array(),
            Vec3::X.to_array(),
            Vec3::Y.to_array(),
            (Vec3::X + Vec3::Y).to_array(),
        ],
    );
    mesh.set_indices(Some(Indices::U32(vec![0, 1, 2, 1, 3, 2])));

    mesh.insert_attribute(
        Mesh::ATTRIBUTE_NORMAL,
        vec![
            (Vec3::Z).to_array(),
            (Vec3::Z).to_array(),
            (Vec3::Z).to_array(),
            (Vec3::Z).to_array(),
        ],
    );

    mesh.insert_attribute(
        Mesh::ATTRIBUTE_UV_0,
        vec![
            Vec2::ZERO.to_array(),
            Vec2::X.to_array(),
            Vec2::Y.to_array(),
            Vec2::ONE.to_array(),
        ],
    );

    mesh
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
    // mut materials: ResMut<Assets<StandardMaterial>>,
    mut custom_materials: ResMut<Assets<CustomMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut wireframe_config: ResMut<WireframeConfig>,
) {
    wireframe_config.global = false;

    let texture = asset_server.load("base-map.png");
    let mesh = meshes.add(generate_quad());
   
    commands.spawn((
        MaterialMeshBundle {
            mesh,
            material: custom_materials.add(CustomMaterial {
                color: Color::WHITE,
                color_texture: Some(texture),
                alpha_mode: AlphaMode::Blend,
            }),

            ..default()
        },
        Wireframe,
    ));

    let camera_and_light_transform =
        Transform::from_xyz(0.0, 2.0, 2.0).looking_at(Vec3::ZERO, Vec3::Y);

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

                let yaw = delta.x;
                let pitch = delta.y;

                if yaw.abs() > 0.1 || pitch.abs() > 0.1 {
                    let mut rotation = yaw * Vec3::X + pitch * Vec3::Y;
                    rotation = Vec3::normalize(rotation);
                    rotation *= time.delta_seconds();
                    // println!("delta: {:?}", delta);
                    // println!("Rotation: {:?}", rotation);
                    camera.rotate_around(
                        Vec3::ZERO,
                        Quat::from_euler(EulerRot::YXZ, rotation.x, rotation.y, 0.0),
                    );
                }
            }
        }
    }
}
