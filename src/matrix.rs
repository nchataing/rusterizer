use std::f32;
use std::ops::Mul;
use crate::vector::Vector3;

pub struct Matrix {
    pub data: [[f32;3];3],
}

pub fn identity() -> Matrix {
    Matrix {
        data: [[1., 0., 0.],
               [0., 1., 0.],
               [0., 0., 1.]
              ]
    }
}

pub fn rot_x(t:f32) -> Matrix {
    Matrix {
        data: [[1., 0., 0.],
               [0., f32::cos(t), -f32::sin(t)],
               [0., f32::sin(t), f32::cos(t)]
              ]
    }
}

pub fn rot_z(t:f32) -> Matrix {
    Matrix {
        data: [[f32::cos(t), 0., f32::sin(t)],
               [0., 1., 0.],
               [-f32::sin(t), 0., f32::cos(t)]
              ]
    }
}

pub fn rot_y(t:f32) -> Matrix {
    Matrix {
        data: [[f32::cos(t), -f32::sin(t), 0.],
               [f32::sin(t), f32::cos(t), 0.],
               [0., 0., 1.]
              ]
    }
}

pub fn mult_m(a: Matrix, b: Matrix) -> Matrix {
    let mut out = Matrix {
        data: [[0., 0., 0.],
                [0., 0., 0.],
                [0., 0., 0.]
                ]
    };

    for i in 0..3{
        for j in 0..3 {
            for k in 0..3 {
                out.data[i][j] += a.data[i][k] * b.data[k][j];
            }
        }
    }

    out
}

pub fn rot(t_x:f32, t_y:f32, t_z:f32) -> Matrix {
    mult_m(rot_x(t_x), mult_m(rot_y(t_y), rot_z(t_z)))
}


impl Mul<Vector3> for &Matrix {
    type Output = Vector3;

    fn mul(self, v: Vector3) -> Vector3 {
        Vector3 {
            x: v.x * self.data[0][0] + v.y * self.data[0][1] + v.z * self.data[0][2],
            y: v.x * self.data[1][0] + v.y * self.data[1][1] + v.z * self.data[1][2],
            z: v.x * self.data[2][0] + v.y * self.data[2][1] + v.z * self.data[2][2],
        }
    }
}
