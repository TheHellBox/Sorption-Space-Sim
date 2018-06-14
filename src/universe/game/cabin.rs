
use universe::Universe;

use nalgebra::geometry::{Point3, UnitQuaternion, Quaternion};

use render::Window;

pub fn cabin_update(universe: &mut Universe, window: &mut Window) -> (Point3<f32>, UnitQuaternion<f32>, (i32, i32)){

    let controls = universe.controls;

    let forward = controls.forward - controls.back;
    let right = controls.right - controls.left;
    let up = controls.up - controls.down;

    match universe.get_go_by_name("Cabin".to_string()){
        Some(ref mut cabin) => {
            let rotation = cabin.rotation;
            let position = cabin.position;
            let forward = (cabin.forward() / 20.0) * forward;
            let right = (cabin.right() / 20.0) * right;
            let up = (cabin.up() / 20.0) * up;
            let direcion = forward + right + up;

            if window.focused == true{
                cabin.rotation = rotation * UnitQuaternion::from_euler_angles(window.mouse.releative.1 as f32 / 100.0, window.mouse.releative.0 as f32 / 100.0, controls.roll).inverse();
            }
            
            let mut cabin_pos = Point3::new(position[0] + direcion[0], position[1] + direcion[1], position[2] + direcion[2]);

            let mut area = [0, 0, 0];

            for x in 0..3{
                if cabin_pos[x] > 10000.0{
                    area[x] += 1;
                    cabin_pos[x] = -10000.0;
                }
                if cabin_pos[x] < -10000.0{
                    area[x] -= 1;
                    cabin_pos[x] = 10000.0;
                }
            }
            cabin.set_position(cabin_pos);
            (cabin_pos, cabin.rotation, (area[0], area[1]))
        },
        _ => {
            (Point3::new(0.0, 0.0, 0.0), UnitQuaternion::from_quaternion(Quaternion::new(0.0,0.0,1.0,0.0)), (0, 0))
        }
    }
}
