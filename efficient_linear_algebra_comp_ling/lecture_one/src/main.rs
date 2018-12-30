fn main() {
    println!("Hello, world!");
    let v1 = Vec3D {
        x: 1.0,
        y: -1.0,
        z: 0.5,
    };
    let v2 = Vec3D {
        x: -0.3,
        y: -2.0,
        z: 1.0,
    };
    let res = v1.dot(&v2);
    println!("{}", res);
    assert_eq!(2.2, res);

    let nv = Vec3D::new(-8.0, 2.0, 5.0);
    println!("Vec3D::new.dot(&v2) -> {}", nv.dot(&v2));
    resizer_and_slice();
    three_or_seven();
}

struct Vec3D {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3D {
    fn dot(&self, v: &Vec3D) -> f32 {
        (self.x * v.x) + (self.y * v.y) + (self.z * v.z)
    }
}

impl Vec3D {
    fn new(x: f32, y: f32, z: f32) -> Vec3D {
        Vec3D { x: x, y: y, z: z }
    }
}

// vec resizer
fn resizer_and_slice() {
    // vectors are resizable arrays
    let mut three_primes = Vec::new();
    three_primes.push(2);
    three_primes.push(3);
    three_primes.push(5);
    println!("three_primes: {:?}", three_primes);

    let four_primes = vec![2, 3, 5, 7];
    //slices are array views, borrowing the backing array
    let first_two: &[usize] = &four_primes[0..2];

    println!("first_two: {:?}", first_two);
    assert_eq!(first_two, &[2, 3]);
}

/*
fn three_or_seven() {
    //initialize     type; capacity
    let div37 = vec![false; 100];
    let mut div37c = vec![false; 100];
    for (n, _) in div37.iter().enumerate() {
        if n % 2 == 0 {
            if n % 3 == 0 {
                div37c[n] = true
            }
        }
    }
    for (i, v) in div37c.iter().enumerate() {
        println!("i:{}, v:{}", i, v)
    }
}
*/

fn three_or_seven() {
    //initialize     type; capacity
    let mut div37 = vec![false; 100];
    for n in 0..div37.len() {
        match n % 2 {
            0 => match (n % 3, n % 7) {
                (0, _) => div37[n] = true,
                (_, 0) => div37[n] = true,
                (_, _) => (),
            },
            _ => (),
        }
    }
    for (i, v) in div37.iter().enumerate() {
        println!("i:{}, v:{}", i, v)
    }
}
