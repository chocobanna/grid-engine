// src/scene.rs
use glam::Vec3;

pub struct Scene {
    pub verts: Vec<Vec3>,
    pub edges: Vec<(usize, usize)>,
}

impl Scene {
    pub fn cube() -> Self {
        let verts = vec![
            Vec3::new(-1.0,-1.0,-1.0), Vec3::new(1.0,-1.0,-1.0),
            Vec3::new(1.0, 1.0,-1.0), Vec3::new(-1.0, 1.0,-1.0),
            Vec3::new(-1.0,-1.0, 1.0), Vec3::new(1.0,-1.0, 1.0),
            Vec3::new(1.0, 1.0, 1.0),  Vec3::new(-1.0, 1.0, 1.0),
        ];
        let edges = vec![
            (0,1),(1,2),(2,3),(3,0),
            (4,5),(5,6),(6,7),(7,4),
            (0,4),(1,5),(2,6),(3,7),
        ];
        Scene { verts, edges }
    }
}
