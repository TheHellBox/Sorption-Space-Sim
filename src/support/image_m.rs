use glium::texture::Texture2d;
use glium::Display;
use image::{ImageBuffer, Rgb};
use support;
use noise::{Perlin, Seedable};
use noise::utils::*;
use nalgebra::clamp;
use scarlet::colors::hslcolor::HSLColor;
use scarlet::color::{RGBColor, Color};
use scarlet::illuminants::Illuminant;

// Really bad code
pub fn gen_planet_texture(seed: &[usize], disp: &Display, surf_color: (f32, f32, f32), oc_color: (f32, f32, f32)) -> Texture2d{
    let (s_x, s_y) = (1024, 512);
    let perlin = Perlin::new();
    let perlin = perlin.set_seed((seed[0] + seed[1] + seed[2]) as u32);
    // Ugh.... Thanks noise-rs creator! This fn is awesome!
    let surface_noise = SphereMapBuilder::new(&perlin)
        .set_size(s_x, s_y)
        .set_bounds(-90.0, 90.0, -180.0, 180.0)
        .build();
    let surface_noise_details = SphereMapBuilder::new(&perlin)
        .set_size(s_x, s_y)
        .set_bounds(-90.0 * 10.0, 90.0 * 10.0, -180.0 * 10.0, 180.0 * 10.0)
        .build();

    let mut planet_tex: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(s_x as u32, s_y as u32);
    for x in 0..s_x{
        for y in 0..s_y{

            let surf_px = surface_noise.get_value(x, y);
            let surf_px_dt = surface_noise_details.get_value(x, y) / 4.0;
            let surf_px = (clamp((surf_px + surf_px_dt) * 0.5 + 0.5, 0.0, 1.0) * 255.0) as f32;

            if surf_px <= 80.0{
                let r = (surf_px * surf_color.0) as u8;
                let g = (surf_px * surf_color.1) as u8;
                let b = (surf_px * surf_color.2) as u8;

                let pix = Rgb([r, g, b]);
                planet_tex.put_pixel(x as u32, y as u32, pix);
            }
            else{
                let r = (surf_px * oc_color.0) as u8;
                let g = (surf_px * oc_color.1) as u8;
                let b = (surf_px * oc_color.2) as u8;

                let pix = Rgb([r, g, b]);
                planet_tex.put_pixel(x as u32, y as u32, pix);
            }

        }
    }
    support::texture_loader::into_texture(planet_tex, disp)
}

pub fn gen_background_texture(seed: &[usize], disp: &Display) -> Texture2d{
    let (s_x, s_y) = (2048, 2048);

    let perlin = Perlin::new();
    let perlin = perlin.set_seed((seed[0] + seed[1] + seed[2]) as u32);

    /*let value = Value::new();
    let value = value.set_seed((seed[0] + seed[1] + seed[2]) as u32);*/

    let cloud_noise = SphereMapBuilder::new(&perlin)
        .set_size(s_x, s_y)
        .set_bounds(-90.0, 90.0, -180.0, 180.0)
        .build();

    /*let perlin = perlin.set_seed((seed[0] + seed[1] + seed[2]) as u32 * 3);
    let dark_noise = SphereMapBuilder::new(&perlin)
        .set_size(s_x, s_y)
        .set_bounds(-90.0, 90.0, -180.0, 180.0)
        .build();*/

    let mut background_tex: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(s_x as u32, s_y as u32);
    for x in 0..s_x{
        for y in 0..s_y{
            let cloud_noise = cloud_noise.get_value(x, y);
            //let dark_noise = dark_noise.get_value(x, y);

            let bg_px = (clamp((cloud_noise) * 0.5 + 0.5, 0.0, 1.0) * 360.0) as f64;
            let hsl_px = HSLColor{h: bg_px, s: 0.2, l: 0.2}.to_xyz(Illuminant::D50);
            let hsl_px = RGBColor::from_xyz( hsl_px );

            let pix = Rgb([(hsl_px.r * 255.0) as u8, (hsl_px.g * 255.0) as u8, (hsl_px.b * 255.0) as u8]);


            background_tex.put_pixel(x as u32, y as u32, pix);
        }
    }
    support::texture_loader::into_texture(background_tex, disp)
}
