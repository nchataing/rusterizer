extern crate sfml;

mod io;
mod mesh;
mod renderer;
#[macro_use]
mod vector;
mod matrix;

use io::off::import;
use std::fs::File;

use sfml::graphics::RenderWindow;
use sfml::window::{Event, Key, Style};

use renderer::*;
use mesh::*;
use vector::Vector3;

use std::f32;

fn main() {
/*
    let mut mesh = Mesh::new();
    let file_name = "objects/teapot.off";
    let mut file = File::open(file_name).unwrap();
    
    let res = import(&mut file,&mut mesh);
    if res == None {
        eprint!("Failed to import mesh from file {}", file_name);
        return;
    }
    
    mesh.offset(vector!(0.0,0.0,-2.0));

     
    let image_buf = &render_par(&scene);

    let mut file = File::create("img.ppm").unwrap();
    let header = format!("P6 {} {} 255\n", scene.width, scene.height);
    file.write(header.as_bytes());
    file.write(image_buf);
*/

    let mut mesh = Mesh::new();
    let mesh_file = "objects/teapot.off";
    let mut file = File::open(mesh_file).unwrap();

    let res = import(&mut file, &mut mesh);
    if res == None {
        eprint!("Failed to import mesh from file {}", mesh_file);
        return;
    }
    
    // Rotate the mesh and translate it
    mesh.set_translation(Vector3::new(0.,0.,-3.));
    mesh.rot_x(-f32::consts::PI/2.);

    let mut window = RenderWindow::new(
        (800,600),
        "Dot",
        Style::CLOSE,
        &Default::default(),
    );
    window.set_vertical_sync_enabled(true);
    window.set_framerate_limit(60); 
    loop {
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed | Event::KeyPressed { code : Key::Escape, .. }
                    => return,
                _ => {}
            }
        }
        render_mesh(&mut window, &mesh);
        window.display();
        mesh.rot_y(f32::consts::PI/180.)
    }
}
