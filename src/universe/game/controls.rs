use glium::glutin::Event;

#[derive(Copy, Clone)]
pub struct Controls{
    pub up: f32,
    pub down: f32,
    pub forward: f32,
    pub back: f32,
    pub right: f32,
    pub left: f32,
}

impl Controls{
    pub fn new() -> Controls{
        Controls{
            up: 0.0,
            down: 0.0,
            forward: 0.0,
            back: 0.0,
            right: 0.0,
            left: 0.0
        }
    }
    pub fn update(&mut self, events: &Vec<Event>){
        use glium::glutin;
        use glium::glutin::{Event, WindowEvent};
        for ev in events.to_owned(){
            match ev {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::KeyboardInput {device_id: _, input: input} => {
                        let modif = match input.state{
                            glutin::ElementState::Pressed => 1.0,
                            glutin::ElementState::Released => 0.0
                        };
                        match input.scancode{
                            17 => {
                                self.forward = modif;
                            },
                            31 => {
                                self.back = modif;
                            },
                            30 => {
                                self.right = modif;
                            },
                            32 => {
                                self.left = modif;
                            },
                            19 => {
                                self.up = modif;
                            },
                            29 => {
                                self.down = modif;
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
    }
}
