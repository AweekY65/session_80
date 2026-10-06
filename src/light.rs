use crate::vec3::{Color, Point3};

#[derive(Clone, Copy, Debug)]
pub struct PointLight {
    pub position: Point3,
    pub color: Color,
}

impl PointLight {
    pub fn new(position: Point3, color: Color) -> Self {
        PointLight { position, color }
    }
}
