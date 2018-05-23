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
            let count = self.get_go_count() + 1;
            // Creating planet model
            let planet = render::object::ObjectBuilder::new()
                .with_model("./assets/models/planet.obj".to_string())
                .with_scale((400.0, 400.0, 400.0))
                .build_with_texture(&window, x.gen_tex(&window.draw_context.display));

            let mut go_planet = GameObject::new(count, (&x.name).to_owned());
            go_planet.set_render_object(planet);
            go_planet.set_position(Point3::new(0.0,0.0,0.0));
            self.objects.insert(count, go_planet);
            if x.rings {
                let count = self.get_go_count() + 1;
                // Creating rings model
                let rings = render::object::ObjectBuilder::new()
                    .with_model("./assets/models/rings.obj".to_string())
                    .with_texture("./assets/textures/rings.png".to_string())
                    .with_scale((400.0, 400.0, 400.0))
                    .build(window);

                let mut rings_go = GameObject::new(count, format!("{} {}", x.name, "rings"));
                rings_go.set_render_object(rings);
                rings_go.set_position(Point3::new(0.0,0.0,0.0));
                self.objects.insert(count, rings_go);
            }

        }


        // Creating spaceship model
        let cabin = render::object::ObjectBuilder::new()
            .with_model("./assets/models/spaceship_cabin.obj".to_string())
            .with_texture("./assets/textures/spaceship_cockpit.png".to_string())
            .build(window);

        let count = self.get_go_count() + 1;
        let mut go_cabin = GameObject::new(count, "Cabin".to_string());
        go_cabin.set_render_object(cabin);
        go_cabin.set_position(Point3::new(0.0,-1.5,-1.0));
        self.objects.insert(count, go_cabin);

        let background = render::object::ObjectBuilder::new()
            .with_model("./assets/models/background.obj".to_string())
            .with_scale((10000.0, 10000.0, 10000.0))
            .with_shader("solid".to_string())
            .build_with_texture(&window, support::image_m::gen_background_texture(&[0, 0, 0], &window.draw_context.display));

        let count = self.get_go_count() + 1;
        let mut go_background = GameObject::new(count, "Background".to_string());
        go_background.set_render_object(background);
        self.objects.insert(count, go_background);

    }
    //Create new game object with id and name
    pub fn add_game_object(&mut self, id: u32, name: String){
        let obj = GameObject::new(id, String::new());
        self.objects.insert(id, obj);
    }
    pub fn get_go_count(&self) -> u32{
        self.objects.len() as u32
    }
    // Get existing game object
    pub fn get_go(&mut self, id: u32) -> &mut GameObject{
        self.objects.get_mut(&id).unwrap()
    }
    // Get game object as option
    pub fn try_get_go(&mut self, id: u32) -> Option<&mut GameObject>{
        self.objects.get_mut(&id)
    }
    pub fn get_go_by_name(&mut self, name: String) -> Option<&mut GameObject>{
        for (id, x) in &mut self.objects {
            if x.name == name{
                return Some(x)
            }
        }
        None
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
        //Update controls
        self.controls.update(window);
        // Copy controls info
        let controls = self.controls;
        //FIXME: Might be bad way to update background position
        let mut bg_pos = Point3::new(0.0, 0.0, 0.0);
        // Set cabin position
        match self.get_go_by_name("Cabin".to_string()){
            Some(ref mut cabin) => {
                //Update cabin position
                let (mut cabin_pos, _) = game::cabin::cabin_update(cabin, window, &controls);
                //Calculate camera rotation
                let camera_rotation = UnitQuaternion::from_euler_angles((controls.rel.1 / 100.0), (controls.rel.0 / 100.0), controls.roll).quaternion().into_owned();
                //Set camera pos/rot
                window.draw_context.camera.set_pos(cabin_pos);
                window.draw_context.camera.set_rot(camera_rotation);
                //Copy cabin pos into background pos
                bg_pos = cabin_pos;
            },
            _ => {}
        }
        // Set background position
        match self.get_go_by_name("Background".to_string()){
            Some(ref mut bg) => {
                bg.set_position(bg_pos);
            },
            _ => {}
        }
        // Call Update on objects
        let objects = &mut self.objects;
        for (_, x) in objects{
            x.update()
        }
    }
}
