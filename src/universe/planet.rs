use rand::{Rng, SeedableRng, StdRng};
use nalgebra;
use support;
use glium::Display;
use glium::texture::Texture2d;
use support::image_m::gen_planet_texture;

enum PlanetType{
    EarthLike = 0,
    IceDesert = 1,
    Moon = 2
}


static colors: [(f32, f32, f32); 3] = [
//EarthLike
(0.5, 1.0, 0.5),
//IceDesert
(0.5, 0.5, 1.0),
//Moon
(1.0, 1.0, 1.0)
];

static ocean_colors: [(f32, f32, f32); 3] = [
//EarthLike
(0.4, 0.4, 1.0),
//IceDesert
(0.3, 0.3, 1.0),
//Moon
(1.0, 1.0, 1.0)
];
#[derive(Debug)]
pub struct Planet{
    pub num: usize,
    pub planet_type: u32,
    pub name: String,
    pub temperature: i32,
    pub orbit: u32,
    pub area: (i32, i32),
    pub seed: [usize; 3],
    pub rings: bool,
    pub scale: f32,
    pub moons: Vec<Planet>
}

impl Planet{
    pub fn gen(num: usize, seed: &[usize], surf_temperature: u32, star_name: String) -> Planet{

        let seed: &[usize] = &[seed[0] + num, seed[1] + num, seed[2] + num];
        let mut rng: StdRng = SeedableRng::from_seed(seed);
        let planet_type = rng.gen_range(0, 3);
        let name = gen_name(seed, star_name);
        let orbit = rng.gen_range(0, 10);
        let area = (rng.gen_range(0, 5), rng.gen_range(0, 5));
        let scale = rng.gen_range(0, 800) as f32;
        let temperature = ((100 - orbit * 10) + (surf_temperature / 100)) as i32;
        let moons = vec![];
        let seed = [seed[0], seed[1], seed[2]];
        let rings = match rng.gen_range(0, 5){
            0 => false,
            1 => true,
            _ => false
        };

        Planet{
            num,
            planet_type,
            name,
            temperature,
            orbit,
            area,
            seed,
            rings,
            scale,
            moons
        }

    }
    pub fn print_stats(&self){
        println!("{}", self.name);
        println!("  Type: {}", self.planet_type);
        println!("  Orbit: {} AU", self.orbit);
        println!("  Temperature: {}°C", self.temperature);
    }
    pub fn gen_tex(&self, disp: &Display) -> Texture2d{
        gen_planet_texture(&self.seed, disp, colors[self.planet_type as usize], ocean_colors[self.planet_type as usize])
    }
}

pub fn gen_name(seed: &[usize], star_name: String) -> String{
    let mut rng: StdRng = SeedableRng::from_seed(seed);
    let range = rng.gen_range(0, 1000);

    format!("{}-{}", star_name, range)
}
