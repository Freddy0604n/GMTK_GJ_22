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

    fn pardia(first: (f32, f32), second: (f32, f32)) -> (f32, f32) { // calculcates the diagonal of a parralelogram
        let difference = (first.1 - second.1).abs();
        let length = ((first.0 + difference.cos() * second.0).powf(2.0) + (difference.sin() * second.0).powf(2.0)).sqrt();
        let degree_from_base = ((difference.sin() * second.0)/ length).asin();
        let degree: f32;
        if first.1 < second.1 {
            degree = first.1 + degree_from_base;
        } else {
            degree = first.1 - degree_from_base;
        }
        (length, degree)
    }

    pub fn apply_force(&mut self, force: f32, direction: f32) {
        let acceleration = force / self.mass;
        let result = RigidBody::pardia((acceleration, direction), (self.velocity, self.vel_direction));
        self.velocity = result.0;
        self.vel_direction = result.1;
    }


}