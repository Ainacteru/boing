pub struct Object {
    position:(f64, f64),
    size:(f64,f64),
}

impl Object {
    pub fn new(x_pos: f64, y_pos:f64, size_x: f64, size_y: f64) -> Object {
        Object {
            position : (x_pos, y_pos),
            size : (size_x, size_y),
        }
    }

    pub fn get_position(&self) -> (f64, f64) {
        self.position
    }

    pub fn set_position(&mut self, x_pos: f64, y_pos:f64) {
        self.position.0 = x_pos;
        self.position.1 = y_pos;
    }

    pub fn get_size(&self) -> (f64, f64) {
        self.size
    }

    pub fn to_string(&self) -> String {
        format!("x: {}, y: {} ", self.position.0.to_string(), self.position.1.to_string())
    }
}