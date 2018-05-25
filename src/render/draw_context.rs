use glium;
use glium::{Display, Program};
use glium::Surface;
use std::collections::hash_map::HashMap;
use glium::vertex::VertexBufferAny;
use camera::Camera;
use render::object::Object;
use glium::texture::Texture2d;
use universe::Universe;
use universe::game::Game;
use openhmd::OpenHMD;

pub struct RenderBuffer{
    pub shaders: HashMap<String, Program>
}

impl RenderBuffer{
    pub fn add_shader(&mut self, name: String, prog: Program){
        self.shaders.insert(name, prog);
    }
}
pub struct DrawContext{
    pub display: Display,
    pub render_buffer: RenderBuffer,
    pub render_data: HashMap<u32, Object>,
    pub camera: Camera,
    pub scr_res: (u32, u32)
}

impl DrawContext{
    pub fn draw(&self, params: &glium::DrawParameters, game: &Game){
        let mut target = self.display.draw();
        target.clear_color_and_depth((0.2, 0.2, 0.4, 1.0), 1.0);

        let perspective = self.camera.perspective.to_homogeneous().as_ref().to_owned();
        let view: [[f32; 4]; 4] = self.camera.view().into();

        for (_, x) in &game.universe.objects{
            let x = &x.render_object;
            match x{
                &Some(ref x) => {
                    if x.enabled {
                        let matrix = x.transform.as_ref().to_owned();
                        let texture = &x.texture;
                        target.draw(
                            &x.model,
                            &glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
                            self.render_buffer.shaders.get(&x.shader).unwrap(),
                            &uniform! { matrix: matrix, perspective: perspective, view: view, tex: texture, wrap: [x.tex_wrap.0, x.tex_wrap.1]},
                            &params
                        ).unwrap();
                    }
                },
                &None => {}
            }
        }
        target.finish().unwrap();
    }

    pub fn draw_vr(&self, params: &glium::DrawParameters, game: &Game, open_hmd: &OpenHMD){
        use glium::texture::{DepthTexture2d, Texture2d, DepthFormat, UncompressedFloatFormat, MipmapsOption};
        use glium::framebuffer::SimpleFrameBuffer;
        use render::OhmdVertex;

        let mut target = self.display.draw();
        target.clear_color_and_depth((0.2, 0.2, 0.4, 1.0), 1.0);

        let depthtexture1 = DepthTexture2d::empty_with_format(&self.display, DepthFormat::F32, MipmapsOption::NoMipmap, self.scr_res.0, self.scr_res.1).unwrap();
        let eye1_tex = Texture2d::empty_with_format(&self.display, UncompressedFloatFormat::F32F32F32F32, MipmapsOption::NoMipmap, self.scr_res.0, self.scr_res.1).unwrap();

        let depthtexture2 = DepthTexture2d::empty_with_format(&self.display, DepthFormat::F32, MipmapsOption::NoMipmap, self.scr_res.0, self.scr_res.1).unwrap();
        let eye2_tex = Texture2d::empty_with_format(&self.display, UncompressedFloatFormat::F32F32F32F32, MipmapsOption::NoMipmap, self.scr_res.0, self.scr_res.1).unwrap();

        let mut picking_target_left = SimpleFrameBuffer::with_depth_buffer(&self.display, &eye1_tex, &depthtexture1).unwrap();
        let mut picking_target_right = SimpleFrameBuffer::with_depth_buffer(&self.display, &eye2_tex, &depthtexture2).unwrap();

        picking_target_left.clear_color_and_depth((0.2, 0.2, 0.4, 1.0), 1.0);
        picking_target_right.clear_color_and_depth((0.2, 0.2, 0.4, 1.0), 1.0);

        let mut picking_targets: [SimpleFrameBuffer; 2] = [picking_target_left, picking_target_right];
        let model_view = open_hmd.get_view();
        let view = self.camera.view();

        let mod_view_left: [[f32; 4]; 4] = (model_view.0 * view).into();
        let mod_view_right: [[f32; 4]; 4] = (model_view.1 * view).into();
        let mod_view: [[[f32; 4]; 4]; 2] = [mod_view_left, mod_view_right];

        let perspectives: [[[f32; 4]; 4]; 2] = [open_hmd.config.projection1, open_hmd.config.projection2];

        for (_, x) in &game.universe.objects{
            let x = &x.render_object;
            match x{
                &Some(ref x) => {
                    if x.enabled {
                        let matrix = x.transform.as_ref().to_owned();
                        let texture = &x.texture;

                        for num in 0..2{
                            picking_targets[num].draw(
                                &x.model,
                                &glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
                                self.render_buffer.shaders.get(&x.shader).unwrap(),
                                &uniform! { matrix: matrix, perspective: perspectives[num], view: mod_view[num], tex: texture, wrap: [x.tex_wrap.0, x.tex_wrap.1]},
                                &params
                            ).unwrap();
                        }
                    }
                },
                &None => {}
            }
        }

        let eye_textures = [&eye1_tex, &eye2_tex];

        let vert_buf = glium::VertexBuffer::new(&self.display,
            &[
                OhmdVertex { coords: [ 0.0, 0.0 ]},
                OhmdVertex { coords: [ 1.0, 0.0 ]},
                OhmdVertex { coords: [ 1.0,  1.0 ]},
                OhmdVertex { coords: [ 0.0,  1.0 ]},
            ]
        ).unwrap();

        let index_buffer = glium::IndexBuffer::new(&self.display, glium::index::PrimitiveType::TriangleStrip, &[1 as u16, 2, 0, 3]).unwrap();

        for x in 0..2{
            target.draw(
                &vert_buf,
                &index_buffer,
                self.render_buffer.shaders.get("ohmd").unwrap(),
                &uniform! {  warpTexture: eye_textures[x], mvp: EYE_DRAW_MAT[x], LensCenter: open_hmd.config.left_lens_center,ViewportScale: open_hmd.config.view_port_scale,
                    WarpScale: open_hmd.config.warp_scale, HmdWarpParam: open_hmd.config.distortion_k, aberr: open_hmd.config.aberration_k},
                &params
            ).unwrap();
        }

        target.finish().unwrap();
    }
}


static EYE_DRAW_MAT: [[[f32; 4]; 4]; 2] = [
[
    [1.0, 0.0, 0.0, 0.0],
    [0.0, 2.0, 0.0, 0.0],
    [0.0, 0.0, 1.0, 0.0],
    [ 0.0 , -1.0, 0.0, 1.0f32],
],
[
    [1.0, 0.0, 0.0, 0.0],
    [0.0, 2.0, 0.0, 0.0],
    [0.0, 0.0, 1.0, 0.0],
    [ -1.0 , -1.0, 0.0, 1.0f32],
]
];
