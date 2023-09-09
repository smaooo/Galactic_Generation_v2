use crate::components::*;
use crate::procedural_meshes::*;
use bevy::input::keyboard;
use bevy::input::touchpad::TouchpadMagnify;
use bevy::pbr::wireframe::Wireframe;
use bevy::render::camera;
use bevy::window::CursorGrabMode;
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
    let raw_mesh = generate_grid(10);
    let mesh = meshes.add(raw_mesh);

    commands.spawn((
        MaterialMeshBundle {
            mesh,
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                rotation: Quat::IDENTITY,
                scale: Vec3::new(1.0, 1.0, 1.0),
            },
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
        Transform::from_xyz(0.5, 0.5, 4.0).looking_at(Vec3::ZERO, -Vec3::Y);

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
    let mut window = windows.single_mut();
    
    if mouse_input.just_released(MouseButton::Left) {
        window.cursor.visible = false;
        if let Some(position) = window.cursor_position() {
            mouse_pos.single_mut().prev_pos = position;
        }
    }
    if keyboard_input.just_released(KeyCode::Escape) {
        window.cursor.visible = true;
    }

    if !window.cursor.visible {
        if let Some(position) = window.cursor_position() {
            let mut delta = mouse_pos.single_mut().calculate_delta(position);
            
            for mut camera in &mut query {
                delta *= time.delta_seconds();

                camera.rotate_local_y(delta.x);
                camera.rotate_local_x(delta.y);
            }
        }
    }
    let move_speed = if keyboard_input.pressed(KeyCode::ShiftLeft) {
        10.0
    } else {
        1.0
    };

    for mut camera in &mut query {
        let left = camera.left() * keyboard_input.pressed(KeyCode::A) as i8 as f32;
        let right = camera.right() * keyboard_input.pressed(KeyCode::D) as i8 as f32;
        let forward = camera.forward() * keyboard_input.pressed(KeyCode::W) as i8 as f32;
        let back = camera.back() * keyboard_input.pressed(KeyCode::S) as i8 as f32;
        let up = camera.up() * keyboard_input.pressed(KeyCode::E) as i8 as f32;
        let down = camera.down() * keyboard_input.pressed(KeyCode::Q) as i8 as f32;

        camera.translation += (left + right + up + down + forward + back) * time.delta_seconds() * move_speed;

        if keyboard_input.just_released(KeyCode::R) {
            camera.translation = Vec3::new(0.5, 0.5, 4.0);
            camera.look_at(Vec3::ZERO, -Vec3::Y);
        }
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
