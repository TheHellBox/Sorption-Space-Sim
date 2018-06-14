use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use image::{ImageBuffer, Rgba};
use rusttype::{point, Font, Scale};

//FIXME: It's a bad way to get all glyphs
static GLYPH_DATA: &'static str = "qwertyuiop[]asdfghjkl;'zxcvbnm,./1234567890-=+`";

pub struct FontEngine{
    pub glyph_list: HashMap<(String, u32), ImageBuffer<Rgba<u8>, Vec<u8>>>
}

#[allow(dead_code)]
impl FontEngine{
    pub fn new() -> FontEngine{
        FontEngine{
            glyph_list: HashMap::new()
        }
    }
    pub fn load_font(&mut self, path: String){
        use std::path::Path;
        let path = Path::new(&path);
        let mut font_file = match File::open(path){
            Ok(x) => x,
            Err(_) => return ()
        };
        let mut raw = vec![];
        match font_file.read_to_end(&mut raw){
            Ok(x) => x,
            Err(_) => return ()
        };
        let font = match Font::from_bytes(raw.as_slice()){
            Ok(x) => x,
            Err(_) => return ()
        };
        let scale = Scale { x: 32.0, y: 32.0 };
        let start = point(20.0, 50.0);
        for glyph in font.layout(GLYPH_DATA, scale, start) {
            let mut image: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(32, 32);
            if let Some(_) = glyph.pixel_bounding_box() {
                glyph.draw(|x, y, v| {
                    image.put_pixel(x,y,Rgba{data: [255, 255, 255, (v * 255.0) as u8]},
                    )
                });
            }
            self.glyph_list.insert((path.file_name().unwrap().to_str().unwrap().to_string(), glyph.id().0), image);
        }
    }
    pub fn get(&self, font: String, glyph: u32) -> Option<&ImageBuffer<Rgba<u8>, Vec<u8>>>{
        self.glyph_list.get(&(font, glyph))
    }
}
