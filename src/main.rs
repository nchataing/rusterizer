extern crate sfml;

mod io;
mod mesh;
mod renderer;
#[macro_use]
mod vector;
mod matrix;
mod camera;

use io::off::import;
use std::fs::File;

use sfml::graphics::{RenderWindow, RenderTarget};
use sfml::window::{Event, Key, Style};
use sfml::system::Vector2i;

use renderer::*;
use mesh::*;
use camera::Camera;
use vector::Vector3;

use std::f32;

fn main() {

    let mut mesh = Mesh::new();
    let mesh_file = "objects/cube.off";
    let mut file = File::open(mesh_file).unwrap();

    let res = import(&mut file, &mut mesh);
    if res == None {
        eprint!("Failed to import mesh from file {}", mesh_file);
        return;
    }

    // Rotate the mesh and translate it
    mesh.translate(Vector3::new(0.,0.,-2.));
    mesh.rot_x(-f32::consts::PI/2.);

    let mut camera = Camera::new();
    let mut window = RenderWindow::new(
        (800,600),
        "Dot",
        Style::CLOSE,
        &Default::default(),
    );
    window.set_vertical_sync_enabled(true);
    window.set_framerate_limit(60);

    // Create empty Z-buffer
    let mut zbuf = vec![std::f32::MIN; (window.size().x * window.size().y) as usize];

    let mut mx = 400;
    let mut my = 300;

    window.set_mouse_position(&Vector2i::new(400,300));

    println!("{} =? {}", f32::consts::PI/4., f32::atan(1.));

    let mut paused = false;
    let mut mouse_left = false;

    loop {
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed | Event::KeyPressed { code : Key::Escape, .. }
                    => return,
                Event::KeyPressed { code: Key::Space, .. } =>
                    paused = !paused,
                _ => {}
            }
        }

        if paused {
            continue
        }

        let mp = window.mouse_position();
        let dx = (mp.x - mx) as f32;
        let dy = (mp.y - my) as f32;
        mx = mp.x;
        my = mp.y;

        //camera.rot_z(dx/20.);
        //camera.rot_x(dy/20.);

        render_raster(&mut window, &mut zbuf, &mesh, &camera);
        window.display();
        mesh.rot_y(f32::consts::PI/180.)
    }
}
