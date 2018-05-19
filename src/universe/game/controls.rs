use render::Window;
use glium::glutin::Event;

#[derive(Copy, Clone)]
pub struct Controls{
    pub up: f32,
    pub down: f32,
    pub forward: f32,
    pub back: f32,
    pub right: f32,
    pub left: f32,
    pub speed: f32,

    pub roll: f32,

    pub rel: (f32, f32)
}

impl Controls{
    pub fn new() -> Controls{
        Controls{
            up: 0.0,
            down: 0.0,
            forward: 0.0,
            back: 0.0,
            right: 0.0,
            left: 0.0,
            speed: 1.0,
            roll: 0.0,
            rel: (0.0, 0.0)
        }
    }
    pub fn update(&mut self, window: &Window){
        use glium::glutin;
        use glium::glutin::{Event, WindowEvent};
        let mut roll = 0.0;
        for ev in window.events.to_owned(){
            match ev {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::KeyboardInput {device_id: _, input} => {
                        let modif = match input.state{
                            glutin::ElementState::Pressed => 1.0,
                            glutin::ElementState::Released => 0.0
                        };
                        let modif_s = modif * self.speed;

                        match input.scancode{
                            17 => {
                                self.forward = modif_s;
                            },
                            31 => {
                                self.back = modif_s;
                            },
                            30 => {
                                self.right = modif_s;
                            },
                            32 => {
                                self.left = modif_s;
                            },
                            19 => {
                                self.up = modif_s;
                            },
                            29 => {
                                self.down = modif_s;
                            },
                            42 => {
                                self.speed = (modif * 2.0) + 1.0;
                            },

                            16 => {
                                roll = modif_s;
                            },
                            18 => {
                                roll = -modif_s;
                            },

                            _ => {
                                println!("{}", input.scancode);
                            }
                        }
                    },
                    _ => {}
                },
                _ => {}
            }
        }
        if roll != 0.0{
            self.roll += roll / 20.0;
        }
        let rel = window.mouse.releative;
        self.rel.0 += rel.0 as f32;
        self.rel.1 += rel.1 as f32;
    }
}
