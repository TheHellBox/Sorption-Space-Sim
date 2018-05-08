use glium;
use glium::{Display, Program};
use glium::Surface;
use std::collections::hash_map::HashMap;
use glium::vertex::VertexBufferAny;
use camera::Camera;
use render::Object;
use support::math::*;
use nalgebra::geometry::UnitQuaternion;
use glium::texture::Texture2d;
use universe::Universe;

pub struct RenderBuffer{
    pub vertex_buffers: HashMap<String, VertexBufferAny>,
    pub texture_buffer: HashMap<String, Texture2d>,
    pub shaders: HashMap<String, Program>
}

impl RenderBuffer{
    pub fn set_vert_bufs(&mut self, vrt_buf: HashMap<String, VertexBufferAny>){
        self.vertex_buffers = vrt_buf;
    }
    pub fn set_texture_buf(&mut self, tex_buf: HashMap<String, Texture2d>){
        self.texture_buffer = tex_buf;
    }
    pub fn add_shader(&mut self, name: String, prog: Program){
        self.shaders.insert(name, prog);
    }
}
pub struct DrawContext{
    pub display: Display,
    pub render_buffer: RenderBuffer,
    pub render_data: HashMap<u32, Object>,
    pub camera: Camera
}

impl DrawContext{
    pub fn draw(&self, params: &glium::DrawParameters, universe: &Universe){
        let mut target = self.display.draw();
        target.clear_color_and_depth((0.2, 0.2, 0.4, 1.0), 1.0);

        for (_, x) in &universe.objects{
            let x = &x.render_object;
            match x{
                &Some(ref x) => {
                    let matrix =  nalg_to_4x4(mat_to_nalg([
                        [x.scale.0, 0.0, 0.0, 0.0],
                        [0.0, x.scale.1, 0.0, 0.0],
                        [0.0, 0.0, x.scale.2, 0.0],
                        [x.position[0], x.position[1], x.position[2], 1.0f32],
                    ]) * UnitQuaternion::from_quaternion(x.rotation).to_homogeneous());

                    let texture = self.render_buffer.texture_buffer.get(&x.texture).unwrap();
                    let perspective = self.camera.perspective.to_homogeneous().as_ref().to_owned();
                    let view = self.camera.view();
                    target.draw(
                        self.render_buffer.vertex_buffers.get(&x.model).unwrap(),
                        &glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
                        self.render_buffer.shaders.get("simple").unwrap(),
                        &uniform! { matrix: matrix, perspective: perspective, view: view, tex: texture, wrap: [x.tex_wrap.0, x.tex_wrap.1]},
                        &params
                    ).unwrap();
                },
                &None => {}
            }
        }
        target.finish().unwrap();
    }
}
