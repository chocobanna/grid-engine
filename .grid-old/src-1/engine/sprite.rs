#[derive(Clone)]
#[allow(dead_code)]
pub struct Sprite {
    pub vertices: Vec<[f32; 2]>,
    pub position: [f32; 2],
    pub rotation: f32,
    pub scale: f32,
    pub color: [f32; 3],

    pub velocity: [f32; 2],
}

impl Sprite {

    pub fn triangle() -> Self {
        Self {
            vertices: vec![
                [0.0, 0.5],
                [-0.5, -0.5],
                [0.5, -0.5],
            ],
            position: [0.0, 0.0],
            rotation: 0.0,
            scale: 1.0,
            color: [1.0, 1.0, 1.0],
            velocity: [0.0, 0.0],
        }
    }

    pub fn position(mut self, x: f32, y: f32) -> Self {
        self.position = [x, y];
        self
    }

    pub fn velocity(mut self, x: f32, y: f32) -> Self {
        self.velocity = [x, y];
        self
    }

    #[allow(dead_code)]
    pub fn rotation(mut self, r: f32) -> Self {
        self.rotation = r;
        self
    }

    pub fn scale(mut self, s: f32) -> Self {
        self.scale = s;
        self
    }

    pub fn color(mut self, r: f32, g: f32, b: f32) -> Self {
        self.color = [r, g, b];
        self
    }
}