pub struct Object {
    position:(f64, f64),
    size:(f64, f64),
    color:(u8, u8, u8)
}

impl Object {
    pub fn new(size_x: f64, size_y: f64) -> Object {
        Object {
            position: (0.0, 0.0),
            size: (size_x, size_y),
            color: (255,255,255)
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

    pub fn set_color(&mut self, r: u8, g: u8, b: u8) {
        self.color.0 = r;
        self.color.1 = g;
        self.color.2 = b;
    }

    pub fn get_color(&self) -> [f32; 4] {
        [(self.color.0/255).into(), (self.color.1/255).into(), (self.color.2/255).into(), 1.0 ]
    }

    pub fn to_string(&self) -> String {
        format!("x: {}, y: {} ", self.position.0.to_string(), self.position.1.to_string())
    }
}