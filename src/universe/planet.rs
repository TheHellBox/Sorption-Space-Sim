use rand::{Rng, SeedableRng, StdRng};

enum PlanetType{
    EarthLike = 0,
    IceDesert = 1,
    GasGiant = 2,
    Moon = 3
}

pub struct Planet{
    pub num: usize,
    pub planet_type: u32,
    pub name: String,
    pub temperature: i32,
    pub orbit: u32,
    pub moons: Vec<Planet>
}

impl Planet{
    pub fn gen(num: usize, seed: &[usize], surf_temperature: u32, star_name: String) -> Planet{
        let seed: &[usize] = &[seed[0] + num, seed[1] + num, seed[2] + num];
        let mut rng: StdRng = SeedableRng::from_seed(seed);
        let pl_type = rng.gen_range(0, 4);
        let name = gen_name(seed, star_name);
        let orbit = rng.gen_range(0, 30);
        let temperature = (100 - orbit * 10) + (surf_temperature / 100);
        let moons = vec![];
        Planet{
            num: num,
            planet_type: pl_type,
            name: name,
            temperature: temperature as i32,
            orbit: orbit,
            moons: moons
        }
    }
    pub fn print_stats(&self){
        println!("{}", self.name);
        println!("  Type: {}", self.planet_type);
        println!("  Orbit: {} AU", self.orbit);
        println!("  Temperature: {}°C", self.temperature);
    }
}
pub fn gen_name(seed: &[usize], star_name: String) -> String{
    let mut rng: StdRng = SeedableRng::from_seed(seed);
    let range = rng.gen_range(0, 1000);

    format!("{}-{}", star_name, range)
}
