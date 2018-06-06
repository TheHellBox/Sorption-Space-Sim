pub mod texture_loader;
pub mod font_loader;
pub mod obj_loader;
pub mod math;
pub mod image_m;
/*
pub struct Settings{
    pub resolution: (u32, u32),
    pub vr: bool
}
impl Settings{
    pub fn from_args() -> Settings{
        let args: Vec<String> = env::args().collect();

        let (mut res_x, mut res_y) = (1024, 768);
        let mut vr = false;
        let mut num = 0;
        for x in args.clone(){
            match x{
                "vr" => {
                    if args[num + 1] == "true"{
                        vr = true;
                    }
                    else{
                        vr = false;
                    }
                },
                "resolution" => {
                    res_x = args[num + 1];
                    res_y = args[num + 2];
                },
            }
            num += 1;
        }
        Settings{
            resolution: (res_x, res_y),
            vr: vr
        }

    }
}*/
