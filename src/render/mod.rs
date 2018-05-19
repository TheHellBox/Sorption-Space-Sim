pub mod draw_context;

use support::math;

use self::draw_context::DrawContext;

use glium;
use glium::{glutin, Display};
use glium::glutin::{ContextBuilder, EventsLoop, WindowBuilder};
use glium::glutin::Event;

use std::collections::hash_map::HashMap;

use camera::Camera;

use nalgebra::geometry::{Quaternion, Point3};
use nalgebra::core::{Vector3, Matrix4};

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub tex_coords: [f32; 2]
}
implement_vertex!(Vertex, position, normal, tex_coords);

#[derive(Copy, Clone)]
pub struct OhmdVertex {
    pub coords: [f32; 2],
}
implement_vertex!(OhmdVertex, coords);

pub struct Mouse{
    pub position: (i32, i32),
    pub releative: (i32, i32)
}

pub struct Window{
    pub events_loop: EventsLoop,
    pub draw_context: DrawContext,
    pub events: Vec<Event>,
    pub mouse: Mouse
}
// Object that can be rendered
pub struct Object{
    pub model: String,
    pub texture: String,
    //pub position: Point3<f32>,
    //pub rotation: Quaternion<f32>,
    pub transform: Matrix4<f32>,
    pub tex_wrap: (f32, f32),
    pub scale: (f32, f32, f32)
}
impl Object{
    pub fn new(model: String, texture: String, scale: (f32, f32, f32)) -> Object{
        Object{
            model: model,
            texture: texture,
            transform: math::calculate_transform(Point3::new(0.0,0.0,0.0), Quaternion::new(0.0,0.0,0.0,0.0), scale),
            tex_wrap: (0.0,0.0),
            scale: scale
        }
    }
    pub fn calculate_transform(&mut self, pos: Point3<f32>, rot: Quaternion<f32>){
        self.transform = math::calculate_transform(pos, rot, self.scale);
    }
    pub fn set_wrap(&mut self, tex_wrap: (f32, f32)){
        self.tex_wrap = tex_wrap;
    }
    pub fn forward(&self) -> Vector3<f32>{
        use alga::linear::Transformation;
        let mut point = Vector3::new(0.0, 0.0, 1.0);
        let matrix = self.transform;
        point = matrix.transform_vector(&point);
        point
    }
}

impl Mouse{
    pub fn new() -> Mouse{
        Mouse{
            position: (0, 0),
            releative: (0, 0)
        }
    }
    pub fn update(&mut self, new_pos: (i32, i32)){
        self.releative.0 = self.position.0 - new_pos.0;
        self.releative.1 = self.position.1 - new_pos.1;
        self.position = new_pos;
    }
}

impl Window {
    pub fn new(sizex: u32, sizey: u32, title: &'static str) -> Window{

        let events_loop = glutin::EventsLoop::new();

        let window = WindowBuilder::new()
            .with_dimensions(sizex, sizey)
            .with_title(title);
        let context = ContextBuilder::new()
            .with_depth_buffer(24);

        let display = Display::new(window, context, &events_loop).unwrap();
        let _ = display.gl_window().window().set_cursor_state(glutin::CursorState::Hide);

        let camera = Camera::new(sizex, sizey);

        Window{
            events_loop: events_loop,
            draw_context: DrawContext{
                display, render_buffer: draw_context::RenderBuffer{
                    vertex_buffers: HashMap::new(),
                    texture_buffer: HashMap::new(),
                    shaders: HashMap::new()
                },
                render_data: HashMap::new(),
                camera: camera,
                scr_res: (sizex, sizey)
            },
            events: vec![],
            mouse: Mouse::new()
        }
    }

    pub fn get_display(&mut self) -> (&mut DrawContext, &mut EventsLoop){
        (&mut self.draw_context, &mut self.events_loop)
    }

    pub fn update(&mut self){
        use glium::glutin::Event::WindowEvent;
        use glium::glutin;

        let mut events = vec![];
        let mut mouse_pos = self.mouse.position;

        self.events_loop.poll_events(|ev| {
            events.push(ev.clone());
            match ev{
                WindowEvent { ref event, .. } => match event{
                    &glutin::WindowEvent::CursorMoved{position, ..} => {
                        mouse_pos = (position.0 as i32, position.1 as i32);
                    },
                    _ => {}
                },
                _ => {}
            }
        });
        if mouse_pos != (-1, -1){
            self.mouse.update((mouse_pos.0 as i32, mouse_pos.1 as i32));
        }
        self.events = events;
    }
}
pub fn get_params() -> glium::DrawParameters<'static>{
    glium::DrawParameters {
        depth: glium::Depth {
            test: glium::DepthTest::IfLess,
            write: true,
            .. Default::default()
        },
        backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
        .. Default::default()
    }
}

pub const SHADER_SIMPLE_FRAG: &'static str = r#"
#version 140
in vec3 v_normal;
in vec2 v_tex_coords;
out vec4 color;

uniform sampler2D tex;
uniform vec2 wrap;

void main() {
    vec3 u_light = vec3(0.1,0.1,0.4);
    float brightness = dot(normalize(v_normal), normalize(u_light));
    vec3 dark_color = vec3(0.7, 0.7, 0.7) * vec3(texture(tex, v_tex_coords + wrap));
    vec3 regular_color = vec3(1.0, 1.0, 1.0) * vec3(texture(tex, v_tex_coords + wrap));
    color = vec4(mix(dark_color, regular_color, brightness), 1.0);
}
"#;

pub const SHADER_SIMPLE_VERT: &'static str = r#"
#version 330

in vec3 position;
in vec3 normal;
in vec2 tex_coords;
out vec3 v_normal;
out vec2 v_tex_coords;
uniform mat4 perspective;
uniform mat4 matrix;
uniform mat4 view;
void main() {
    mat4 modelview = view * matrix;
    v_normal = normal;
    gl_Position = perspective * modelview * vec4(position, 1.0);
    v_tex_coords = tex_coords;
}
"#;

pub const SHADER_DISTORTION_FRAG: &'static str = r#"
#version 330

//per eye texture to warp for lens distortion
uniform sampler2D warpTexture;

//Position of lens center in m (usually eye_w/2, eye_h/2)
uniform vec2 LensCenter;
//Scale from texture co-ords to m (usually eye_w, eye_h)
uniform vec2 ViewportScale;
//Distortion overall scale in m (usually ~eye_w/2)
uniform float WarpScale;
//Distoriton coefficients (PanoTools model) [a,b,c,d]
uniform vec4 HmdWarpParam;

//chromatic distortion post scaling
uniform vec3 aberr;

in vec2 T;
out vec4 color;

void main()
{
    //output_loc is the fragment location on screen from [0,1]x[0,1]
    vec2 output_loc = vec2(T.s, T.t);
    //Compute fragment location in lens-centered co-ordinates at world scale
    vec2 r = output_loc * ViewportScale - LensCenter;
    //scale for distortion model
    //distortion model has r=1 being the largest circle inscribed (e.g. eye_w/2)
    r /= WarpScale;

    //|r|**2
    float r_mag = length(r);
    //offset for which fragment is sourced
    vec2 r_displaced = r * (HmdWarpParam.w + HmdWarpParam.z * r_mag +
    HmdWarpParam.y * r_mag * r_mag +
    HmdWarpParam.x * r_mag * r_mag * r_mag);
    //back to world scale
    r_displaced *= WarpScale;
    //back to viewport co-ord
    vec2 tc_r = (LensCenter + aberr.r * r_displaced) / ViewportScale;
    vec2 tc_g = (LensCenter + aberr.g * r_displaced) / ViewportScale;
    vec2 tc_b = (LensCenter + aberr.b * r_displaced) / ViewportScale;

    float red = texture(warpTexture, tc_r).r;
    float green = texture(warpTexture, tc_g).g;
    float blue = texture(warpTexture, tc_b).b;
    //Black edges off the texture
    color = ((tc_g.x < 0.0) || (tc_g.x > 1.0) || (tc_g.y < 0.0) || (tc_g.y > 1.0)) ? vec4(0.0, 0.0, 0.0, 1.0) : vec4(red, green, blue, 1.0);
}
"#;

pub const SHADER_DISTORTION_VERT: &'static str = r#"
#version 330
layout (location=0) in vec2 coords;
uniform mat4 mvp;
out vec2 T;
void main(void)
{
    T = coords;
    gl_Position = mvp * vec4(coords, 0.0, 1.0);
}
"#;
