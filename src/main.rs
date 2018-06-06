#[macro_use]
extern crate glium;

extern crate rand;
extern crate nalgebra;
extern crate alga;
extern crate image;
extern crate openhmd_rs;
extern crate noise;
extern crate scarlet;
extern crate rusttype;

mod universe;
mod player;
mod render;
mod support;
mod camera;
mod openhmd;

use std::collections::HashMap;

fn main() {
    // Here we init engine
    let mut window = render::Window::new(1024, 768, "Yet another space sim");
    //Building shaders
    let program = glium::Program::from_source(&window.draw_context.display, &render::std_shaders::SHADER_SIMPLE_VERT, &render::std_shaders::SHADER_SIMPLE_FRAG, None).unwrap();
    window.draw_context.render_buffer.add_shader("simple".to_string(), program);

    let program = glium::Program::from_source(&window.draw_context.display, &render::std_shaders::SHADER_SIMPLE_VERT, &render::std_shaders::SHADER_SOLID_FRAG, None).unwrap();
    window.draw_context.render_buffer.add_shader("solid".to_string(), program);

    let ohmd_shaders = glium::Program::from_source(&window.draw_context.display, &render::std_shaders::SHADER_DISTORTION_VERT, &render::std_shaders::SHADER_DISTORTION_FRAG, None).unwrap();
    window.draw_context.render_buffer.add_shader("ohmd".to_string(), ohmd_shaders);

    let params = render::get_params();

    let openhmd = openhmd::OpenHMD::new();

    window.font_engine.load_font("./Roboto-Medium.ttf".to_string(), &window.draw_context.display);

    // And here we init game
    println!("\nWelcome to yet another space sim! We are already created commader for you: \n");
    let mut game = universe::game::Game::new(&mut window);

    let player = player::Player::new("The HellBox".to_string(), 0, (0,0,0), HashMap::new());

    // Move player to the universe
    game.universe.set_player(player);

    let vr = false;

    //Starting the main loop0
    'main: loop{
        openhmd.context.update();
        window.update();
        game.update(&mut window);
        match vr{
            true => window.draw_context.draw_vr(&params, &game, &openhmd),
            false => window.draw_context.draw(&params, &game)
        }
        //println!("{:?}", window.draw_context.render_data.get_mut(&1).unwrap().rotation);
    }
}
