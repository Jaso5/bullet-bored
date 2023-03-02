// BASIC

// Required
let pos = [50.0,50.0];
let rot = 0.0;
let scale = [1.0, 1.0];
// Bullet state - persistent
let direction = [1.0, 0.0];
const speed = 1.0;

fn init() {
    // Todo, add bullet specific init
}

fn tick(dt: f32) {
    if position.x >= 100 or position.x <= 0 {
        direction[0] = -direction[0];
    }
    let vel = [speed * dt, speed * dt];
    position += 
}