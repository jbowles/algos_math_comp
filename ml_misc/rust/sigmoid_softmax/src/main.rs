extern crate nalgebra as na;

fn sigmoid(x: f32) -> f32 {
    1.0 / (1.0 + (-x).exp())
}

fn main() {
    println!("sigmoid: {}", sigmoid(10.0));
}
