#[derive(Debug)]
pub struct RigidBody {
    velocity: f32,
    vel_direction: f32, // according to degrees
    mass: f32, // in kg
    width: usize,
    height: usize,
    position: usize
}

impl RigidBody {
    pub fn new(width: usize, height: usize, position: usize, mass: f32) -> RigidBody {
        RigidBody {
            velocity: 0.0,
            vel_direction: 90.0,
            mass,
            width, 
            height,
            position,
        }
    }

    pub fn apply_force(&mut self, force: f32, direction: f32) {
        let acceleration = force / self.mass;
        self.velocity += acceleration;
    }
}