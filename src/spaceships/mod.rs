use nalgebra::geometry::{Quaternion, Point3};

pub enum Ship{
    Simple = 0
}

struct Ship{
    pub position: Point3<f32>,
    pub rotation: Quartenion<f32>,
    pub ship: u32
}
