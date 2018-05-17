
use universe::gameobject::Game_Object;
use universe::game::controls::Controls;

use nalgebra::geometry::{Point3, UnitQuaternion, Quaternion};

use render::Window;

pub fn cabin_update(cabin: &mut Game_Object, window: &mut Window, controls: &Controls) -> (Point3<f32>, Quaternion<f32>){
    let forward = controls.forward - controls.back;
    let right = controls.right - controls.left;
    let up = controls.up - controls.down;

    let rotation = cabin.rotation;
    let position = cabin.position;
    let forward = (cabin.forward() / 20.0) * forward;
    let right = (cabin.right() / 20.0) * right;
    let up = (cabin.up() / 20.0) * up;
    let direcion = forward + right + up;

    let rot = rotation.lerp(&UnitQuaternion::from_euler_angles(0.0, -(window.mouse_pos.0 as f32 / 100.0), 0.0), 0.04);
    let cabin_pos = Point3::new(position[0] + direcion[0], position[1] + direcion[1], position[2] + direcion[2]);

    cabin.set_rotation(rot);
    cabin.set_position(cabin_pos);
    (cabin_pos, rot)
}
