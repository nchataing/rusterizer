use sfml::graphics::{Color, CircleShape, Shape, RenderWindow, RenderTarget, Transformable};
use crate::mesh::Mesh;
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

pub fn render_mesh(window: &mut RenderWindow, mesh: &Mesh) -> () {
    
    window.clear(&Color::BLACK);
    // Create a dot for drawing
    let mut dot = CircleShape::new(4.0,4);
    dot.set_fill_color(&Color::GREEN);
    dot.set_outline_color(&Color::GREEN);
    dot.set_outline_thickness(0.);
     
    let size_u = window.size();
    let size_x = size_u.x as f32;
    let size_y = size_u.y as f32;
    
    // Process the object rotation matrix
    let rot = mesh.get_rotation_mat(); 

    for v in &mesh.vertices {
        let v_obj = (&rot * (v.pt - mesh.rot_o)) + mesh.rot_o + mesh.translation;
        let proj = project(v_obj);
        if v_obj.z < 0. && proj.x >= -1. && proj.x <= 1. && proj.y >= -1. && proj.y <= 1. {
            dot.set_position(
            (
                (1. + proj.x) * size_x / 2.,
                (1. - proj.y) * size_y / 2.
            ));
            window.draw(&dot)
        }
    }
}
