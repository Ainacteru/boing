

pub mod transform {
    use graphics::{Context, Transformed};

    pub fn translate(c : &Context, original_pos:(f64,f64), new_pos: (f64,f64)) -> [[f64; 3]; 2] {
        c
            .transform
            .trans(original_pos.0, original_pos.1)
            .scale(1.0, -1.0)
            .trans(new_pos.0, new_pos.1)
    }

    pub fn non_moving(c : &Context, pos:(f64,f64)) -> [[f64; 3]; 2] {
        c
            .transform
            .trans(pos.0, pos.1)
            .scale(1.0, -1.0)
    }
}