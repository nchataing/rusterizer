use crate::vector::Vector3;
use crate::matrix::Matrix4;

pub struct Camera {
    pub translation: Vector3,
    pub near: f32,
    pub far: f32,
    pub fov: f32,
    pub rot_x: f32,
    pub rot_y: f32,
    pub rot_z: f32,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            translation : Vector3::zero(),
            rot_x       : 0.,
            rot_y       : 0.,
            rot_z       : 0.,
            near        : 0.1,
            far         : 20.,
            fov         : 90.,
        }
    }

    pub fn rot_x(&mut self, t: f32) {
        self.rot_x += t
    }

    pub fn rot_y(&mut self, t: f32) {
        self.rot_y += t
    }

    pub fn rot_z(&mut self, t: f32) {
        self.rot_z += t
    }

    pub fn translate(&mut self, vec: Vector3) {
        self.translation = self.translation + vec
    }

    pub fn get_direction(&self) -> Vector3 {
        let (v,w) = &self.get_mat() * Vector3::new(0.,0.,1.);
        v.normalize()
    }

    pub fn get_mat(&self) -> Matrix4 {
        Matrix4::rot_and_translate(self.rot_x, self.rot_y, self.rot_z, self.translation)
    }
}
