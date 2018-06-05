pub mod draw_context;
pub mod std_shaders;
pub mod object;

use self::draw_context::DrawContext;

use glium;
use glium::{glutin, Display};
use glium::glutin::{ContextBuilder, EventsLoop, WindowBuilder};
use glium::glutin::Event;
use std::collections::hash_map::HashMap;

use camera::Camera;

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
    pub focused: bool,
    pub res: (u32, u32),
    pub mouse: Mouse
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
#[allow(dead_code)]
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
                    shaders: HashMap::new()
                },
                render_data: HashMap::new(),
                camera: camera,
                scr_res: (sizex, sizey)
            },
            events: vec![],
            focused: true,
            res: (sizex, sizey),
            mouse: Mouse::new()
        }
    }

    pub fn get_display(&mut self) -> (&mut DrawContext, &mut EventsLoop){
        (&mut self.draw_context, &mut self.events_loop)
    }
    pub fn set_mouse_pos(&mut self, x: i32, y: i32){
        let _ = self.draw_context.display.gl_window().window().set_cursor_position(x, y);
    }
    pub fn update(&mut self){
        use glium::glutin::Event::WindowEvent;
        use glium::glutin;

        let mut events = vec![];
        let mut mouse_pos = self.mouse.position;
        let mut focused = None;
        self.events_loop.poll_events(|ev| {
            events.push(ev.clone());
            match ev{
                WindowEvent { ref event, .. } => match event{
                    &glutin::WindowEvent::CursorMoved{position, ..} => {
                        mouse_pos = (position.0 as i32, position.1 as i32);
                    },
                    &glutin::WindowEvent::Focused(focus) => {
                        focused = Some(focus);
                    },
                    _ => {}
                },
                _ => {}
            }
        });
        if focused.is_some(){
            self.focused = focused.unwrap();
        }
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
