trait Movable {
    fn draw(&self);
    fn move_to(&mut self, x: f32, y: f32);
}

struct Node {
    x: f32,
    y: f32,
}

impl Movable for Node {
    fn draw(&self) {
        println!("x={}, y={}", self.x, self.y);
    }

    fn move_to(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }
}

fn main() {
    let m = Node{x: 4.0, y: 4.0};
    m.draw();
}