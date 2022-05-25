mod window;

use std::thread;

fn main() {
    let handle = thread::spawn(move || {window::start(vec![window::Sprite::new(200, 3, 3, vec![0xFFFFFF; 9], 1)], vec![0x444444; 500 * 800], 500, 800)});
    let res = handle.join();
    println!("Closed window");
}
