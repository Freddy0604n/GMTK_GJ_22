mod window;
mod physics;

use std::thread;
use std::sync::mpsc;

fn main() {
    let handle = thread::spawn(move || {window::start(vec![window::Sprite::new(200, 3, 3, vec![0xFFFFFF; 9]), window::Sprite::new(150 * 500, 320, 1, vec![0xFFFFFF; 320])], vec![0x444444; 500 * 800], 500, 800)});
    println!("Game running in window...");
    let mut player = physics::RigidBody::new(2, 2, 0, 80.5);
    player.apply_force(20.0, 90.0);
    let _res = handle.join();
    println!("{:?}", player);
}
