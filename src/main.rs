#[macro_use]
extern crate glium;

extern crate rand;
extern crate nalgebra;
extern crate alga;
extern crate image;

mod universe;
mod player;
mod render;
mod support;
mod camera;

use std::collections::HashMap;

fn main() {
    use nalgebra::geometry::{Quaternion, Point3};

    // Here we init engine
    let mut window = render::Window::new(1280, 768, "Yet another space sim");
    let mut mesh_buffer = match support::obj_loader::gen_buffer(&window.draw_context.display){
        Some(x) => x,
        None => HashMap::new()
    };
    let mut texture_buffer = match support::texture_loader::gen_buffer(&window.draw_context.display){
        Some(x) => x,
        None => HashMap::new()
    };
    //Building shaders
    let program = glium::Program::from_source(&window.draw_context.display, &render::SHADER_SIMPLE_VERT, &render::SHADER_SIMPLE_FRAG, None).unwrap();
    window.draw_context.render_buffer.set_vert_bufs(mesh_buffer);
    window.draw_context.render_buffer.set_texture_buf(texture_buffer);
    window.draw_context.render_buffer.add_shader("simple".to_string(), program);

    //FIXME: Move params to other place
    let params = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::DepthTest::IfLess,
            write: true,
            .. Default::default()
        },
        backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
        .. Default::default()
    };

    // And here we init game
    println!("\nWelcome to yet another space sim! We are already created commader for you: \n");
    let mut universe = universe::Universe::new([0, 0, 0, 1]);
    let mut player = player::Player::new("The HellBox".to_string(), 0, (0,0,0), HashMap::new());
    // Move player to the universe
    universe.set_player(player);

    universe.init(&mut window);
    //Starting the main loop0
    'main: loop{
        window.update();
        universe.update(&mut window);
        window.draw_context.draw(&params, &universe);
        //println!("{:?}", window.draw_context.render_data.get_mut(&1).unwrap().rotation);
    }
}
