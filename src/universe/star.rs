use universe::planet::Planet;
use rand::{Rng, SeedableRng, StdRng};
use nalgebra::geometry::Point3;

pub struct Star{
    pub star_type: u32,
    pub name: String,
    pub coords: Point3<usize>,
    pub surf_temperature: u32,
    pub planets: Vec<Planet>
}

impl Star{
    pub fn gen(coords: Point3<usize>, seed: &[usize]) -> Option<Star>{
        let seed: &[usize] = &[seed[0] + coords[0], seed[1] + coords[1], seed[2] + coords[2]];
        let mut rng: StdRng = SeedableRng::from_seed(seed);
        let exist = rng.gen_range(0, 2);
        if exist == 1{
            let st_type = rng.gen_range(0, 3);
            let surf_temperature = rng.gen_range(1000, 20000);
            let name = gen_star_name(&[seed[0] + coords[0], seed[1] + coords[1], seed[2] + coords[2]]);
            let mut planets = vec![];
            for x in 1..rng.gen_range(0, 30){
                planets.push(Planet::gen(x, seed, surf_temperature, name.clone()));
            }
            Some(Star{
                star_type: st_type,
                name: name,
                coords: coords,
                surf_temperature: surf_temperature,
                planets: planets
            })
        }
        else{
            None
        }
    }
    #[allow(dead_code)]
    pub fn print_stats(&self){
        println!("{}:", self.name);
        println!("  Coords: {} {} {}", self.coords[0], self.coords[1], self.coords[2]);
        println!("  Type: {}", self.star_type);
        println!("  Temperature: {}°C", self.surf_temperature);
        println!("  Planets:");
        for x in &self.planets{
            println!("      {}: {}  {} AU   {}°C", x.num, x.name, x.orbit, x.temperature);
        }
    }
}

pub fn gen_star_name(seed: &[usize]) -> String{
    let mut rng: StdRng = SeedableRng::from_seed(seed);
    let range = rng.gen_range(0, 5);
    let range2 = rng.gen_range(0, 1000);

    let name = match range{
        0 => "Beta",
        1 => "Alpha",
        2 => "CX",
        3 => "BG",
        4 => "Zaxs",
        5 => "SEV",
        _ => "ERROR"
    };
    format!("{} {}", name, range2)
}
