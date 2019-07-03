use std::f32;
use std::f32::consts::PI;
use std::ops::Mul;
use crate::vector::Vector3;
use crate::camera::Camera;

pub struct Matrix4 {
    pub cells: [[f32; 4]; 4],
}

macro_rules! matrix {
    [ $x:expr ] => (Matrix4 { cells: [[$x; 4]; 4] });
    [ $a00:expr, $a01:expr, $a02:expr, $a03:expr;
      $a10:expr, $a11:expr, $a12:expr, $a13:expr;
      $a20:expr, $a21:expr, $a22:expr, $a23:expr;
      $a30:expr, $a31:expr, $a32:expr, $a33:expr ] => (Matrix4 {
            cells: [
                [$a00, $a01, $a02, $a03],
                [$a10, $a11, $a12, $a13],
                [$a20, $a21, $a22, $a23],
                [$a30, $a31, $a32, $a33]
            ]
        })
}

impl Matrix4 {

    pub fn identity() -> Matrix4 {
        matrix![
            1., 0., 0., 0.;
            0., 1., 0., 0.;
            0., 0., 1., 0.;
            0., 0., 0., 1.
        ]
    }

    pub fn rot_x(t: f32) -> Matrix4 {
        matrix![
            1., 0.,      0.,       0.;
            0., t.cos(), -t.sin(), 0.;
            0., t.sin(), t.cos(),  0.;
            0., 0.,      0.,       0.
        ]
    }

    pub fn rot_y(t: f32) -> Matrix4 {
        matrix![
            t.cos(), -t.sin(), 0., 0.;
            t.sin(), t.cos(),  0., 0.;
            0.,      0.,       1., 0.;
            0.,      0.,       0., 0.
        ]
    }

    pub fn rot_z(t: f32) -> Matrix4 {
        matrix![
            t.cos(),  0., t.sin(), 0.;
            0.,       1., 0.,      0.;
            -t.sin(), 0., t.cos(), 0.;
            0.,       0., 0.,      0.
        ]
    }

    pub fn rot(t_x: f32, t_y: f32, t_z: f32) -> Matrix4 {
        Matrix4::rot_x(t_x) * Matrix4::rot_y(t_y) * Matrix4::rot_z(t_z)
    }

    pub fn translate(v: Vector3) -> Matrix4 {
        matrix![
            1., 0., 0., v.x;
            0., 1., 0., v.y;
            0., 0., 1., v.z;
            0., 0., 0., 1.
        ]
    }

    pub fn rot_and_translate(t_x:f32, t_y:f32, t_z:f32, v: Vector3) -> Matrix4 {
        let mut out = Matrix4::rot(t_x, t_y, t_z);
        out.cells[0][3] = v.x;
        out.cells[1][3] = v.y;
        out.cells[2][3] = v.z;
        out
    }

    pub fn project(cam: &Camera) -> Matrix4 {
        let f = cam.far;
        let n = cam.near;
        let s = 1. / f32::tan(cam.fov * PI / (360.));

        matrix![
            s,  0., 0.,      0.;
            0., s,  0.,      0.;
            0., 0., f/(f-n), -f*n/(f-n);
            0., 0., -1.,     0.
        ]
    }
}

impl Mul<Matrix4> for Matrix4 {
    type Output = Matrix4;

    fn mul(self, other: Matrix4) -> Matrix4{
        let mut out = Matrix4::identity();

        for i in 0..3{
            for j in 0..3 {
                for k in 0..3 {
                    out.cells[i][j] += self.cells[i][k] * other.cells[k][j];
                }
            }
        }

        out
    }
}

impl Mul<Vector3> for &Matrix4{
    type Output = Vector3;

    fn mul(self, v: Vector3) -> Vector3 {
        let w = v.x * self.cells[3][0] + v.y * self.cells[3][1] + v.z * self.cells[3][2] + self.cells[3][3];

        let not_norm = Vector3 {
            x: v.x * self.cells[0][0] + v.y * self.cells[0][1] + v.z * self.cells[0][2] + self.cells[0][3],
            y: v.x * self.cells[1][0] + v.y * self.cells[1][1] + v.z * self.cells[1][2] + self.cells[1][3],
            z: v.x * self.cells[2][0] + v.y * self.cells[2][1] + v.z * self.cells[2][2] + self.cells[2][3],
        };

        not_norm / w
    }
}
