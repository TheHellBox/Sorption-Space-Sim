use std::collections::HashMap;
use universe::star::Star;
use nalgebra::geometry::Point3;
use alga::linear::EuclideanSpace;

pub struct Player{
    pub name: String,
    pub level: u32,
    pub star_coords: Point3<usize>,
    pub area: (u32, u32),
    pub planet: usize,
    pub resources: HashMap<String, u32>
}

impl Player{
    pub fn new(name: String, level: u32, star_coords: (usize, usize, usize), resources: HashMap<String, u32>) -> Player{
        Player{
            name: name,
            level: level,
            planet: 0,
            area: (0, 0),
            star_coords: Point3::new(star_coords.0, star_coords.1, star_coords.2),
            resources: resources
        }
    }

    pub fn add_res(&mut self, name: String, count: u32){
        let result = match self.resources.get(&name) {
            Some(x) => {
                x.clone()
            },
            None => {
                0
            }
        };
        self.resources.insert(name, result + count as u32);
    }
    pub fn set_area(&mut self, x: u32, y: u32){
        self.area = (x, y);
    }
    pub fn print_stats(&self){
        println!("{}:", self.name);
        println!("  Level {}", self.level);
        println!("  Star Coords {} {} {}", self.star_coords[0], self.star_coords[1], self.star_coords[2]);
        println!("  Resources:");
        for (name, count) in self.resources.clone(){
            println!("      {}: {}", name, count);
        }
    }
    pub fn jump(&mut self, dest: Point3<usize>, seed: [usize; 4]){
        let point = Point3::new(dest[0] as f32, dest[1] as f32, dest[2] as f32);
        let coords = Point3::new(self.star_coords[0] as f32, self.star_coords[1] as f32, self.star_coords[2] as f32);
        let distance = coords.distance(&point);
        let fuel = self.resources.get(&"Hydrogen".to_string()).unwrap().clone();
        if fuel >= distance as u32{
            self.resources.insert("Hydrogen".to_string(), fuel - distance as u32).unwrap();
            self.star_coords = Point3::new(point[0] as usize, point[1] as usize, point[2] as usize);
            self.planet = 0;
            println!("  Jump done, star name: {}", match Star::gen(self.star_coords, &seed){
                Some(x) => x.name,
                None => "There no star".to_string()
            });
        }
        else{
            println!("Not enough fuel!");
        }
    }
    pub fn jump_to(&mut self, planet: usize){
        self.planet = planet;
    }
    pub fn update(&mut self, seed: [usize; 4]){
        /*use std::io;
        use std::io::prelude::*;
        use nalgebra::Real;
        use std::ops::Sub;

        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            let line = line.unwrap();
            let line: Vec<_> = line.split(" ").collect();
            println!("");
            match line[0]{
                "stats" => {
                    self.print_stats();
                }
                "get" => {
                    println!("  {}", match self.resources.get(line[1]){
                        Some(x) => x,
                        None => &0
                    });
                }
                "star" => {
                    match Star::gen(self.star_coords, &seed){
                        Some(x) => x.print_stats(),
                        None => println!("There no star")
                    };
                }
                "planet" => {
                    match Star::gen(self.star_coords, &seed){
                        Some(x) => x.planets[self.planet - 1].print_stats(),
                        None => println!("There no star")
                    };
                }
                "jump_to" => {
                    self.jump_to(line[1].parse::<usize>().unwrap());
                }
                "jump" => {
                    if line.len() > 3{
                        let point = Point3::new(line[1].parse::<usize>().unwrap(), line[2].parse::<usize>().unwrap(), line[3].parse::<usize>().unwrap());
                        self.jump(point, seed);
                    }
                }
                _ => println!("Unknow command!")
            }
        }*/
    }
}
