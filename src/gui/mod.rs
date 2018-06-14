pub mod widgets;

use glium::index::PrimitiveType::TriangleStrip;
use glium::{DrawParameters, VertexBuffer, IndexBuffer, Surface, Frame};
use universe::game::Game;
use render::Window;
use render::Vertex;


pub struct Gui{
    pub buttons: Vec<widgets::Button>
}

impl Gui{
    pub fn draw_gui(&self, target: &mut Frame, window: &Window, params: &DrawParameters){
        let disp = &window.draw_context;
        let vert_buf = VertexBuffer::new(&disp.display,
            &[
                Vertex { position: [ 0.0, 0.0, -1.0 ], normal: [ 0.0, 0.0, 0.0 ], tex_coords: [0.0, 0.0]},
                Vertex { position: [ 1.0, 0.0, -1.0 ], normal: [ 0.0, 0.0, 0.0 ], tex_coords: [1.0, 0.0]},
                Vertex { position: [ 1.0,  1.0, -1.0 ], normal: [ 0.0, 0.0, 0.0 ], tex_coords: [1.0, 1.0]},
                Vertex { position: [ 0.0,  1.0, -1.0 ], normal: [ 0.0, 0.0, 0.0 ], tex_coords: [0.0, 1.0]},
            ]
        ).unwrap();
        let index_buffer = IndexBuffer::new(&disp.display, TriangleStrip, &[1 as u16, 2, 0, 3]).unwrap();

        let perspective = disp.camera.perspective.to_homogeneous().as_ref().to_owned();
        let view: [[f32; 4]; 4] = disp.camera.rot_view().into();

        for x in &self.buttons{
            let matrix: [[f32; 4]; 4] =
            [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [ -0.5 , 0.0, -0.5, 1.0f32],
            ];

            target.draw(
                &vert_buf,
                &index_buffer,
                disp.render_buffer.shaders.get("solid").unwrap(),
                &uniform! { matrix: matrix, perspective: perspective, view: view, tex: &x.base.texture, wrap: [0.0 as f32, 0.0 as f32]},
                &params
            ).unwrap();

        }
    }
}
