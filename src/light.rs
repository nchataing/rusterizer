use crate::vector::Vector3;
use sfml::graphics::Color;

pub struct Light {
    pub dir: Vector3,
//  pub color: Color,
//  pub intensity: f32,
}
impl Light {
    pub fn new(x: f32, y: f32, z: f32) -> Light {
        Light {
            dir : Vector3::new(x,y,z).normalize()
        }
    }
}
