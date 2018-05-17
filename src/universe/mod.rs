pub mod star;
pub mod planet;
pub mod gameobject;
pub mod game;

use glium;
use render;
use player;
use nalgebra::geometry::{Point3, UnitQuaternion, Quaternion};
use std::collections::HashMap;
use self::game::controls::Controls;

use self::gameobject::Game_Object;

// Universe, place where you can exist
pub struct Universe{
    pub seed: [usize; 4],
    pub player: Option<player::Player>,
    // Move away
    pub controls: Controls,
    //              ID, the object itself
    pub objects: HashMap<u32, Game_Object>
}

impl Universe{
    // Creating universe
    pub fn new(seed: [usize; 4]) -> Universe{
        Universe{
            seed: seed,
            player: None,
            controls: Controls::new(),
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
        let planet = render::Object::new(
            "./assets/models/planet.obj".to_string(),
            "./assets/textures/i_amKerbol.png".to_string(),
            (4.0, 4.0, 4.0)
        );
        let mut go_planet = Game_Object::new(0, String::new());
        go_planet.set_render_object(planet);
        go_planet.set_position(Point3::new(0.0,0.0,35.0));

        self.objects.insert(0, go_planet);

        // Creating spaceship model
        let cabin = render::Object::new(
            "./assets/models/spaceship_cabin.obj".to_string(),
            "./assets/textures/spaceship_cockpit.png".to_string(),
            (0.1, 0.1, 0.1)
        );
        let mut go_cabin = Game_Object::new(1, String::new());
        go_cabin.set_render_object(cabin);
        go_cabin.set_position(Point3::new(0.0,-1.5,-1.0));

        self.objects.insert(1, go_cabin);
    }
    //Create new game object with id and name
    pub fn add_game_object(&mut self, id: u32, name: String){
        let obj = Game_Object::new(0, String::new());
        self.objects.insert(id, obj);
    }
    // Get existing game object
    pub fn get_go(&mut self, id: u32) -> &mut Game_Object{
        self.objects.get_mut(&id).unwrap()
    }
    // Get game object as option
    pub fn try_get_go(&mut self, id: u32) -> Option<&mut Game_Object>{
        self.objects.get_mut(&id)
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
        self.controls.update(&window.events);

        let controls = self.controls;

        match self.try_get_go(1){
            Some(ref mut cabin) => {
                let (mut cabin_pos, rotation) = game::cabin::cabin_update(cabin, window, &controls);
                cabin_pos[1] += 0.20;
                let camera_rotation = UnitQuaternion::from_euler_angles(0.0, -(window.mouse_pos.0 as f32 / 100.0), 0.0).quaternion().into_owned();
                window.draw_context.camera.set_pos(cabin_pos);
                window.draw_context.camera.set_rot(camera_rotation);
            }
            _ => {}
        }

        let mut objects = &mut self.objects;
        for (_, x) in objects{
            x.update()
        }
    }
}
