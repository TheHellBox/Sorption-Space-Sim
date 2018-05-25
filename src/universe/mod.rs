pub mod star;
pub mod planet;
pub mod gameobject;
pub mod game;

use render;
use player;
use nalgebra::geometry::{Point3, UnitQuaternion};
use std::collections::HashMap;
use self::game::controls::Controls;

use self::gameobject::{GameObject, GameObjectBuilder};
use support;

// Universe, place where you can exist
pub struct Universe{
    pub seed: [usize; 4],
    pub player: Option<player::Player>,
    // Move away
    pub controls: Controls,
    //              ID, the object itself
    pub objects: HashMap<u32, GameObject>,

    pub events: Vec<String>
}

impl Universe{
    // Creating universe
    pub fn new(seed: [usize; 4]) -> Universe{
        Universe{
            seed: seed,
            player: None,
            controls: Controls::new(),
            objects: HashMap::new(),
            events: vec![]
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
        let mut area = (0, 0);
        //Add default items to player's inventory
        match self.player{
            Some(ref mut player) => {
                player.add_res("Hydrogen".to_string(), 100);
                player.add_res("Tokens".to_string(), 2000);
                player.print_stats();
                star_coords = player.star_coords;
                area = player.area;
            },
            None => ()
        }
        let star = self.get_star(star_coords).unwrap();
        for x in star.planets{
            println!("{:?}", x);
            let enabled = x.area == area;
            // Creating planet model
            let planet = render::object::ObjectBuilder::new()
                .with_model("./assets/models/planet.obj".to_string())
                .with_enabled(enabled)
                .with_scale((400.0, 400.0, 400.0))
                .build_with_texture(&window, x.gen_tex(&window.draw_context.display));

            let mut go_planet = GameObjectBuilder::new()
                .with_name((&x.name).to_owned())
                .with_render_object(planet)
                .with_tags(vec!["Planet".to_string()]);

            self.build_game_object(go_planet);
            if x.rings {
                // Creating rings model
                let rings = render::object::ObjectBuilder::new()
                    .with_model("./assets/models/rings.obj".to_string())
                    .with_enabled(enabled)
                    .with_texture("./assets/textures/rings.png".to_string())
                    .with_scale((400.0, 400.0, 400.0))
                    .build(window);

                let mut rings_go = GameObjectBuilder::new()
                    .with_name(format!("{} {}", x.name, "rings"))
                    .with_render_object(rings)
                    .with_tags(vec!["Ring".to_string()]);
                self.build_game_object(rings_go);

            }

        }
        // Creating spaceship model
        let cabin = render::object::ObjectBuilder::new()
            .with_model("./assets/models/spaceship_cabin.obj".to_string())
            .with_texture("./assets/textures/spaceship_cockpit.png".to_string())
            .build(window);

        let mut go_cabin = GameObjectBuilder::new()
            .with_name("Cabin".to_string())
            .with_render_object(cabin);
        self.build_game_object(go_cabin);

        let background = render::object::ObjectBuilder::new()
            .with_model("./assets/models/background.obj".to_string())
            .with_scale((10000.0, 10000.0, 10000.0))
            .with_shader("solid".to_string())
            .build_with_texture(&window, support::image_m::gen_background_texture(&[0, 0, 0], &window.draw_context.display));

        let mut go_background = GameObjectBuilder::new()
            .with_name("Background".to_string())
            .with_render_object(background);
        self.build_game_object(go_background);

    }
    //Create new game object with id and name
    pub fn add_game_object(&mut self, id: u32, name: String){
        let obj = GameObject::new(id, String::new());
        self.objects.insert(id, obj);
    }
    pub fn build_game_object(&mut self, builder: GameObjectBuilder){
        let count = self.get_go_count() + 1;
        let object = builder.with_id(count).build();
        self.objects.insert(count, object);
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
    //Returns game object with the same name
    pub fn get_go_by_name(&mut self, name: String) -> Option<&mut GameObject>{
        for (id, x) in &mut self.objects {
            if x.name == name{
                return Some(x)
            }
        }
        None
    }
    //Returns game objects with the same tag
    pub fn get_go_by_tag(&mut self, tag: String) -> Vec<&mut GameObject>{
        let mut objects = vec![];
        for (id, x) in &mut self.objects {
            if x.tags.contains(&tag){
                objects.push(x);
            }
        }
        objects
    }
    // Updating universe
    pub fn update(&mut self, window: &mut render::Window){
        //Update controls
        self.controls.update(window);
        match self.player{
            Some(ref mut x) => {
                x.update(self.seed);
            }
            None => {
                ()
            }
        }

    }
}
