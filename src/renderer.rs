use sfml::graphics::{Color, Vertex, VertexArray, PrimitiveType, RenderWindow, RenderTarget};
use sfml::system::{Vector2f};
use crate::mesh::Mesh;
use crate::camera::Camera;
use crate::vector::*;
use crate::matrix::*;
use crate::light::Light;
use std::f32::consts::PI;
use std::f32;

pub fn fill_triangle(window: &RenderWindow, points: &mut VertexArray, zbuf: &mut Vec<f32>,
                     va: Vector3, vb: Vector3, vc: Vector3, color: Color) {

    // Near and far plane clipping : no need to run through the procedure is the
    // triangle is fully outside the field of view.
    if (va.z > 1. && vb.z > 1. && vc.z > 1.) ||
       (va.z < -1. && vb.z < -1. && vc.z < -1.) {
        return;
    }

    fn edge_function(a: &Vector3, b: &Vector3, c: &Vector3) -> f32 {
        (c.x - a.x) * (b.y - a.y) - (c.y - a.y) * (b.x - a.x)
    }

    let x0 = va.x.min(vb.x).min(vc.x).max(0.);
    let x1 = va.x.max(vb.x).max(vc.x).min((window.size().x - 1) as f32);
    let y0 = va.y.min(vb.y).min(vc.y).max(0.);
    let y1 = va.y.max(vb.y).max(vc.y).min((window.size().y - 1) as f32);
    let area = edge_function(&va, &vb, &vc);
    if area == 0. {
        return;
    }

    let za = 1. / va.z;
    let zb = 1. / vb.z;
    let zc = 1. / vc.z;

    let mut p = Vector3::new(0., y0.floor(), 0.);
    while p.y <= y1.ceil() {
        p.x = x0.floor();
        while p.x <= x1.ceil() {
            let wc = edge_function(&va, &vb, &p);
            let wa = edge_function(&vb, &vc, &p);
            let wb = edge_function(&vc, &va, &p);
            let inside = wa >= 0. && wb >= 0. && wc >= 0.;
            if inside {
                let z = area / (wa * za + wb * zb + wc * zc);
                let off = p.y as usize * window.size().x as usize + p.x as usize;
                if zbuf[off] > z && z > -1. && z < 1. {
                    points.append(&Vertex::with_pos_color((p.x, p.y), color));
                    zbuf[off] = z;
                }
            }
            p.x = p.x + 1.;
        }
        p.y = p.y + 1.;
    }
}

pub fn render_normal(window: &mut RenderWindow, zbuf: &mut Vec<f32>, mesh: &Mesh, cam: &Camera) {
    let mut points = VertexArray::new(PrimitiveType::Points, 0);

    let size_u = window.size();
    let size_x = size_u.x as f32;
    let size_y = size_u.y as f32;

    // Process the object rotation matrix
    let obj_mat = mesh.get_mat();
    let cam_mat = cam.get_mat();
    let proj_mat = Matrix4::project(&cam);
    let m = proj_mat * cam_mat * obj_mat;

    // Process the coordinates of each point
    let mut cam_vertices : Vec<Vector3> = vec!();
    let mut scr_vertices : Vec<Vector3> = vec!();

    for v in &mesh.vertices {
        let (mut p, w) = &m * v.pt;
        // Perspective divide.
        p = p / w;
        cam_vertices.push(p);
        // To screen coordinates.
        p.x = (1. + p.x) * size_x / 2.;
        p.y = (1. - p.y) * size_y / 2.;
        scr_vertices.push(p);
    }

    for tri in &mesh.faces {
        let normal_col = Vector3::normal(&mesh.vertices[tri.a].pt,
            &mesh.vertices[tri.b].pt, &mesh.vertices[tri.c].pt);
        let color = Color::rgb(
            ((1. + normal_col.x) * 128.) as u8,
            ((1. + normal_col.y) * 128.) as u8,
            ((1. + normal_col.z.abs()) * 128.) as u8);

        let normal = Vector3::normal(&cam_vertices[tri.a],
            &cam_vertices[tri.b], &cam_vertices[tri.c]);
        if normal.z >= 0. {
            fill_triangle(window, &mut points, zbuf,
                scr_vertices[tri.a], scr_vertices[tri.b], scr_vertices[tri.c], color);
        } else {
            fill_triangle(window, &mut points, zbuf,
                scr_vertices[tri.a], scr_vertices[tri.c], scr_vertices[tri.b], color);
        }
    }

    window.clear(&Color::BLACK);
    window.draw(&points);
}

pub fn render_shadow(window: &mut RenderWindow, zbuf: &mut Vec<f32>, mesh: &Mesh, 
                     cam: &Camera, light: &Light) {

    let mut points = VertexArray::new(PrimitiveType::Points, 0);

    let size_u = window.size();
    let size_x = size_u.x as f32;
    let size_y = size_u.y as f32;

    // Process the object rotation matrix
    let obj_mat = mesh.get_mat();
    let cam_mat = cam.get_mat();
    let proj_mat = Matrix4::project(&cam);
    let m = proj_mat * cam_mat;
    
    // Process the coordinates of each point
    let mut obj_vertices : Vec<Vector3> = vec!();
    let mut cam_vertices : Vec<Vector3> = vec!();
    let mut scr_vertices : Vec<Vector3> = vec!();

    for v in &mesh.vertices {
        let (obj_pt, _) = &obj_mat * v.pt;
        obj_vertices.push(obj_pt);
        let (mut p, w) = &m * obj_pt;
        // Perspective divide.
        p = p / w;
        cam_vertices.push(p);
        // To screen coordinates.
        p.x = (1. + p.x) * size_x / 2.;
        p.y = (1. - p.y) * size_y / 2.;
        scr_vertices.push(p);
    }

    for tri in &mesh.faces {
        let normal_col = Vector3::normal(&obj_vertices[tri.a],
            &obj_vertices[tri.b], &obj_vertices[tri.c]);
        let mut shading = -light.dir.dot(&normal_col);
        let gray = ((1. + shading) * 128.) as u8;
        let color = Color::rgb(gray, gray, gray);

        let normal = Vector3::normal(&cam_vertices[tri.a],
            &cam_vertices[tri.b], &cam_vertices[tri.c]);
        if normal.z >= 0. {
            fill_triangle(window, &mut points, zbuf,
                scr_vertices[tri.a], scr_vertices[tri.b], scr_vertices[tri.c], color);
        } else {
            fill_triangle(window, &mut points, zbuf,
                scr_vertices[tri.a], scr_vertices[tri.c], scr_vertices[tri.b], color);
        }
    }

    window.clear(&Color::BLACK);
    window.draw(&points);

    
}

pub fn render_wireframe(window: &mut RenderWindow, mesh: &Mesh, cam: &Camera) {

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
        let (p, w) = &m * v.pt;
        let px = p.x / w;
        let py = p.y / w;
        proj_vert.push(((1. + px) * size_x / 2., (1. - py) * size_y / 2.))
    }

    for tri in &mesh.faces {
        vertex_array.append(&Vertex::with_pos(proj_vert[tri.a]));
        vertex_array.append(&Vertex::with_pos(proj_vert[tri.b]));
        vertex_array.append(&Vertex::with_pos(proj_vert[tri.b]));
        vertex_array.append(&Vertex::with_pos(proj_vert[tri.c]));
        vertex_array.append(&Vertex::with_pos(proj_vert[tri.c]));
        vertex_array.append(&Vertex::with_pos(proj_vert[tri.a]));
    }

    window.draw(&vertex_array);
}
