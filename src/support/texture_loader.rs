use glium;
use image;

use glium::texture::Texture2d;
use glium::Display;
use std::collections::HashMap;

pub fn load(path: String, disp: &Display) -> Texture2d{
    use std::path::Path;
    use glium::texture::RawImage2d;
    use image::GenericImage;

    let img = image::open(Path::new(&path)).unwrap().to_rgba();
    let dis = img.dimensions();
    let glium_raw_tex = RawImage2d::from_raw_rgba_reversed(&img.into_raw(), dis);
    let tex = Texture2d::new(disp, glium_raw_tex).unwrap();
    tex
}

pub fn gen_buffer(disp: &Display) -> Option<HashMap<String, Texture2d>>{
    use std::fs;
    use std::path::Path;
    let dir = Path::new("./assets/textures/");
    if dir.exists(){
        let paths = fs::read_dir(dir).unwrap();
        let mut textures: HashMap<String, Texture2d> = HashMap::with_capacity(1024);
        for path in paths {
            let path = path.unwrap().path();
            if path.is_file() {
                let name = path.display().to_string();
                if path.extension().unwrap() == "png"{
                    print!("Loading texture {} ... ", path.display());
                    let tex = load(path.display().to_string(), disp);
                    textures.insert(name, tex);
                    println!("Done!");
                }
            }
        }
        Some(textures)
    }
    else{
        None
    }
}
