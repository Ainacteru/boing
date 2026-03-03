pub struct Vector {
    position: (f64, f64),
    direction: f64,
    mode: Angle,
    magnitude : f64,
}

#[derive(Clone, Copy)]
pub enum Angle {
    RAD,
    DEG,
}

impl Vector {
    pub fn new(position: (f64, f64), direction: f64, mode:Angle, magnitude: f64) -> Vector{
        Vector {
            position,
            direction,
            mode,
            magnitude,
        }
    }
    pub fn get_direction(&self, angle: Angle) -> f64 {
        match (&self.mode, angle) {
            (Angle::RAD, Angle::RAD) => self.direction,
            (Angle::RAD, Angle::DEG) => self.direction * (180.0 / std::f64::consts::PI),
            (Angle::DEG, Angle::RAD) => self.direction * (std::f64::consts::PI / 180.0),
            (Angle::DEG, Angle::DEG) => self.direction,
        }
    }  
    pub fn get_magnitude(&self) -> f64 {
        self.magnitude
    }
    pub fn normal(&self) -> Vector {
        Vector {
            position: self.position,
            direction: self.direction + 90.0,
            mode: self.mode,
            magnitude: self.magnitude,
        }
    }
    pub fn dot(&self, vector: Vector) -> f64{
        self.position.0 * vector.position.0 + self.position.1 * vector.position.1
    }
    
}

impl Vector {
    pub fn to_string(&self) -> String {
        format!(
            "Position: (x: {}, y: {}), directions in {}: {} with magnitude: {}",
            self.position.0,
            self.position.1,
            self.mode.to_string(),
            self.direction,
            self.magnitude
        )
    }
}

impl Angle {
    pub fn to_string(&self) -> String {
        match self {
            Angle::DEG => "degrees".to_string(),
            Angle::RAD => "radians".to_string()
        }
    }
}