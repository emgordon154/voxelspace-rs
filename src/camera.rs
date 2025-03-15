pub struct Camera {
    pub x: f64,
    pub y: f64,
    pub phi: f64
}

impl Camera {
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            phi: 0.0
        }
    }
}