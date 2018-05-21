use rand::{Rng, SeedableRng, StdRng};
use noise::{NoiseFn, Perlin, Seedable};
use noise::utils::*;
use image::{ImageBuffer, Rgb};
use nalgebra;
use support;
use glium::Display;
use glium::texture::Texture2d;

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

// Really bad code
pub fn gen_texture(seed: &[usize], disp: &Display) -> Texture2d{
    let (s_x, s_y) = (1024, 512);
    let perlin = Perlin::new();
    let perlin = perlin.set_seed((seed[0] + seed[1] + seed[2]) as u32);
    // Ugh.... Thanks noise-rs creator! This fn is awesome!
    let surface_noise = SphereMapBuilder::new(&perlin)
        .set_size(s_x, s_y)
        .set_bounds(-90.0, 90.0, -180.0, 180.0)
        .build();

    let mut planet_tex: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(s_x as u32, s_y as u32);
    for x in 0..s_x{
        for y in 0..s_y{

            let surf_px = surface_noise.get_value(x, y);
            let surf_px = (nalgebra::clamp(surf_px * 0.5 + 0.5, 0.0, 1.0) * 255.0) as u32;

            if surf_px <= 80{
                let pix = Rgb([surf_px as u8 / 2, surf_px as u8, surf_px as u8 / 2]);
                planet_tex.put_pixel(x as u32, y as u32, pix);
            }
            else{
                let pix = Rgb([60 as u8, 60 as u8, surf_px as u8]);
                planet_tex.put_pixel(x as u32, y as u32, pix);
            }
        }
    }
    support::texture_loader::into_texture(planet_tex, disp)
}
