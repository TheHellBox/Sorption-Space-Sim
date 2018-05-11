pub mod draw_context;

use support::math;

use self::draw_context::DrawContext;

use glium::{glutin, Display};
use glium::glutin::{ContextBuilder, EventsLoop, WindowBuilder};
use glium::glutin::Event;

use std::collections::hash_map::HashMap;

use camera::Camera;

use nalgebra::geometry::{Quaternion, Point3, UnitQuaternion, Translation3};
use nalgebra::core::{Vector3, Matrix4};

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub tex_coords: [f32; 2]
}
implement_vertex!(Vertex, position, normal, tex_coords);

pub struct Window{
    pub events_loop: EventsLoop,
    pub draw_context: DrawContext,
    pub events: Vec<Event>,
    pub mouse_pos: (u32, u32)
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
    pub fn new(model: String, texture: String, scale: (f32, f32, f32), position: Point3<f32>, rotation: Quaternion<f32>) -> Object{
        Object{
            model: model,
            texture: texture,
            transform: math::calculate_transform(position, rotation, scale),
            tex_wrap: (0.0,0.0),
            scale: scale
        }
    }
    pub fn calculate_transform(&mut self, pos: Point3<f32>, rot: Quaternion<f32>){
        let scale_matrix = Matrix4::new(
            self.scale.0, 0.0, 0.0, 0.0,
            0.0, self.scale.1, 0.0, 0.0,
            0.0, 0.0, self.scale.2, 0.0,
            0.0, 0.0, 0.0, 0.0,
        );
        let translation_matrix = Translation3::from_vector(pos.coords).to_homogeneous();
        let rotation_matrix = UnitQuaternion::from_quaternion(rot).to_homogeneous();
        self.transform = scale_matrix * translation_matrix * rotation_matrix;
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
impl Window {
    pub fn new(sizex: u32, sizey: u32, title: &'static str) -> Window{

        let events_loop = glutin::EventsLoop::new();

        let window = WindowBuilder::new()
            .with_dimensions(sizex, sizey)
            .with_title(title);
        let context = ContextBuilder::new()
            .with_depth_buffer(24);

        let display = Display::new(window, context, &events_loop).unwrap();

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
                camera: camera
            },
            events: vec![],
            mouse_pos: (0, 0)
        }
    }

    pub fn get_display(&mut self) -> (&mut DrawContext, &mut EventsLoop){
        (&mut self.draw_context, &mut self.events_loop)
    }

    pub fn update(&mut self){
        use glium::glutin::Event::WindowEvent;
        use glium::glutin;

        let mut events = vec![];
        let mut mouse_pos = (-1.0, -1.0);

        self.events_loop.poll_events(|ev| {
            events.push(ev.clone());
            match ev{
                WindowEvent { ref event, .. } => match event{
                    &glutin::WindowEvent::CursorMoved{position, ..} => {
                        mouse_pos = position;
                    },
                    _ => {}
                },
                _ => {}
            }
        });
        if mouse_pos != (-1.0, -1.0){
            self.mouse_pos.0 = mouse_pos.0 as u32;
            self.mouse_pos.1 = mouse_pos.1 as u32;
        }
        self.events = events;
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
