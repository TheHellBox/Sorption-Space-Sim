pub mod controls;
pub mod cabin;
pub mod area_checker;

use universe;
use render;

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

        let (cabin_pos, rotation, area) = cabin::cabin_update(&mut self.universe, window);
        let camera_rotation = rotation;
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
