const BASE_RES: u32 = 10;
const ORDER: u32 = 2;

struct Points {
    xs: Vec<f64>,
    ys: Vec<f64>,
}

// given two sets of (x,y) points and a new x val,
// return the linear interpolated y value
// y = mx+b   == rise / run
// rise == (y1 - y0)
// run == (x1 - x0)
// we use an offset y0 + (x - x0) since we trying to find value as this x point
fn interp(xs: Vec<f64>, ys: Vec<f64>, x: f64, order: u32) -> f64 {
    let mut y: f64 = 0.0;
    let mut tmp: f64;
    for i in 0..order as usize {
        tmp = ys[i];
        for j in 0..order as usize {
            if i == j {
                continue;
            }
            tmp *= (x - xs[j]) / (xs[i] - xs[j]);
        }
        y += tmp;
    }
    y
}

/*
    ```ltx
    f(x) = \sum_{i=0}^n y_i \prod_{j\ne i}\frac{x-x_j}{x_i - x_j}
    ```
*/
fn interpolate(xs: Vec<f64>, ys: Vec<f64>, n: u32, order: u32) -> Points {
    let scap = 10 * n as usize;
    let mut points = Points {
        xs: Vec::with_capacity(scap),
        ys: Vec::with_capacity(scap),
    };
    for i in 0..(n - 1) as usize {
        for j in 0..10 as usize {
            //println!("i+1: {}", i + i)
            let delta = xs[i + 1] - xs[i];
            let idx = 10 * i + j;
            points.xs.insert(idx, xs[i] + 0.1 * (j as f64) * delta);
            if i < ((n - order) as usize) {
                points.ys.insert(
                    idx,
                    interp(
                        xs[i..(xs.len())].to_vec(),
                        ys[i..(ys.len())].to_vec(),
                        points.xs[idx],
                        order,
                    ),
                );
            } else {
                let nord = (n - order) as usize;
                points.ys.insert(
                    idx,
                    interp(
                        xs[nord..(xs.len())].to_vec(),
                        ys[nord..(ys.len())].to_vec(),
                        points.xs[idx],
                        order,
                    ),
                );
            }
        }
    }
    points
}

/*
f(x)=∑i=0nyi∏j≠ix−xjxi−xj
*/

fn main() {
    let mut xs: Vec<f64> = Vec::with_capacity(BASE_RES as usize);
    let mut ys: Vec<f64> = Vec::with_capacity(BASE_RES as usize);
    for i in 0..BASE_RES {
        xs.push(4.0 * std::f64::consts::PI * (i as f64) / (BASE_RES as f64));
        ys.push(f64::sin(xs[i as usize]));
    }

    let res = interpolate(xs, ys, BASE_RES, ORDER + 1);

    for i in 0..(10 * (BASE_RES - 1)) as usize {
        print!("{:e} {:e}\n", res.xs[i], res.ys[i])
    }
}
