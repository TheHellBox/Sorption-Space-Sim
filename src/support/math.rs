use nalgebra;
use nalgebra::geometry::{Point3, UnitQuaternion, Quaternion, Translation3};
use nalgebra::core::{Matrix4};

pub fn m16_to_4x4(mat: [f32; 16]) -> [[f32;4]; 4]{
    let mat = [
        [mat[0], mat[1], mat[2], mat[3]],
        [mat[4], mat[5], mat[6], mat[7]],
        [mat[8], mat[9], mat[10], mat[11]],
        [mat[12], mat[13], mat[14], mat[15]],
    ];
    mat
}

pub fn nalg_to_4x4(mat: nalgebra::core::MatrixN<f32, nalgebra::core::dimension::U4>) -> [[f32;4]; 4]{
    let mat = [
        [mat[0], mat[1], mat[2], mat[3]],
        [mat[4], mat[5], mat[6], mat[7]],
        [mat[8], mat[9], mat[10], mat[11]],
        [mat[12], mat[13], mat[14], mat[15]],
    ];
    mat
}

pub fn mat16_to_nalg(mat: [f32;16]) -> nalgebra::core::MatrixN<f32, nalgebra::core::dimension::U4>{
    let mut raw: nalgebra::core::MatrixN<f32, nalgebra::core::dimension::U4> = nalgebra::core::MatrixN::new_scaling(0.0);
    for x in 0..16{
        raw[x] = mat[x];
    }
    raw
}

pub fn mat_to_nalg(mat: [[f32;4]; 4]) -> nalgebra::core::MatrixN<f32, nalgebra::core::dimension::U4>{
    let mut raw: nalgebra::core::MatrixN<f32, nalgebra::core::dimension::U4> = nalgebra::core::MatrixN::new_scaling(0.0);
    for x in 0..4{
        for y in 0..4{
            raw[y*4 + x] = mat[y][x];
        }
    }
    raw
}

pub fn lerp(val1: f32, val2: f32, t: f32) -> f32{
    val1 + t * (val2 - val1)
}

pub fn calculate_transform(pos: Point3<f32>, rot: Quaternion<f32>, scale: (f32, f32, f32)) -> Matrix4<f32>{
    let scale_matrix = Matrix4::new(
        scale.0, 0.0, 0.0, 0.0,
        0.0, scale.1, 0.0, 0.0,
        0.0, 0.0, scale.2, 0.0,
        0.0, 0.0, 0.0, 1.0,
    );
    let translation_matrix = Translation3::from_vector(pos.coords).to_homogeneous();
    let rotation_matrix = UnitQuaternion::from_quaternion(rot).to_homogeneous();
    translation_matrix * scale_matrix * rotation_matrix
}
