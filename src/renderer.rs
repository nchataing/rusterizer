use sfml::graphics::{Color, Vertex, VertexArray, PrimitiveType, RenderWindow, RenderTarget};
use sfml::system::{Vector2f};
use crate::mesh::Mesh;
use crate::camera::Camera;
use crate::vector::*;
use crate::matrix::*;
use std::f32::consts::PI;
use std::f32;
use std::cmp::Ordering;

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

pub fn put_pixel(window: &RenderWindow,
                 points: &mut VertexArray,
                 zbuf: &mut Vec<f32>,
                 x: f32, y: f32, z: f32,
                 color: Color) {
    if x < 0. || x as u32 >= window.size().x {
        return;
    }

    let off = y as usize * window.size().x as usize+ x as usize;
    if zbuf[off] > z {
        points.append(&Vertex::with_pos_color((x, y), color));
        zbuf[off as usize] = z;
    }
}

pub fn fill_line(window: &RenderWindow,
                 points: &mut VertexArray,
                 zbuf: &mut Vec<f32>,
                 xs: f32, zs: f32, xe: f32, ze: f32, y: f32,
                 color: Color) {
    if y < 0. || y as u32 >= window.size().y {
        return;
    }

    let dz = if xs != xe { (ze - zs) / (xe - xs) } else { 0. };
    let mut x = xs;
    let mut z = zs;
    while x <= xe {
        x += 1.;
        z += dz;
        put_pixel(window, points, zbuf, x, y, z, color);
    }
}

pub fn fill_triangle(window: &RenderWindow,
                     points: &mut VertexArray,
                     zbuf: &mut Vec<f32>,
                     va: Vector3,
                     vb: Vector3,
                     vc: Vector3,
                     color: Color) {

    let mut vs = [va, vb, vc];
    vs.sort_by(|a, b| a.y.partial_cmp(&b.y).unwrap_or(Ordering::Equal));
    let [a, b, c] = vs;

    let (dx1, dz1) =
        if b.y > a.y {
            ((b.x - a.x) / (b.y - a.y),
             (b.z - a.z) / (b.y - a.y))
        } else {
            (0., 0.)
        };
    let (dx2, dz2) =
        if c.y > a.y {
            ((c.x - a.x) / (c.y - a.y),
             (c.z - a.z) / (c.y - a.y))
        } else {
            (0., 0.)
        };
    let (dx3, dz3) =
        if c.y > b.y {
            ((c.x - b.x) / (c.y - b.y),
             (c.z - b.z) / (c.y - b.y))
        } else {
            (0., 0.)
        };

    let mut xs = a.x;
    let mut zs = a.z;
    let mut xe = a.x;
    let mut ze = a.z;
    let mut y = a.y;

    if dx1 > dx2 {
        while y <= b.y {
            fill_line(window, points, zbuf, xs, zs, xe, ze, y, color);
            xs += dx2;
            zs += dz2;
            xe += dx1;
            ze += dz1;
            y += 1.;
        }
        xe = b.x;
        while y <= c.y {
            fill_line(window, points, zbuf, xs, zs, xe, ze, y, color);
            xs += dx2;
            zs += dz2;
            xe += dx3;
            ze += dz3;
            y += 1.;
        }
    } else {
        while y <= b.y {
            fill_line(window, points, zbuf, xs, zs, xe, ze, y, color);
            xs += dx1;
            zs += dz1;
            xe += dx2;
            ze += dz2;
            y += 1.;
        }
        xs = b.x;
        while y <= c.y {
            fill_line(window, points, zbuf, xs, zs, xe, ze, y, color);
            xs += dx3;
            zs += dz3;
            xe += dx2;
            ze += dz2;
            y += 1.;
        }
    }
}

pub fn render_raster(window: &mut RenderWindow, zbuf: &mut Vec<f32>, mesh: &Mesh, cam: &Camera) {
    let mut points = VertexArray::new(PrimitiveType::Points, 0);

    let size_u = window.size();
    let size_x = size_u.x as f32;
    let size_y = size_u.y as f32;

    // Clear the Z-buffer
    for c in zbuf.iter_mut() {
        *c = std::f32::MAX;
    }

    // Process the object rotation matrix
    let obj_mat = mesh.get_mat();
    let cam_mat = cam.get_mat();
    let proj_mat = Matrix4::project(&cam);
    let m = proj_mat * cam_mat * obj_mat;

    // Process the coordinates of each point
    let mut proj_vert : Vec<Vector3> = vec!();
    for v in &mesh.vertices {
        let mut proj = &m * v.pt;
        proj.x = (1. + proj.x) * size_x / 2.;
        proj.y = (1. - proj.y) * size_y / 2.;
        proj_vert.push(proj);
    }

    for tri in &mesh.faces {
        let norm = Vector3::normal(&mesh.vertices[tri.a].pt,
            &mesh.vertices[tri.b].pt, &mesh.vertices[tri.c].pt);
        let color = Color::rgb(
            ((1. + norm.x) * 128.) as u8,
            ((1. + norm.y) * 128.) as u8,
            ((1. + norm.z.abs()) * 128.) as u8);
        fill_triangle(window, &mut points, zbuf,
            proj_vert[tri.a], proj_vert[tri.b], proj_vert[tri.c], color)
    }

    window.clear(&Color::BLACK);
    window.draw(&points)
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

    window.draw(&vertex_array);
}
