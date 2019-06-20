
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

use sfml::graphics::Color;
use crate::vector::Vector3;
use crate::mesh::*;

macro_rules! face {
    ($a:expr, $b:expr, $c:expr) => (Face { a: $a, b: $b, c: $c, color: None })
}

pub fn import(f: &mut File, mesh: &mut Mesh) -> Option<()> {
    let reader = BufReader::new(f);
    let mut lines = reader.lines()
        .map(|s| s.unwrap())
        .filter(|s| s.len() > 0)
        .filter(|s| s.as_bytes()[0] != b'#');

    let fmt = lines.next()?;
    if fmt != "OFF" {
        println!("unrecognized format [{}]", fmt);
        return None;
    }

    let sizes: Vec<usize> = lines.next()?
        .split(' ')
        .filter_map(|s| s.parse::<usize>().ok())
        .collect();

    if sizes.len() != 3 {
        println!("invalid sizes");
        return None;
    }

    let (nr_vertices, nr_faces, nr_edges) = (sizes[0], sizes[1], sizes[2]);

    println!("preparing to load [{}] vertices", nr_vertices);

    for _i in 0 .. nr_vertices {
        let coords: Vec<f32> = lines.next()?
            .split(' ')
            .filter_map(|s| s.parse::<f32>().ok())
            .collect();

        let vertex =
            if coords.len() == 3 {
                Vertex {
                    pt: Vector3 { x:coords[0], y:coords[1], z:coords[2] },
                    color: None
                }
            } else if coords.len() == 7 {
                Vertex {
                    pt: Vector3 { x:coords[0], y:coords[1], z:coords[2] },
                    color: None /*Some (color!(coords[3], coords[4], coords[5]))*/
                }
            } else {
                return None;
            };

        mesh.vertices.push(vertex);
    }

    println!("loaded {} vertices", mesh.vertices.len());
    println!("preparing to load [{}] faces", nr_faces);

    for _i in 0 .. nr_faces {
        let l = lines.next()?;
        let verts: Vec<u64> = l.split(' ')
            .filter_map(|s| s.parse::<u64>().ok())
            .collect();

        if verts.len() == 0 {
            println!("empty face line");
            return None;
        }

        let nr_verts = verts[0] as usize;
        if verts.len() < nr_verts + 1 || nr_verts < 3 {
            println!("invalid vertex count [{}]", nr_verts);
            return None;
        }

        for f in 0 .. nr_verts - 2 {
            let a = verts[1] as usize;
            let b = verts[f + 2] as usize;
            let c = verts[f + 3] as usize;

            if a >= mesh.vertices.len() ||
               b >= mesh.vertices.len() ||
               c >= mesh.vertices.len() {
                println!("invalid vertex indexes [{}, {}, {}]", a, b, c);
                return None;
            }

            let face = face!(a, b, c);
            mesh.faces.push(face);
        }
    }

    println!("loaded {} faces\n", mesh.faces.len());

    Some(())
}

/*
https://people.sc.fsu.edu/%7Ejburkardt/data/off/off.html

 OFF is a data directory which contains examples of OFF files. An OFF file is good for storing a description a 2D or 3D object constructed from polygons. There is even a simple extension which can handle objects in 4D.
OFF File Characteristics:

    ASCII (there is also a binary version);
    Color optional;
    3D;
    No compression;

While there are many variations and extensions of the OFF format, the simplest files have the following structure:

Line 1
    OFF
Line 2
    vertex_count face_count edge_count
One line for each vertex:
    x y z
    for vertex 0, 1, ..., vertex_count-1
One line for each polygonal face:
    n v1 v2 ... vn,
    the number of vertices, and the vertex indices for each face.

The vertices are implicitly numbered by the order of their listing in the file. The first vertex will have index 0, and the last will have index vertex_count-1. Comment lines may be inserted anywhere, and are denoted by the character # appearing in column 1. Blank lines are also acceptable.

Colors can be assigned to the vertices by following the XYZ coordinates of a vertex with the RGBA color coordinates (the "A" coordinate controls the transparency). In particular, this means that the form of the vertex lines would be:

One line for each colored vertex:
    x y z r g b a
    for vertex 0, 1, ..., vertex_count-1

Colors can be assigned to the faces by following the vertex indices by the RGBA color coordinates of the face. In particular, this means that the form of the face lines would be:

One line for each polygonal face:
    n v1 v2 ... vn r g b a,
    the number of vertices, the vertex indices, and the RGBA color coordinates for each face.

Normally, color is not specified for BOTH the vertices and faces. If the node colors are specified, then the faces are colored with a smooth interpolation of the colors of their vertices.
Example OFF file:

OFF
#
#  cube.off
#  A cube.
#  There is extra RGBA color information specified for the faces.
#
8 6 12
  1.632993   0.000000   1.154701
  0.000000   1.632993   1.154701
 -1.632993   0.000000   1.154701
  0.000000  -1.632993   1.154701
  1.632993   0.000000  -1.154701
  0.000000   1.632993  -1.154701
 -1.632993   0.000000  -1.154701
  0.000000  -1.632993  -1.154701
  4  0 1 2 3  1.000 0.000 0.000 0.75
  4  7 4 0 3  0.300 0.400 0.000 0.75
  4  4 5 1 0  0.200 0.500 0.100 0.75
  4  5 6 2 1  0.100 0.600 0.200 0.75
  4  3 2 6 7  0.000 0.700 0.300 0.75
  4  6 5 4 7  0.000 1.000 0.000 0.75
*/
