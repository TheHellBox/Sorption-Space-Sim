use glium::texture::Texture2d;

pub struct Widget_Base {
    pub position: (f32, f32),
    pub size: (f32, f32),
    pub color: (u8, u8, u8),
    pub texture: Texture2d
}

pub struct Button{
    pub base: Widget_Base,
    pub text: String
}
