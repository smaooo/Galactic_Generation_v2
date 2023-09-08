use crate::components::*;
use crate::procedural_meshes::*;
use bevy::input::keyboard;
use bevy::input::touchpad::TouchpadMagnify;
use bevy::pbr::wireframe::Wireframe;
use bevy::{pbr::wireframe::WireframeConfig, prelude::*};

pub fn setup(
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

    let mesh = meshes.add(generate_grid(10));
    commands.spawn((
        MaterialMeshBundle {
            mesh,
            material: materials.add(StandardMaterial {
                base_color_texture: Some(color_texture.clone()),
                normal_map_texture: Some(normal_texture.clone()),
                ..default()
            }),

            ..default()
        },
        Wireframe,
        CustomUV,
    ));

    let camera_and_light_transform =
        Transform::from_xyz(1.0, 1.0, 4.0).looking_at(Vec3::new(1.0, 1.0, 0.0), -Vec3::Y);

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

pub fn input_handler(
    mouse_input: Res<Input<MouseButton>>,
    keyboard_input: Res<Input<keyboard::KeyCode>>,
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

    if mouse_input.pressed(MouseButton::Left) && !keyboard_input.pressed(KeyCode::ShiftLeft) {
        if let Some(position) = window.cursor_position() {
            for mut camera in &mut query {
                let delta = mouse_pos.single_mut().calculate_delta(position);

                if delta.x.abs() > 0.1 || delta.y.abs() > 0.1 {
                    let dir_x = camera.local_x();
                    let dir_y = camera.local_y();

                    let mut rotation = delta.x * dir_x + delta.y * dir_y;
                    rotation = Vec3::normalize(rotation);
                    rotation *= time.delta_seconds() * 2.0;

                    camera.rotate_around(
                        Vec3::ZERO,
                        Quat::from_euler(EulerRot::YXZ, 0.0, rotation.y, 0.0),
                    );
                    camera.rotate_around(
                        Vec3::ZERO,
                        Quat::from_euler(EulerRot::YXZ, rotation.x, 0.0, 0.0),
                    );
                }
            }
        }
    } else if mouse_input.pressed(MouseButton::Left) && keyboard_input.pressed(KeyCode::ShiftLeft) {
        if let Some(position) = window.cursor_position() {
            for mut camera in &mut query {
                let delta = mouse_pos.single_mut().calculate_delta(position);

                if delta.x.abs() > 0.1 || delta.y.abs() > 0.1 {
                    let dir_x = camera.local_x();
                    let dir_y = camera.local_y();

                    camera.translation += (dir_x * delta.x + dir_y * delta.y) * 0.0001;
                }
            }
        }
    }

    if mouse_input.just_pressed(MouseButton::Middle) {
        println!("Middle mouse button pressed");
    }
}

pub fn touchpad_gestures(
    mut evr_touchpad_magnify: EventReader<TouchpadMagnify>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
) {
    for evr_magnify in evr_touchpad_magnify.iter() {
        for mut camera in camera_query.iter_mut() {
            let direction = camera.local_z();
            camera.translation -= direction * evr_magnify.0;
        }
    }
}

pub fn draw_gizmos(
    mut gizmos: Gizmos,
    meshes: Res<Assets<Mesh>>,
    mesh_query: Query<&Handle<Mesh>, With<CustomUV>>,
) {
    let mesh_handle = mesh_query.single();
    let mesh = meshes.get(mesh_handle).unwrap();

    let vertices = mesh.attribute(Mesh::ATTRIBUTE_POSITION);
    let normals = mesh.attribute(Mesh::ATTRIBUTE_NORMAL);
    let tangents = mesh.attribute(Mesh::ATTRIBUTE_TANGENT);
    if let (Some(v), Some(n), Some(t)) = (vertices, normals, tangents) {
        let a = t.get_bytes().to_vec();
        let mut raw_vals = vec![];
        for i in (0..a.len()).step_by(4) {
            let tmp = [a[i], a[i + 1], a[i + 2], a[i + 3]];
            raw_vals.push(f32::from_le_bytes(tmp));
        }

        let mut t_3 = vec![];
        for i in (0..raw_vals.len()).step_by(4) {
            let tmp = [raw_vals[i], raw_vals[i + 1], raw_vals[i + 2]];
            t_3.push(tmp);
        }

        v.as_float3()
            .zip(n.as_float3().zip(Some(t_3)))
            .map(|(v, (n, t))| {
                for i in 0..v.len() {
                    let v = Vec3::from_array(v[i]);
                    let n = Vec3::from_array(n[i]);
                    let t = Vec3::from_array(t[i]);
                    gizmos.sphere(v, Quat::IDENTITY, 0.01, Color::RED);
                    gizmos.ray(v, n * 0.1, Color::BLUE);
                    gizmos.ray(v, t * 0.1, Color::GREEN);
                }
            });
    }
}
