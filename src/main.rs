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
use sfml::window::{Event, Key, mouse::Button, Style};
use sfml::system::Vector2i;

use renderer::*;
use mesh::*;
use camera::Camera;
use vector::Vector3;

use std::f32;

fn main() {

    let mut mesh = Mesh::new();
    let mesh_file = "objects/space_station.off";
    let mut file = File::open(mesh_file).unwrap();

    let res = import(&mut file, &mut mesh);
    if res == None {
        eprint!("Failed to import mesh from file {}", mesh_file);
        return;
    }

    let width: u32 = 800;
    let height: u32 = 600;

    // Rotate the mesh and translate it
    mesh.translate(Vector3::new(0.,0.,-12.));
    mesh.rot_x(-f32::consts::PI/2.);

    let mut camera = Camera::new();
    let mut window = RenderWindow::new(
        (width, height),
        "Dot",
        Style::CLOSE,
        &Default::default(),
    );
    window.set_vertical_sync_enabled(true);
    window.set_framerate_limit(60);
    window.set_mouse_position(&Vector2i::new(width as i32 / 2, height as i32 / 2));

    // Create empty Z-buffer
    let mut zbuf = vec![std::f32::MIN; (width * height) as usize];

    let mut paused = false;
    let mut mouse_left = false;
    let mut start_coords = None;

    loop {
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed | Event::KeyPressed { code: Key::Escape, .. } =>
                    return,
                Event::KeyPressed { code: Key::Space, .. } =>
                    paused = !paused,
                Event::MouseButtonPressed { button: Button::Left, x, y } =>
                    start_coords = Some ((x as f32, y as f32)),
                Event::MouseButtonReleased { button: Button::Left, .. } =>
                    start_coords = None,
                _ => {}
            }
        }

        if paused {
            continue
        }

        match start_coords {
            Some ((sx, sy)) => {
                let mp = window.mouse_position();
                mesh.rot_y((mp.x as f32 - sx) * 0.001);
                mesh.rot_x((mp.y as f32 - sy) * 0.001);
            },
            _ => {}
        }

        // Clear the Z-buffer.
        for c in zbuf.iter_mut() {
            *c = std::f32::MAX;
        }

        render_raster(&mut window, &mut zbuf, &mesh, &camera);
        window.display();
    }
}
