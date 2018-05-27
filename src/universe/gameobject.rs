use render::object::Object;
use nalgebra::geometry::{Quaternion, Point3, UnitQuaternion};
use nalgebra::Vector3;

pub struct GameObject{
    // Id of the game object
    pub id: u32,
    // Name of the Game Object
    pub name: String,
    // Mesh
    pub render_object: Option<Object>,
    // Global position
    pub position: Point3<f32>,
    // Rotation of the object releative to the parent's
    pub rotation: UnitQuaternion<f32>,
    // Parent ID
    pub childs: Vec<GameObject>,

    pub tags: Vec<String>,

    pub area: (i32, i32)
}

impl GameObject{
    // Create new game object
    pub fn new(id: u32, name: String) -> GameObject{
        GameObject{
            id: id,
            name: name,
            render_object: None,
            position: Point3::new(0.0,0.0,0.0),
            rotation: UnitQuaternion::from_quaternion(Quaternion::new(0.0,0.0,1.0,0.0)),
            childs: vec![],
            tags: vec![],
            area: (0, 0)
        }
    }
    // Set parent of the game object
    pub fn add_child(&mut self, child: GameObject){
        self.childs.push(child);
    }
    // Set render object (render::object::Object)
    pub fn set_render_object(&mut self, object: Object){
        self.render_object = Some(object);
    }
    // Set global position
    pub fn set_position(&mut self, position: Point3<f32>){
        self.position = position;
    }
    pub fn set_rotation(&mut self, quat: Quaternion<f32>){
        self.rotation = UnitQuaternion::from_quaternion(quat);
    }
    pub fn forward(&mut self) -> Vector3<f32>{
        self.direction(Vector3::new(0.0, 0.0, -1.0))
    }
    pub fn right(&mut self) -> Vector3<f32>{
        self.direction(Vector3::new(-1.0, 0.0, 0.0))
    }
    pub fn up(&mut self) -> Vector3<f32>{
        self.direction(Vector3::new(0.0, 1.0, 0.0))
    }
    pub fn set_area(&mut self, area: (i32, i32)){
        self.area = area;
    }
    pub fn direction(&mut self, vec: Vector3<f32>) -> Vector3<f32>{
        use alga::linear::Transformation;
        let mut point = vec;
        let matrix = self.rotation.to_homogeneous();
        point = matrix.transform_vector(&point);
        point
    }
    pub fn update(&mut self){
        // Update positions and other stuff, WIP

        /*
        match parent{
            Some(ref x) => {
                let delta_rot = x.local_rotation * self.local_rotation.inverse();
                //self.global_position = x.global_position * self.local_position.coords;
            }
            None => {
                //self.global_position = self.local_position
            }
        }*/

        match self.render_object{
            Some(ref mut x) => {
                //x.position = x.position + self.global_position.coords;
                x.calculate_transform(self.position, *self.rotation.quaternion())
            }
            None => {}
        }
        for x in &mut self.childs{
            //let rotation = (self.rotation * x.rotation);

            x.update();
        }
    }
}

pub struct GameObjectBuilder{
    // Id of the game object
    pub id: u32,
    // Name of the Game Object
    pub name: String,
    // Mesh
    pub render_object: Option<Object>,
    // Global position
    pub position: Point3<f32>,
    // Rotation of the object releative to the parent's
    pub rotation: UnitQuaternion<f32>,
    // Parent ID
    pub childs: Vec<GameObject>,

    pub tags: Vec<String>,

    pub area: (i32, i32)
}
impl GameObjectBuilder{
    pub fn new() -> GameObjectBuilder{
        GameObjectBuilder{
            id: 0,
            name: String::new(),
            render_object: None,
            position: Point3::new(0.0, 0.0, 0.0),
            rotation: UnitQuaternion::from_quaternion(Quaternion::new(0.0,0.0,1.0,0.0)),
            childs: vec![],
            tags: vec![],
            area: (0, 0)
        }
    }
    pub fn with_id(self, id: u32) -> Self{
        GameObjectBuilder{
            id,
            ..self
        }
    }
    pub fn with_name(self, name: String) -> Self{
        GameObjectBuilder{
            name,
            ..self
        }
    }
    pub fn with_render_object(self, render_object: Object) -> Self{
        GameObjectBuilder{
            render_object: Some(render_object),
            ..self
        }
    }
    pub fn with_position(self, position: Point3<f32>) -> Self{
        GameObjectBuilder{
            position,
            ..self
        }
    }
    pub fn with_rotation(self, rotation: UnitQuaternion<f32>) -> Self{
        GameObjectBuilder{
            rotation,
            ..self
        }
    }
    pub fn with_childs(self, childs: Vec<GameObject>) -> Self{
        GameObjectBuilder{
            childs,
            ..self
        }
    }
    pub fn with_tags(self, tags: Vec<String>) -> Self{
        GameObjectBuilder{
            tags,
            ..self
        }
    }
    pub fn with_area(self, area: (i32, i32)) -> Self{
        GameObjectBuilder{
            area,
            ..self
        }
    }
    pub fn build(self) -> GameObject{
        GameObject{
            id: self.id,
            name: self.name,
            render_object: self.render_object,
            position: self.position,
            rotation: self.rotation,
            childs: self.childs,
            tags: self.tags,
            area: self.area
        }
    }
}
