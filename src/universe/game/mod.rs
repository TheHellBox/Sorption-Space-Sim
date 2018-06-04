pub mod controls;
pub mod cabin;
pub mod area_checker;

use universe;
use render;
use nalgebra::geometry::{Point3, UnitQuaternion};

pub struct Game{
    pub universe: universe::Universe
}
impl Game{
    pub fn new(window: &mut render::Window) -> Game{
        let mut universe = universe::Universe::new([0, 1, 2, 3]);
        universe.init(window);
        Game{
            universe
        }
    }
    pub fn update(&mut self, window: &mut render::Window){
        self.universe.update(window);
        // Copy controls info
        let controls = self.universe.controls;

        let (mut cabin_pos, rotation, area) = cabin::cabin_update(&mut self.universe, window);
        let camera_rotation = rotation * UnitQuaternion::from_euler_angles((window.mouse.releative.1 as f32 / 100.0), (window.mouse.releative.0 as f32 / 100.0), controls.roll).inverse();
        //Set camera pos/rot
        window.draw_context.camera.set_pos(cabin_pos);
        window.draw_context.camera.set_rot(camera_rotation);

        // Set background position
        match self.universe.get_go_by_name("Background".to_string()){
            Some(ref mut bg) => {
                bg.set_position(cabin_pos);
            },
            _ => {}
        }

        if area != (0, 0){
            area_checker::check_area(area, &mut self.universe);
        }

        // Call Update on objects
        let objects = &mut self.universe.objects;
        for (_, x) in objects{
            x.update()
        }

        match self.universe.player{
            Some(ref mut x) => {
                x.area.0 += area.0;
                x.area.1 += area.1;
            }
            None => {
                ()
            }
        }
    }
}
