pub mod star;
pub mod planet;
pub mod gameobject;
pub mod game;

use render;
use player;
use nalgebra::geometry::{Point3, UnitQuaternion};
use std::collections::HashMap;
use self::game::controls::Controls;

use self::gameobject::GameObject;
use support;

// Universe, place where you can exist
pub struct Universe{
    pub seed: [usize; 4],
    pub player: Option<player::Player>,
    // Move away
    pub controls: Controls,
    //              ID, the object itself
    pub objects: HashMap<u32, GameObject>
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
    pub fn get_star(&self, coords: Point3<usize>) -> Option<star::Star>{
        star::Star::gen(coords, &self.seed)
    }
    // Prepare universe for playing
    pub fn init(&mut self, window: &mut render::Window){
        let mut star_coords = Point3::new(0, 0 ,0);
        //Add default items to player's inventory
        match self.player{
            Some(ref mut player) => {
                player.add_res("Hydrogen".to_string(), 100);
                player.add_res("Tokens".to_string(), 2000);
                player.print_stats();
                star_coords = player.star_coords;
            },
            None => ()
        }
        let star = self.get_star(star_coords).unwrap();
        for x in star.planets{
            // Creating planet model
            let planet = render::Object::new(
                support::obj_loader::load_as_vb("./assets/models/planet.obj".to_string(), &window.draw_context.display),
                x.gen_tex(&window.draw_context.display),
                (4.0, 4.0, 4.0)
            );
            let mut go_planet = GameObject::new((10 + x.num) as u32, String::new());
            go_planet.set_render_object(planet);
            go_planet.set_position(Point3::new(0.0,0.0,35.0 * x.num as f32));
            self.objects.insert((10 + x.num) as u32, go_planet);

            if x.rings {
                // Creating rings model
                let rings = render::Object::new(
                    support::obj_loader::load_as_vb("./assets/models/rings.obj".to_string(), &window.draw_context.display),
                    support::texture_loader::load("./assets/textures/spaceship_cockpit.png".to_string(), &window.draw_context.display),
                    (4.0, 4.0, 4.0)
                );
                let mut rings_go = GameObject::new((10 + x.num) as u32, String::new());
                rings_go.set_render_object(rings);
                rings_go.set_position(Point3::new(0.0,0.0,35.0 * x.num as f32));
                self.objects.insert((30 + x.num) as u32, rings_go);
            }

        }


        // Creating spaceship model
        let cabin = render::Object::new(
            support::obj_loader::load_as_vb("./assets/models/spaceship_cabin.obj".to_string(), &window.draw_context.display),
            support::texture_loader::load("./assets/textures/spaceship_cockpit.png".to_string(), &window.draw_context.display),
            (0.1, 0.1, 0.1)
        );
        let mut go_cabin = GameObject::new(1, String::new());
        go_cabin.set_render_object(cabin);
        go_cabin.set_position(Point3::new(0.0,-1.5,-1.0));

        self.objects.insert(1, go_cabin);
    }
    //Create new game object with id and name
    pub fn add_game_object(&mut self, id: u32, name: String){
        let obj = GameObject::new(0, String::new());
        self.objects.insert(id, obj);
    }
    // Get existing game object
    pub fn get_go(&mut self, id: u32) -> &mut GameObject{
        self.objects.get_mut(&id).unwrap()
    }
    // Get game object as option
    pub fn try_get_go(&mut self, id: u32) -> Option<&mut GameObject>{
        self.objects.get_mut(&id)
    }
    // Updating universe
    pub fn update(&mut self, window: &mut render::Window){
        match self.player{
            Some(ref mut x) => {
                x.update(self.seed);
            }
            None => {
                ()
            }
        }
        self.controls.update(&window);

        let controls = self.controls;

        match self.try_get_go(1){
            Some(ref mut cabin) => {
                let (mut cabin_pos, _) = game::cabin::cabin_update(cabin, window, &controls);
                let camera_rotation = UnitQuaternion::from_euler_angles((controls.rel.1 / 100.0), (controls.rel.0 / 100.0), controls.roll).quaternion().into_owned();
                window.draw_context.camera.set_pos(cabin_pos);
                window.draw_context.camera.set_rot(camera_rotation);
            }
            _ => {}
        }

        let objects = &mut self.objects;
        for (_, x) in objects{
            x.update()
        }
    }
}
