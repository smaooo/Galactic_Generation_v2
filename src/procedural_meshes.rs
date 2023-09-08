use bevy::math::*;
use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy::render::render_resource::PrimitiveTopology;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vertex {
    pub position: Vec3,
    pub normal: Vec3,
    pub tangent: Vec4,
    pub tex_coord0: Vec2,
}

impl Default for Vertex {
    fn default() -> Self {
        Self {
            position: Vec3::ZERO,
            normal: Vec3::ZERO,
            tangent: Vec4::ZERO,
            tex_coord0: Vec2::ZERO,
        }
    }
}

// In the following functions the y and z parameters can be swapped to change the orientation of the grid.

fn generate_quad_vertices(
    i: u32,
    resolution: u32,
    vertices: &mut std::vec::Vec<Vertex>,
    triangles: &mut std::vec::Vec<u32>,
) {
    let mut vi = (resolution + 1) * i;
    let mut ti: i32 = (6 * resolution) as i32 * (i as i32 - 1);

    let mut vertex = Vertex::default();

    vertex.normal.z = 1.0;
    vertex.tangent.x = -1.0;
    vertex.tangent.w = -1.0;

    vertex.position.x = -0.5;
    vertex.position.y = (i as f32 / resolution as f32) - 0.5;
    vertex.tex_coord0.y = i as f32 / resolution as f32;

    vertices[vi as usize] = vertex;

    vi += 1;
    for x in 1..resolution + 1 {
        vertex.position.x = (x as f32 / resolution as f32) - 0.5;
        vertex.tex_coord0.x = x as f32 / resolution as f32;
        vertices[vi as usize] = vertex;

        if i > 0 {
            triangles[ti as usize] = vi - resolution - 2;
            triangles[ti as usize + 1] = vi - resolution - 1;
            triangles[ti as usize + 2] = vi - 1;
            triangles[ti as usize + 3] = vi - resolution - 1;
            triangles[ti as usize + 4] = vi;
            triangles[ti as usize + 5] = vi - 1;
        }

        vi += 1;
        ti += 6;
    }
}

fn generate_uv_sphere_vertices(
    u: u32,
    resolution: u32,
    vertices: &mut std::vec::Vec<Vertex>,
    triangles: &mut std::vec::Vec<u32>,
) {
    let mut vi = (resolution + 1) * u;
    let mut ti: i32 = (6 * resolution) as i32 * (u as i32 - 1);

    let mut vertex = Vertex::default();
    vertex.position.y = -1.0;
    vertex.normal.y = -1.0;

    vertex.tangent.w = -1.0;

    let mut circle = Vec2::ZERO;
    circle.x = f32::sin(2.0 * std::f32::consts::PI * u as f32 / resolution as f32);
    circle.y = f32::cos(2.0 * std::f32::consts::PI * u as f32 / resolution as f32);

    vertex.tangent.x = circle.y;
    vertex.tangent.z = circle.x;

    circle.y = -circle.y;

    vertex.tex_coord0.x = u as f32 / resolution as f32;

    vertices[vi as usize] = vertex;

    vi += 1;
    for v in 1..resolution + 1 {
        let circle_radius = f32::sin(std::f32::consts::PI * v as f32 / resolution as f32);
        vertex.position.x = circle.x * circle_radius;
        vertex.position.z = circle.y * circle_radius;
        vertex.position.y = -f32::cos(std::f32::consts::PI * v as f32 / resolution as f32);
        vertex.normal = vertex.position;
        vertex.tex_coord0.y = v as f32 / resolution as f32;
        vertices[vi as usize] = vertex;

        if u > 0 {
            triangles[ti as usize] = vi - resolution - 2;
            triangles[ti as usize + 1] = vi - resolution - 1;
            triangles[ti as usize + 2] = vi - 1;
            triangles[ti as usize + 3] = vi - resolution - 1;
            triangles[ti as usize + 4] = vi;
            triangles[ti as usize + 5] = vi - 1;
        }

        vi += 1;
        ti += 6;
    }
}

fn generate_tris_vertices(
    i: u32,
    resolution: u32,
    vertices: &mut std::vec::Vec<Vertex>,
    triangles: &mut std::vec::Vec<u32>,
) {
    let mut vi = (resolution + 1) * i;
    let mut ti: i32 = (6 * resolution) as i32 * (i as i32 - 1);

    let mut x_offset = -0.25;
    let mut u_offset = 0.0;

    let i_a = -(resolution as i32) - 2;
    let i_b = -(resolution as i32) - 1;
    let i_c = -1;
    let i_d = 0;
    let mut t_a = IVec3::new(i_a, i_d, i_c);
    let mut t_b = IVec3::new(i_a, i_b, i_d);

    if (i & 1) == 1 {
        x_offset = 0.25;
        u_offset = 0.5 / (resolution as f32 + 0.5);

        t_a = IVec3::new(i_a, i_b, i_c);
        t_b = IVec3::new(i_b, i_d, i_c);
    }

    x_offset /= resolution as f32 - 0.5;

    let mut vertex = Vertex::default();

    vertex.normal.z = 1.0;
    vertex.tangent.x = -1.0;
    vertex.tangent.w = -1.0;

    vertex.position.x = x_offset;
    vertex.position.y = ((i as f32 / resolution as f32) - 0.5) * f32::sqrt(3.0) / 2.0;
    vertex.tex_coord0.x = u_offset;
    vertex.tex_coord0.y = vertex.position.y / (1.0 + (0.5 / resolution as f32)) + 0.5;

    vertices[vi as usize] = vertex;

    vi += 1;
    for x in 1..resolution + 1 {
        vertex.position.x = (x as f32 / resolution as f32) + x_offset;
        vertex.tex_coord0.x = x as f32 / (resolution as f32 + 0.5) + u_offset;
        vertices[vi as usize] = vertex;

        if i > 0 {
            triangles[ti as usize] = (vi as i32 + t_a.x) as u32;
            triangles[ti as usize + 1] = (vi as i32 + t_a.y) as u32;
            triangles[ti as usize + 2] = (vi as i32 + t_a.z) as u32;
            triangles[ti as usize + 3] = (vi as i32 + t_b.x) as u32;
            triangles[ti as usize + 4] = (vi as i32 + t_b.y) as u32;
            triangles[ti as usize + 5] = (vi as i32 + t_b.z) as u32;
        }

        vi += 1;
        ti += 6;
    }
}

pub fn generate_grid(resolution: u32) -> Mesh {
    let vertex_count: u32 = (resolution + 1) * (resolution + 1);
    let index_count: u32 = 6 * resolution * resolution;

    let mut vertices = vec![Vertex::default(); vertex_count as usize];
    let mut triangles = vec![0 as u32; index_count as usize];

    for i in 0..resolution + 1 {
        generate_uv_sphere_vertices(i, resolution, &mut vertices, &mut triangles);
    }

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vertices.iter().map(|v| v.position).collect::<Vec<Vec3>>(),
    );

    mesh.set_indices(Some(Indices::U32(triangles)));

    mesh.insert_attribute(
        Mesh::ATTRIBUTE_NORMAL,
        vertices.iter().map(|v| v.normal).collect::<Vec<Vec3>>(),
    );

    mesh.insert_attribute(
        Mesh::ATTRIBUTE_UV_0,
        vertices.iter().map(|v| v.tex_coord0).collect::<Vec<Vec2>>(),
    );

    mesh.insert_attribute(
        Mesh::ATTRIBUTE_TANGENT,
        vertices.iter().map(|v| v.tangent).collect::<Vec<Vec4>>(),
    );

    mesh
}
