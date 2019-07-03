use sfml::graphics::{Color, Vertex, VertexArray, PrimitiveType, RenderWindow, RenderTarget};
use crate::mesh::Mesh;
use crate::camera::Camera;
use crate::vector::*;
use crate::matrix::*;
use std::f32::consts::PI;
use std::f32;

pub fn project(pt: Vector3) -> Vector3 {
    let fov = 90.;
    let near = 0.1;
    let far = 20.;
    let scale = 1. / f32::tan(fov * PI / (360.));

    Vector3::new(
        -scale * pt.x / pt.z,
        -scale * pt.y / pt.z,
        far * (1. - near / pt.z) / (far - near)
    )
}

pub fn render_mesh(window: &mut RenderWindow, mesh: &Mesh, cam: &Camera) {

    window.clear(&Color::BLACK);

    // Create a VertexArray for drawing
    let nb_tris = mesh.faces.len();
    let mut vertex_array = VertexArray::new(PrimitiveType::Lines, 3*nb_tris);

    let size_u = window.size();
    let size_x = size_u.x as f32;
    let size_y = size_u.y as f32;

    // Process the object rotation matrix
    let obj_mat = mesh.get_mat();
    let cam_mat = cam.get_mat();
    let proj_mat = Matrix4::project(&cam);

    let m = proj_mat * cam_mat * obj_mat;

    // Process the coordinates of each point
    let mut proj_vert : Vec<(f32,f32)> = vec!();

    for v in &mesh.vertices {
        let proj = &m * v.pt;
        proj_vert.push(((1. + proj.x) * size_x / 2., (1. - proj.y) * size_y / 2.))
    }

    for tri in &mesh.faces {
        vertex_array.append(&Vertex::with_pos(proj_vert[tri.a]));
        vertex_array.append(&Vertex::with_pos(proj_vert[tri.b]));
        vertex_array.append(&Vertex::with_pos(proj_vert[tri.b]));
        vertex_array.append(&Vertex::with_pos(proj_vert[tri.c]));
        vertex_array.append(&Vertex::with_pos(proj_vert[tri.c]));
        vertex_array.append(&Vertex::with_pos(proj_vert[tri.a]));
    }

    window.draw(&vertex_array)
}
