use universe::Universe;

pub fn check_area(ch_area: (i32, i32), universe: &mut Universe){
    let (area_old, area) = match universe.player{
        Some(ref mut x) => {
            let old = x.area;
            x.area.0 += ch_area.0;
            x.area.1 += ch_area.1;
            let new = x.area;
            (old, new)
        }
        None => {
            ((0, 0), (0, 0))
        }
    };

    for obj in universe.get_go_by_area(area_old){
        match obj.render_object{
            Some(ref mut x) => {
                if obj.tags.contains(&"Planet".to_string()) || obj.tags.contains(&"Rings".to_string()){
                    x.enabled = false;
                }
            }
            None => {}
        }
    }
    println!("{:?}", area);
    for obj in universe.get_go_by_area(area){
        match obj.render_object{
            Some(ref mut x) => {
                if obj.tags.contains(&"Planet".to_string()) || obj.tags.contains(&"Rings".to_string()){
                    x.enabled = true;
                }
            }
            None => {}
        }
    }
}
