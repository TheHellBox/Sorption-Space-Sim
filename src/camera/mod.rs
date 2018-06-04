
use nalgebra::geometry::{Perspective3, UnitQuaternion, Quaternion, Translation3, Point3};
use nalgebra::{Matrix4};
pub struct Camera{
    pub perspective: Perspective3<f32>,
    pub position: Translation3<f32>,
    pub rotation: UnitQuaternion<f32>,
}

impl Camera{
    pub fn new(sx: u32, sy: u32) -> Camera{
        let perspective = Perspective3::new((sx as f32 / sy as f32), 3.14 / 2.0, 0.01, 200000.0);

        let position = Translation3::new(0.0,0.0,0.0);
        let rotation = Point3::new(0.0,0.0,0.0);

        Camera{
            perspective: perspective,
            position: position,
            rotation: UnitQuaternion::from_quaternion(Quaternion::new(0.0,0.0,1.0,0.0)),
        }
    }
    pub fn set_rot(&mut self, rot: UnitQuaternion<f32>){
        self.rotation = rot.inverse();
    }
    pub fn set_pos(&mut self, point: Point3<f32>){
        self.position.vector = -point.coords;
    }
    pub fn view(&self) -> Matrix4<f32>{
        let translation_matrix: Matrix4<f32> = self.position.to_homogeneous();
        let rotation: Matrix4<f32> = self.rotation.to_homogeneous();
        let mat = (rotation * translation_matrix);
        mat
    }
}
