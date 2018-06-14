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
mod gui;

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

    window.font_engine.load_font("./Roboto-Medium.ttf".to_string());

    // And here we init game
    println!("\nWelcome to yet another space sim! We are already created commader for you: \n");
    let mut game = universe::game::Game::new(&mut window);

    let player = player::Player::new("The HellBox".to_string(), 0, (0,0,0), HashMap::new());

    // Move player to the universe
    game.universe.set_player(player);

    let vr = false;

    window.gui_manager.buttons.push(
        gui::widgets::Button{
            base: gui::widgets::Widget_Base{
                position: (0.0, 0.0),
                size: (1.0, 1.0),
                color: (0, 0, 0),
                texture: support::texture_loader::load("./assets/textures/test.png".to_string(), &window.draw_context.display)
            },
            text: String::new()
        }
    );
    //Starting the main loop0
    'main: loop{
        openhmd.context.update();
        window.update();
        game.update(&mut window);
        let mut frame = match vr{
            true => window.draw_context.draw_vr(&params, &game, &openhmd),
            false => window.draw_context.draw(&params, &game)
        };
        window.gui_manager.draw_gui(&mut frame, &window, &params);
        frame.finish().unwrap();
        //println!("{:?}", window.draw_context.render_data.get_mut(&1).unwrap().rotation);
    }
}
