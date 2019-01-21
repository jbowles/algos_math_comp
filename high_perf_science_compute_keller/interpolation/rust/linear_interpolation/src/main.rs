const BASE_RES: u32 = 10;

//#[derive(Debug)]
struct Result {
    xs: Vec<f64>,
    ys: Vec<f64>,
}

/*
// slope of a line; ratio of a vertical line
fn slope(y0: f64, y1: f64, x0: f64, x1: f64) -> f64 {
    (y1 - y0) / (x1 - x0)
}
// offset to find value at x
fn offset(y0: f64, x0: f64, x: f64) -> f64 {
    y0 + (x - x0)
}
*/

// interp takes two sets of (x,y) points and a new value x.
// returns the interpolated value for y.
fn interp(y0: f64, y1: f64, x0: f64, x1: f64, x: f64) -> f64 {
    //offset(y0, x0, x) * slope(y0, y1, x0, x1)
    y0 + (x - x0) * (y1 - y0) / (x1 - x0)
}

/* interpolate for result, with n number of initial values and x,y coordinates to fit.

allocate result x,y slices 10 * number of intial size -1
loop initial value size
loop interploated value size
set index to 10 * initial value + interpolated value
set delta to initial value X+1 - X (e.g, 1)
set result X initial value X + 0.1 * interoplated value * delta (0 + 0.1 + 0 * 1 == 1.1)
set result Y interp()
*/
fn interpolate(x_fit: Vec<f64>, y_fit: Vec<f64>, n: u32) -> Result {
    let scap: usize = (10 * (n - 1)) as usize;
    let mut res = Result {
        xs: Vec::with_capacity(scap),
        ys: Vec::with_capacity(scap),
    };
    //println!("res.xs: {:?}, res.ys: {:?}", res.xs.len(), res.ys.len());
    for i in 0..(n - 1) {
        for j in 0..10 {
            //alias index usize
            let i_plus1: usize = (i + 1) as usize;
            let idx: usize = ((10 * i) + j) as usize;
            let i_as: usize = i as usize;

            let delta = x_fit[i_plus1] - x_fit[i as usize];
            res.xs.insert(idx, x_fit[i_as] + 0.1 * (j as f64) * delta);
            res.ys.insert(
                idx,
                interp(
                    y_fit[i_as],
                    y_fit[i_plus1],
                    x_fit[i_as],
                    x_fit[i_plus1],
                    res.xs[idx],
                ),
            );
        }
    }
    return res;
}

/*
    ./target/debug/linear_interpolation > linplot_out
*/
fn main() {
    let mut xs: Vec<f64> = Vec::with_capacity(BASE_RES as usize);
    let mut ys: Vec<f64> = Vec::with_capacity(BASE_RES as usize);
    //println!("{:?}", xs);
    for i in 0..BASE_RES {
        xs.push(4.0 * std::f64::consts::PI * (i as f64) / (BASE_RES as f64));
        ys.push(f64::sin(xs[i as usize]));
    }

    let res = interpolate(xs, ys, BASE_RES);

    for i in 0..(10 * (BASE_RES - 1)) {
        print!("{:e} {:e}\n", res.xs[i as usize], res.ys[i as usize])
    }
}
