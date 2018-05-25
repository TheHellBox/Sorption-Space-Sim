
use universe::gameobject::GameObject;
use universe::game::controls::Controls;
use universe::Universe;

use nalgebra::geometry::{Point3, UnitQuaternion, Quaternion};

use render::Window;
use player::Player;
use std::cmp::Ordering;

pub fn cabin_update(universe: &mut Universe, window: &mut Window) -> (Point3<f32>, Quaternion<f32>, (i32, i32)){

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

            let rot = rotation.lerp(&UnitQuaternion::from_euler_angles((controls.rel.1 / 100.0), (controls.rel.0 / 100.0), controls.roll), 0.04);
            let mut cabin_pos = Point3::new(position[0] + direcion[0], position[1] + direcion[1], position[2] + direcion[2]);

            let mut area = (0, 0);

            if cabin_pos[0] > 10000.0{
                area.0 += 1;
                cabin_pos[0] = -10000.0;
            }
            if cabin_pos[0] < -10000.0{
                area.0 -= 1;
                cabin_pos[0] = 10000.0;
            }

            if cabin_pos[2] > 10000.0{
                area.1 += 1;
                cabin_pos[2] = -10000.0;
            }
            if cabin_pos[2] < -10000.0{
                area.1 -= 1;
                cabin_pos[2] = 10000.0;
            }
            cabin.set_rotation(rot);
            cabin.set_position(cabin_pos);
            (cabin_pos, rot, area)
        },
        _ => {
            (Point3::new(0.0, 0.0, 0.0), Quaternion::new(0.0,0.0,1.0,0.0), (0, 0))
        }
    }
}
