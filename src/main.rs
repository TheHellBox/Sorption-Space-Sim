#[macro_use]
extern crate glium;

extern crate rand;
extern crate nalgebra;
extern crate alga;
extern crate image;
extern crate openhmd_rs;
extern crate noise;

mod universe;
mod player;
mod render;
mod support;
mod camera;
mod openhmd;

use std::collections::HashMap;

fn main() {
    // Here we init engine
    let mut window = render::Window::new(1920, 1080, "Yet another space sim");
    //Building shaders
    let program = glium::Program::from_source(&window.draw_context.display, &render::std_shaders::SHADER_SIMPLE_VERT, &render::std_shaders::SHADER_SIMPLE_FRAG, None).unwrap();
    window.draw_context.render_buffer.add_shader("simple".to_string(), program);

    let program = glium::Program::from_source(&window.draw_context.display, &render::std_shaders::SHADER_SIMPLE_VERT, &render::std_shaders::SHADER_SOLID_FRAG, None).unwrap();
    window.draw_context.render_buffer.add_shader("solid".to_string(), program);

    let ohmd_shaders = glium::Program::from_source(&window.draw_context.display, &render::std_shaders::SHADER_DISTORTION_VERT, &render::std_shaders::SHADER_DISTORTION_FRAG, None).unwrap();
    window.draw_context.render_buffer.add_shader("ohmd".to_string(), ohmd_shaders);

    let params = render::get_params();

    let openhmd = openhmd::OpenHMD::new();
    // And here we init game
    println!("\nWelcome to yet another space sim! We are already created commader for you: \n");
    let mut universe = universe::Universe::new([0, 0, 0, 1]);
    let mut player = player::Player::new("The HellBox".to_string(), 0, (0,0,0), HashMap::new());
    // Move player to the universe
    universe.set_player(player);

    universe.init(&mut window);
    let vr = false;

    //Starting the main loop0
    'main: loop{
        openhmd.context.update();
        window.update();
        universe.update(&mut window);
        match vr{
            true => window.draw_context.draw_vr(&params, &universe, &openhmd),
            false => window.draw_context.draw(&params, &universe)
        }
        //println!("{:?}", window.draw_context.render_data.get_mut(&1).unwrap().rotation);
    }
}
