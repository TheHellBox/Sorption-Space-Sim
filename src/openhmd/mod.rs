use openhmd_rs;
use nalgebra::core::Matrix4;

pub struct HMDParams{
    pub scr_res: (u32, u32),
    pub scr_size: (f32, f32),
    pub left_lens_center: [f32; 2],
    pub right_lens_center: [f32; 2],
    pub view_port_scale: [f32; 2],
    pub distortion_k: [f32; 4],
    pub aberration_k: [f32; 3],
    pub projection1: [[f32;4]; 4],
    pub projection2: [[f32;4]; 4],
    pub warp_scale: f32
}

pub struct OpenHMD{
    pub context: openhmd_rs::Context,
    pub device: openhmd_rs::Device,
    pub config: HMDParams
}

impl OpenHMD{
    pub fn new() -> OpenHMD{
        let context = openhmd_rs::Context::new();
        context.probe();
        context.update();
        let device = context.list_open_device(0);
        let config = gen_cfg(&device);

        println!("\nDevice description: ");
        println!("Vendor:   {}", context.list_gets(0, openhmd_rs::ohmd_string_value::OHMD_VENDOR));
        println!("Product:  {}", context.list_gets(0, openhmd_rs::ohmd_string_value::OHMD_PRODUCT));
        println!("Path:     {}\n", context.list_gets(0, openhmd_rs::ohmd_string_value::OHMD_PATH));
        println!("Opening device {}...", 0);

        OpenHMD{
            context: context,
            device: device,
            config: config
        }
    }
    pub fn get_view(&self) -> (Matrix4<f32>, Matrix4<f32>){
        use support::math::m16_to_4x4;
        use support::math::mat16_to_nalg;
        let view_left = mat16_to_nalg( match self.device.getf(openhmd_rs::ohmd_float_value::OHMD_LEFT_EYE_GL_MODELVIEW_MATRIX){
            Some(x) => x,
            None => [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,0.0, 0.0, 0.0, 0.0,0.0, 0.0, 0.0, 0.0]
        });
        let view_right = mat16_to_nalg( match self.device.getf(openhmd_rs::ohmd_float_value::OHMD_RIGHT_EYE_GL_MODELVIEW_MATRIX){
            Some(x) => x,
            None => [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,0.0, 0.0, 0.0, 0.0,0.0, 0.0, 0.0, 0.0]
        });
        (view_right, view_left)
    }
}

pub fn gen_cfg(device: &openhmd_rs::Device) -> HMDParams{
    use support::math::m16_to_4x4;

    let scrw = match device.geti(openhmd_rs::ohmd_int_value::OHMD_SCREEN_HORIZONTAL_RESOLUTION){
        Some(x) => x,
        _ => 1280
    } as u32;
    println!("{}", scrw);
    let scrh = match device.geti(openhmd_rs::ohmd_int_value::OHMD_SCREEN_VERTICAL_RESOLUTION){
        Some(x) => x,
        _ => 800
    } as u32;

    // Calculating HMD params
    let scr_size_w = match device.getf(openhmd_rs::ohmd_float_value::OHMD_SCREEN_HORIZONTAL_SIZE){
        Some(x) => x[0],
        _ => {
            println!("Something is wrong!");
            0.149760
        }
    };
    let scr_size_h = match device.getf(openhmd_rs::ohmd_float_value::OHMD_SCREEN_VERTICAL_SIZE ){
        Some(x) => x[0],
        _ => 0.093600
    };
    let distortion_k = match device.getf(openhmd_rs::ohmd_float_value::OHMD_UNIVERSAL_DISTORTION_K ){
        Some(x) => [x[0], x[1], x[2], x[3]],
        _ => [0.0,0.0,0.0,1.0]
    };
    let aberration_k = match device.getf(openhmd_rs::ohmd_float_value::OHMD_UNIVERSAL_ABERRATION_K ){
        Some(x) =>  [x[0], x[1], x[2]],
        _ => [0.0,0.0,1.0]
    };

    let view_port_scale = [scr_size_w / 2.0, scr_size_h];

    let sep = match device.getf(openhmd_rs::ohmd_float_value::OHMD_LENS_HORIZONTAL_SEPARATION ){
        Some(x) => x[0],
        _ => 0.063500
    };
    let mut left_lens_center: [f32; 2] = [0.0, match device.getf(openhmd_rs::ohmd_float_value::OHMD_LENS_VERTICAL_POSITION){
        Some(x) => x[0],
        _ => 0.046800
    }];
    let mut right_lens_center: [f32; 2] = [0.0, match device.getf(openhmd_rs::ohmd_float_value::OHMD_LENS_VERTICAL_POSITION){
        Some(x) => x[1],
        _ => 0.046800
    }];

    let oproj = m16_to_4x4( match device.getf(openhmd_rs::ohmd_float_value::OHMD_LEFT_EYE_GL_PROJECTION_MATRIX){
        Some(x) => x,
        _ => [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,0.0, 0.0, 0.0, 0.0,0.0, 0.0, 0.0, 0.0]
    });
    let oproj2 = m16_to_4x4(match device.getf(openhmd_rs::ohmd_float_value::OHMD_RIGHT_EYE_GL_PROJECTION_MATRIX){
        Some(x) => x,
        _ => [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,0.0, 0.0, 0.0, 0.0,0.0, 0.0, 0.0, 0.0]
    });

    left_lens_center[0] = view_port_scale[0] - sep/2.0;
    right_lens_center[0] = sep/2.0;

    HMDParams{

        scr_res: (scrw, scrh),
        scr_size: (scr_size_w, scr_size_h),

        left_lens_center: left_lens_center,
        right_lens_center: right_lens_center,

        view_port_scale: view_port_scale,

        distortion_k: distortion_k,
        aberration_k: aberration_k,

        projection1: oproj,
        projection2: oproj2,

        warp_scale: left_lens_center[0] / right_lens_center[0]
    }
}
