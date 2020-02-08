extern crate sfml;

mod io;
mod mesh;
mod renderer;
#[macro_use]
mod vector;
mod matrix;
mod camera;
mod light;

use io::off::import;
use std::fs::File;
use std::env;

use sfml::graphics::{RenderWindow, RenderTarget};
use sfml::window::{Event, Key, mouse::Button, Style, mouse::Wheel};
use sfml::system::Vector2i;

use renderer::*;
use mesh::*;
use camera::Camera;
use vector::Vector3;
use light::Light;

use std::f32;

fn main() {

    let args: Vec<_> = env::args().collect();
    let mut mesh_file = "";

    if args.len() == 1 {
        print!("Expected OBJ file path as argument\n");
        return;
    } else {
        mesh_file = &args[1]
    }

    let mut mesh = Mesh::new();
    let mut file = File::open(mesh_file).unwrap();

    let res = import(&mut file, &mut mesh);
    if res == None {
        eprint!("Failed to import mesh from file {}\n", mesh_file);
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

    let light = Light::new(1.,-1.,-1.);

    // Create empty Z-buffer
    let mut zbuf = vec![std::f32::MAX; (width * height) as usize];

    let mut paused = false;
    let mut mouse_left = false;
    let mut rotate = false;
    let mut prev_mp = Vector2i::new(0, 0);

    loop {
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed | Event::KeyPressed { code: Key::Escape, .. } =>
                    return,
                Event::KeyPressed { code: Key::Space, .. } =>
                    paused = !paused,
                Event::MouseButtonPressed { button: Button::Left, x, y } => {
                    prev_mp.x = x;
                    prev_mp.y = y;
                    rotate = true
                },
                Event::MouseButtonReleased { button: Button::Left, .. } =>
                    rotate = false,
                Event::MouseWheelScrolled {wheel: Wheel::Vertical, delta, ..} =>
                    mesh.translate(Vector3::new(0.,0.,delta)),
                _ => {},
            }
        }

        if paused {
            continue
        }

        if rotate {
            let mp = window.mouse_position();
            mesh.rot_y((mp.x - prev_mp.x) as f32 * 0.005);
            mesh.rot_x((mp.y - prev_mp.y) as f32 * 0.005);
            prev_mp = mp
        }

        // Clear the Z-buffer.
        for c in zbuf.iter_mut() {
            *c = std::f32::MAX;
        }

        render_shadow(&mut window, &mut zbuf, &mesh, &camera, &light);
        window.display();
    }
}
