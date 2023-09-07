use bevy::{
    core::FrameCount,
    input::mouse::MouseMotion,
    math::*,
    prelude::{shape::Cube, *},
    render::{
        mesh::{Indices, VertexAttributeValues},
        render_graph::Node,
        render_resource::PrimitiveTopology,
    },
    window::CursorGrabMode,
};

#[derive(Component)]
struct CustomUV;

#[derive(Component)]
struct MousePos {
    prev_pos: Vec2,
    current_pos: Vec2,
}

impl Default for MousePos {
    fn default() -> Self {
        Self {
            prev_pos: Vec2::ZERO,
            current_pos: Vec2::ZERO,
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
        ],
    );
    mesh.set_indices(Some(Indices::U32(vec![0, 1, 2])));

    mesh.insert_attribute(
        Mesh::ATTRIBUTE_NORMAL,
        vec![
            (-Vec3::Z).to_array(),
            (-Vec3::Z).to_array(),
            (-Vec3::Z).to_array(),
        ],
    );

    mesh.insert_attribute(
        Mesh::ATTRIBUTE_UV_0,
        vec![
            Vec2::ZERO.to_array(),
            Vec2::X.to_array(),
            Vec2::Y.to_array(),
        ],
    );

    mesh
}
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, input_handler)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    //let texture = asset_server.load("../base-map.png");
    let mesh = meshes.add(generate_quad());

    commands.spawn((PbrBundle {
        mesh,
        // material: materials.add(StandardMaterial {
        //     base_color_texture: Some(texture),
        //     ..default()
        // })
        ..default()
    },));

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

    // Text to describe the controls.
    commands.spawn(
        TextBundle::from_section(
            "Controls:\nSpace: Change UVs\nX/Y/Z: Rotate\nR: Reset orientation",
            TextStyle {
                font_size: 20.0,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        }),
    );
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
                    println!("delta: {:?}", delta);
                    println!("Rotation: {:?}", rotation);
                    camera.rotate_around(Vec3::ZERO, Quat::from_euler(EulerRot::YXZ, rotation.x, rotation.y, 0.0));


                }
            }
        }
    }
}

fn generate_sphere_cube() -> Mesh {
    Mesh::new(PrimitiveTopology::TriangleList)
}
