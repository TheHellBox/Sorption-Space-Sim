pub mod star;
pub mod planet;
pub mod gameobject;

use glium;
use render;
use player;
use nalgebra::geometry::{Point3, UnitQuaternion, Quaternion};
use std::collections::HashMap;

use self::gameobject::Game_Object;

// Universe, place where you can exist
pub struct Universe{
    pub seed: [usize; 4],
    pub player: Option<player::Player>,
    //              ID, the object itself
    pub objects: HashMap<u32, Game_Object>
}

impl Universe{
    // Creating universe
    pub fn new(seed: [usize; 4]) -> Universe{
        Universe{
            seed: seed,
            player: None,
            objects: HashMap::new()
        }
    }
    // Set player to the universe
    pub fn set_player(&mut self, player: player::Player){
        self.player = Some(player);
    }
    // Generate star with seed of the universe
    pub fn get_star(&mut self, coords: Point3<usize>) -> Option<star::Star>{
        star::Star::gen(coords, &self.seed)
    }
    // Prepare universe for playing
    pub fn init(&mut self, window: &mut render::Window){

        //Add default items to player's inventory
        match self.player{
            Some(ref mut player) => {
                player.add_res("Hydrogen".to_string(), 100);
                player.add_res("Tokens".to_string(), 2000);
                player.print_stats();
            },
            None => ()
        }
        // Creating planet model
        let planet = render::Object{
            model: "./assets/models/planet.obj".to_string(),
            texture: "./assets/textures/i_amKerbol.png".to_string(),
            position: Point3::new(0.0,0.0,-40.0),
            rotation: Quaternion::new(0.0,0.0,1.0,0.0),
            tex_wrap: (0.0, 0.0),
            scale: (4.0, 4.0, 4.0)
        };
        let mut go_planet = Game_Object::new(0, String::new());
        go_planet.set_render_object(planet);

        self.objects.insert(0, go_planet);
        // Creating spaceship model
        let cabin = render::Object{
            model: "./assets/models/spaceship_cabin.obj".to_string(),
            texture: "./assets/textures/spaceship_cockpit.png".to_string(),
            position: Point3::new(0.0,-0.25,0.25),
            rotation: Quaternion::new(0.0,0.0,1.0,0.0),
            tex_wrap: (0.0, 0.0),
            scale: (0.1, 0.1, 0.1)
        };
        let mut go_cabin = Game_Object::new(1, String::new());
        go_cabin.set_render_object(cabin);

        self.objects.insert(1, go_cabin);
    }
    // Updating universe
    pub fn update(&mut self, window: &mut render::Window){
        use glium::glutin::Event::WindowEvent;
        match self.player{
            Some(ref mut x) => {
                x.update(self.seed);
            }
            None => {
                ()
            }
        }
        match self.objects.get_mut(&1).unwrap().render_object{
            Some(ref mut cabin) => {
                let rot_prev = cabin.rotation;
                let pos_prev = cabin.position;
                let forward = -cabin.forward() / 100.0;
                println!("{} \n {}", forward, rot_prev.vector());

                let rot = rot_prev.lerp(&UnitQuaternion::from_euler_angles(0.0, -(window.mouse_pos.0 as f32 / 100.0), 0.0).quaternion().into_owned(), 0.4);
                let camera_rotation = UnitQuaternion::from_euler_angles(0.0, -(window.mouse_pos.0 as f32 / 100.0), 0.0).quaternion().into_owned();
                let cabin_pos = Point3::new(pos_prev[0] + forward[0], pos_prev[1] + forward[1], pos_prev[2] + forward[2]);
                cabin.set_rotation(rot);
                cabin.set_position(cabin_pos);

                window.draw_context.camera.set_pos(Point3::new(cabin_pos[0], cabin_pos[1] + 0.20, cabin_pos[2]));
                window.draw_context.camera.set_rot(camera_rotation);
            }
            _ => {}
        }
        let mut objects = &mut self.objects;
        let mut objects_iter = objects.iter_mut();

        loop{
            let object = objects_iter.next();
            match object{
                Some((id, x)) => {
                    let id = x.parent;
                    x.update(Some(&objects[&id]));
                }
                None => {
                    break
                }
            }
        }
    }
}
