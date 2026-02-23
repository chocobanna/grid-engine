pub struct Physics {
    pub position: [f32; 2],
    pub velocity: [f32; 2],
    pub rotation: f32,
}

impl Physics {
    pub fn new() -> Self {
        Self {
            position: [0.0, 0.0],
            velocity: [0.6, 0.45],
            rotation: 0.0,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.position[0] += self.velocity[0] * dt;
        self.position[1] += self.velocity[1] * dt;

        let bounds = 0.8;

        if self.position[0] > bounds || self.position[0] < -bounds {
            self.velocity[0] *= -1.0;
        }

        if self.position[1] > bounds || self.position[1] < -bounds {
            self.velocity[1] *= -1.0;
        }

        self.rotation += dt * 2.0;
    }
}