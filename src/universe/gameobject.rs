use render::Object;
use nalgebra::geometry::{Quaternion, Point3, UnitQuaternion};

pub struct Game_Object{
    // Id of the game object
    pub id: u32,
    // Name of the Game Object
    pub name: String,
    // Mesh
    pub render_object: Option<Object>,
    // Global position
    pub global_position: Point3<f32>,
    // Position releative to the parent's
    pub local_position: Point3<f32>,
    // Rotation of the object releative to the parent's
    pub local_rotation: UnitQuaternion<f32>,
    // Parent ID
    pub parent: u32
}

impl Game_Object{
    // Create new game object
    pub fn new(id: u32, name: String) -> Game_Object{
        Game_Object{
            id: id,
            name: name,
            render_object: None,
            global_position: Point3::new(0.0,0.0,0.0),
            local_position: Point3::new(0.0,0.0,0.0),
            local_rotation: UnitQuaternion::from_quaternion(Quaternion::new(0.0,0.0,0.0,1.0)),
            parent: 0
        }
    }
    // Set parent of the game object
    pub fn set_parent(&mut self, id: u32){
        self.parent = id;
    }
    // Set render object (render::Object)
    pub fn set_render_object(&mut self, object: Object){
        self.render_object = Some(object);
    }
    // Set position releative to the parent, if there no parent, then it will set global position
    pub fn set_local_position(&mut self, position: Point3<f32>){
        self.local_position = position;
    }
    // Set global position
    pub fn set_position(&mut self, position: Point3<f32>){
        self.global_position = position;
    }
    pub fn set_rotation(&mut self, quat: Quaternion<f32>){
        self.local_rotation = UnitQuaternion::from_quaternion(quat);
    }

    pub fn update(&mut self, parent: Option<&Game_Object> ){
        // Update positions and other stuff, WIP
        match parent{
            Some(ref x) => {
                let delta_rot = x.local_rotation * self.local_rotation.inverse();
                //self.global_position = x.global_position * self.local_position.coords;
            }
            None => {
                //self.global_position = self.local_position
            }
        }
        match self.render_object{
            Some(ref mut x) => {
                x.position = x.position + self.global_position.coords;
            }
            None => {}
        }
    }
}
