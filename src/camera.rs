// src/camera.rs

use glam::{Vec3, Quat};

/// A simple free‑moving camera with yaw (Y‑axis) and pitch (X‑axis) rotation.
pub struct Camera {
    /// World position of the camera.
    pub pos: Vec3,
    /// Rotation around the Y axis (left/right).
    pub yaw: f32,
    /// Rotation around the X axis (up/down).
    pub pitch: f32,
    /// Movement speed multiplier.
    pub speed: f32,
    /// Rotation speed multiplier (radians per mouse‑unit).
    pub rot_speed: f32,
}

impl Camera {
    /// Create a new camera at the origin looking down –Z.
    pub fn new() -> Self {
        Self {
            pos: Vec3::new(0.0, 0.0, -5.0),
            yaw: 0.0,
            pitch: 0.0,
            speed: 0.1,
            rot_speed: 0.002,
        }
    }

    /// Build the camera’s orientation quaternion: yaw around Y, then pitch around X.
    pub fn rotation(&self) -> Quat {
        let q_yaw   = Quat::from_axis_angle(Vec3::Y, self.yaw);
        let q_pitch = Quat::from_axis_angle(Vec3::X, self.pitch);
        q_yaw * q_pitch
    }

    /// The forward (look) vector in world space.
    pub fn forward(&self) -> Vec3 {
        // Z‑axis points forward in camera space.
        self.rotation() * Vec3::Z
    }

    /// The right (strafe) vector in world space.
    pub fn right(&self) -> Vec3 {
        // X‑axis points right in camera space; only yaw matters for horizontal strafing.
        Quat::from_axis_angle(Vec3::Y, self.yaw) * Vec3::X
    }
}
