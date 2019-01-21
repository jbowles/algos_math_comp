extern crate nalgebra as na;
use na::{Vector3, Vector4};

// ./target/debug/nnrs
fn main() {
    version1();
    version2();
    version3();
    version4();
    version4_1();
}

fn version1() {
    let weight: f32 = 0.1;
    let number_of_toes = [8.5, 9.5, 10.0, 9.0];
    let input = number_of_toes[0 as usize];
    let pred = neural_network(input, weight);
    println!("version1: {}", pred)
}

fn neural_network(input: f32, weight: f32) -> f32 {
    let prediction = input * weight;
    prediction
}

fn version2() {
    let toes = vec![8.5, 9.5, 9.9, 9.0];
    let wlrec = vec![0.65, 0.8, 0.8, 0.9];
    let nfans = vec![1.2, 1.3, 0.5, 1.0];
    let weights = vec![0.1, 0.2, 0.0];
    let input = vec![toes[0], wlrec[0], nfans[0]];
    let pred = neural_network_n(input, weights);
    println!("version2: {}", pred)
}

fn neural_network_n(input: Vec<f32>, weights: Vec<f32>) -> f32 {
    let prediction = w_sum(input, weights);
    prediction
}

fn w_sum(a: Vec<f32>, b: Vec<f32>) -> f32 {
    let mut output: f32 = 0.0;
    for i in 0..a.len() {
        output += a[i] * b[i]
    }
    output
}

#[allow(dead_code)]
fn w_sum_n(a: Vec<f32>, b: Vec<f32>) -> f32 {
    let mut output: f32 = 0.0;
    for (_, (aval, bval)) in a.iter().zip(&b).enumerate() {
        output += aval * bval
    }
    output
}

fn version3() {
    let toes = Vector4::new(8.5, 9.5, 9.9, 9.0);
    let wlrec = Vector4::new(0.65, 0.8, 0.8, 0.9);
    let nfans = Vector4::new(1.2, 1.3, 0.5, 1.0);
    let weights = Vector3::new(0.1, 0.2, 0.0);
    let input = Vector3::new(toes[0], wlrec[0], nfans[0]);
    let pred = neural_network_dot(input, weights);
    println!("version2: {}", pred)
}

fn neural_network_dot(input: Vector3<f32>, weights: Vector3<f32>) -> f32 {
    let pred = input.dot(&weights);
    return pred;
}

fn ele_mul(input: f32, weights: Vector3<f32>) -> Vector3<f32> {
    let pred = input * weights;
    pred
}

fn neural_network_ele_mul(input: f32, weights: Vector3<f32>) -> Vector3<f32> {
    ele_mul(input, weights)
}

fn version4() {
    //let input: f32 = 0.64;
    //let weights = Vector3::new(0.3, 0.2, 0.9);
    let pred = neural_network_ele_mul(0.65, Vector3::new(0.3, 0.2, 0.9));
    println!("{}", pred)
}

fn version4_1() {
    //let weights = vec![0.3, 0.2, 0.9];
    //let pred = neural_network_ele_mul(0.64, Vector3::from_row_slice(&weights));
    let pred = neural_network_ele_mul(0.65, Vector3::from_row_slice(&vec![0.3, 0.2, 0.9]));
    println!("{}", pred)
}
