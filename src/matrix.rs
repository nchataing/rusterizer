use std::f32;
use std::f32::consts::PI;
use std::ops::Mul;
use crate::vector::Vector3;
use crate::camera::Camera;

pub struct Matrix {
    pub data: [[f32;4];4],
}

impl Matrix {

    pub fn identity() -> Matrix {
        Matrix {
            data: [[1., 0., 0., 0.],
                [0., 1., 0., 0.],
                [0., 0., 1., 0.],
                [0., 0., 0., 1.]
                ]
        }
    }

    pub fn rot_x(t:f32) -> Matrix {
        Matrix {
            data: [[1., 0., 0., 0.],
                [0., f32::cos(t), -f32::sin(t), 0.],
                [0., f32::sin(t), f32::cos(t), 0.],
                [0., 0., 0., 0.]
                ]
        }
    }

    pub fn rot_y(t:f32) -> Matrix {
        Matrix {
            data: [[f32::cos(t), -f32::sin(t), 0., 0.],
                [f32::sin(t), f32::cos(t), 0., 0.],
                [0., 0., 1., 0.],
                [0., 0., 0., 0.]
                ]
        }
    }

    pub fn rot_z(t:f32) -> Matrix {
        Matrix {
            data: [[f32::cos(t), 0., f32::sin(t), 0.],
                [0., 1., 0., 0.],
                [-f32::sin(t), 0., f32::cos(t), 0.],
                [0., 0., 0., 0.]
                ]
        }
    }

    pub fn rot(t_x:f32, t_y:f32, t_z:f32) -> Matrix {
        Matrix::rot_x(t_x) * Matrix::rot_y(t_y) * Matrix::rot_z(t_z)
    }

    pub fn translate(v: Vector3) -> Matrix {
        Matrix {
            data: [
                [1., 0., 0., v.x],
                [0., 1., 0., v.y],
                [0., 0., 1., v.z],
                [0., 0., 0., 1. ] 
            ]
        }
    }

    pub fn rot_and_translate(t_x:f32, t_y:f32, t_z:f32, v: Vector3) -> Matrix {
        let mut out = Matrix::rot(t_x, t_y, t_z);
        out.data[0][3] = v.x;
        out.data[1][3] = v.y;
        out.data[2][3] = v.z;
        out
    }

    pub fn project(cam: &Camera) -> Matrix {
        let f = cam.far;
        let n = cam.near;
        let s = 1. / f32::tan(cam.fov * PI / (360.));
        
        Matrix {
            data: [
                [s, 0., 0., 0.],
                [0., s, 0., 0.],
                [0., 0., f/(f-n), -f*n/(f-n)],
                [0., 0., -1., 0.]
            ]
        }
    }
}

impl Mul<Matrix> for Matrix {
    type Output = Matrix;

    fn mul(self, other: Matrix) -> Matrix {
        let mut out = Matrix::identity();
        
        for i in 0..3{
            for j in 0..3 {
                for k in 0..3 {
                    out.data[i][j] += self.data[i][k] * other.data[k][j];
                }
            }
        }

        out
    }
}

impl Mul<Vector3> for &Matrix {
    type Output = Vector3;

    fn mul(self, v: Vector3) -> Vector3 {
        let w = v.x * self.data[3][0] + v.y * self.data[3][1] + v.z * self.data[3][2] + self.data[3][3];
        
        let not_norm = Vector3 {
            x: v.x * self.data[0][0] + v.y * self.data[0][1] + v.z * self.data[0][2] + self.data[0][3],
            y: v.x * self.data[1][0] + v.y * self.data[1][1] + v.z * self.data[1][2] + self.data[1][3],
            z: v.x * self.data[2][0] + v.y * self.data[2][1] + v.z * self.data[2][2] + self.data[2][3],
        };

        not_norm / w
    }
}
