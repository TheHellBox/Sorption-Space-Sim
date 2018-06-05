#![allow(dead_code)]
pub extern crate tobj;

use glium;
use glium::Display;
use render::Vertex;

use std::collections::HashMap;
use glium::vertex::VertexBufferAny;

pub fn load(data: String) -> Vec<Vertex> {
    use std::path::Path;
    let raw = tobj::load_obj(&Path::new(&data));
    assert!(raw.is_ok());
    let (models, _) = raw.unwrap();
    let mut vertex_data = Vec::new();

    for model in &models {
        let mesh = &model.mesh;
        for idx in &mesh.indices {
            let i = *idx as usize;
            let pos = [mesh.positions[3 * i], mesh.positions[3 * i + 1], mesh.positions[3 * i + 2]];
            let normal =
                if !mesh.normals.is_empty() {
                    [mesh.normals[3 * i], mesh.normals[3 * i + 1], mesh.normals[3 * i + 2]]
                } else {
                    [0.0, 0.0, 0.0]
            };
            let texcord =
                if !mesh.texcoords.is_empty() {
                    [mesh.texcoords[i * 2], mesh.texcoords[i * 2 + 1]]
                } else {
                    [0.0, 0.0]
            };
            vertex_data.push(Vertex {
                position: pos,
                normal: normal,
                tex_coords: texcord
            });
        }
    }
    vertex_data
}

pub fn load_as_vb(data: String, disp: &Display) -> VertexBufferAny{
    let raw = load(data);
    let mesh = glium::vertex::VertexBuffer::new(disp, &raw).unwrap().into_vertex_buffer_any();
    mesh
}

pub fn gen_buffer(disp: &Display) -> Option<HashMap<String, VertexBufferAny>>{
    use std::fs;
    use std::path::Path;
    let dir = Path::new("./assets/models/");
    if dir.exists(){
        let paths = fs::read_dir(dir).unwrap();
        let mut models: HashMap<String, VertexBufferAny> = HashMap::with_capacity(1024);
        for path in paths {
            let path = path.unwrap().path();
            if path.is_file() {
                let name = path.display().to_string();
                if path.extension().unwrap() == "obj"{
                    print!("Loading model {} ... ", path.display());
                    let raw = load(path.display().to_string());
                    let mesh = glium::vertex::VertexBuffer::new(disp, &raw).unwrap().into_vertex_buffer_any();

                    models.insert(name, mesh);

                    println!("Done!");
                }
            }
        }
        Some(models)
    }
    else{
        println!("Path ./assets/models/ doesn't exist");
        None
    }
}
