use crate::object::Object;

pub mod object;

fn main() {
    let mut ball:Object = Object::new(0.0,0.0,1.0,1.0);

    let mut pos: (f64, f64) = (0.0, 0.0);

    loop {

        ball.set_position(pos.0, pos.1);

        println!("{}", ball.to_string());

        pos.0 += 1.0;
        pos.1 += 1.0;
    }

}
