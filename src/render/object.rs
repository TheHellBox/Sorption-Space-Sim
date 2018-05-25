use support;
use glium::texture::Texture2d;
use nalgebra::geometry::{Quaternion, Point3};
use nalgebra::core::{Vector3, Matrix4};
use glium::vertex::VertexBufferAny;
use support::math;
use render::Window;

// Object that can be rendered
pub struct Object{
    pub model: VertexBufferAny,
    pub texture: Texture2d,
    pub transform: Matrix4<f32>,
    pub tex_wrap: (f32, f32),
    pub scale: (f32, f32, f32),
    pub enabled: bool,
    pub shader: String
}

impl Object{
    pub fn new(model: VertexBufferAny, texture: Texture2d, scale: (f32, f32, f32), shader: String) -> Object{
        Object{
            model: model,
            texture: texture,
            transform: math::calculate_transform(Point3::new(0.0,0.0,0.0), Quaternion::new(0.0,0.0,0.0,0.0), scale),
            tex_wrap: (0.0,0.0),
            scale: scale,
            enabled: true,
            shader: shader
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

pub struct ObjectBuilder{
    pub model: String,
    pub texture: String,
    pub transform: Matrix4<f32>,
    pub tex_wrap: (f32, f32),
    pub scale: (f32, f32, f32),
    pub enabled: bool,
    pub shader: String
}
// A easy way to construct game object. P.S Performance is really bad
impl ObjectBuilder{
    pub fn new() -> ObjectBuilder{
        let default_model = "./assets/models/cube.obj".to_string();
        let default_texture = "./assets/textures/spaceship_cockpit.png".to_string();
        ObjectBuilder{
            model: default_model,
            texture: default_texture,
            transform: math::calculate_transform(Point3::new(0.0,0.0,0.0), Quaternion::new(0.0,0.0,0.0,0.0), (0.1, 0.1, 0.1)),
            tex_wrap: (0.0,0.0),
            scale: (0.1, 0.1, 0.1),
            enabled: true,
            shader: "simple".to_string()
        }
    }

    pub fn with_shader(self, shader: String) -> Self{
        ObjectBuilder{
            shader,
            ..self
        }
    }
    pub fn with_texture(self, texture: String) -> Self{
        ObjectBuilder{
            texture,
            ..self
        }
    }
    pub fn with_model(self, model: String) -> Self{
        ObjectBuilder{
            model,
            ..self
        }
    }
    pub fn with_scale(self, scale: (f32, f32, f32)) -> Self{
        ObjectBuilder{
            scale,
            ..self
        }
    }
    pub fn with_transform(self, transform: Matrix4<f32>) -> Self{
        ObjectBuilder{
            transform,
            ..self
        }
    }
    pub fn with_enabled(self, enabled: bool) -> Self{
        ObjectBuilder{
            enabled,
            ..self
        }
    }
    pub fn build(self, window: &Window) -> Object{
        let model = support::obj_loader::load_as_vb(self.model, &window.draw_context.display);
        let texture = support::texture_loader::load(self.texture, &window.draw_context.display);
        Object{
            model: model,
            texture: texture,
            transform: self.transform,
            tex_wrap: self.tex_wrap,
            scale: self.scale,
            enabled: self.enabled,
            shader: self.shader,
        }
    }
    pub fn build_with_assets(self, window: &Window, model: VertexBufferAny, texture: Texture2d) -> Object{
        Object{
            model: model,
            texture: texture,
            transform: self.transform,
            tex_wrap: self.tex_wrap,
            scale: self.scale,
            enabled: self.enabled,
            shader: self.shader,
        }
    }
    pub fn build_with_texture(self, window: &Window, texture: Texture2d) -> Object{
        let model = support::obj_loader::load_as_vb(self.model, &window.draw_context.display);
        Object{
            model: model,
            texture: texture,
            transform: self.transform,
            tex_wrap: self.tex_wrap,
            scale: self.scale,
            enabled: self.enabled,
            shader: self.shader,
        }
    }
    pub fn build_with_model(self, window: &Window, model: VertexBufferAny) -> Object{
        let texture = support::texture_loader::load(self.texture, &window.draw_context.display);
        Object{
            model: model,
            texture: texture,
            transform: self.transform,
            tex_wrap: self.tex_wrap,
            scale: self.scale,
            enabled: self.enabled,
            shader: self.shader,
        }
    }
}
