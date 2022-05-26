#[derive(Debug)]
pub struct RigidBody {
    velocity: (f32, f32),
    mass: f32, // in kg
    width: usize,
    height: usize,
    position: (f32, f32)
}

impl RigidBody {
    pub fn new(width: usize, height: usize, position: (f32, f32), mass: f32) -> RigidBody {
        RigidBody {
            velocity: (0.0, 0.0),
            mass,
            width, 
            height,
            position,
        }
    }

    pub fn apply_force(&mut self, force_vec: (f32, f32)) {
        self.velocity.0 += force_vec.0 / self.mass;
        self.velocity.1 += force_vec.1 / self.mass;
    }

    pub fn update_position(&mut self) {
        self.position.0 += self.velocity.0;
        self.position.1 += self.velocity.1;
    }
}