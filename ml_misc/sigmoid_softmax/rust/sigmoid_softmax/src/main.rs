extern crate nalgebra as na;
use na::{
    DMatrix, Dynamic, Matrix, Matrix2x3, Matrix3x4, Matrix4, MatrixArray, MatrixVec, RowVector4,
    Vector2, Vector4, U2, U3, U4,
};
use std::ops::DivAssign;

//./target/debug/sigmoid_softmax
fn main() {
    //println!("sigmoid: {}", sigmoid(10.0));
    //playplay();

    let mut m4 = Matrix4::from_rows(&[
        RowVector4::new(1.0, 2.0, 3.0, 6.0),
        RowVector4::new(2.0, 4.0, 5.0, 6.0),
        RowVector4::new(3.0, 8.0, 7.0, 6.0),
        RowVector4::new(3.0, 8.0, 7.0, 6.0),
    ]);
    println!("m4 = {}", m4);
    softmax_naive1(&mut m4);
    println!("m4 = {}", m4);

    let mut m41 = Matrix4::from_rows(&[
        RowVector4::new(1.0, 2.0, 3.0, 6.0),
        RowVector4::new(2.0, 4.0, 5.0, 6.0),
        RowVector4::new(3.0, 8.0, 7.0, 6.0),
        RowVector4::new(3.0, 8.0, 7.0, 6.0),
    ]);

    softmax_naive_by_row(&mut m41);
    println!("m41 = {}", m41);

    let mut m42 = Matrix4::from_rows(&[
        RowVector4::new(1.0, 2.0, 3.0, 6.0),
        RowVector4::new(2.0, 4.0, 5.0, 6.0),
        RowVector4::new(3.0, 8.0, 7.0, 6.0),
        RowVector4::new(3.0, 8.0, 7.0, 6.0),
    ]);

    softmax_naive_by_column(&mut m42);
    println!("m42 = {}", m42);
}

#[allow(dead_code)]
fn sigmoid(x: f32) -> f32 {
    1.0 / (1.0 + (-x).exp())
}

//type AbstractMatrix = Matrix<f32, Dynamic, Dynamic, MatrixVec<f64, Dynamic, Dynamic>>;
//fn softmax(m: &mut Matrix4<f64>) -> Matrix4<f64> {
fn softmax_naive1(m: &mut Matrix4<f64>) {
    m.apply(|v| v.exp());

    let mut v4 = Vector4::zeros();
    let (r, c) = m.shape();
    for idx in 0..c {
        let cm = m.column_mut(idx);
        let sum = cm.iter().fold(0.0, |sum, val| sum + val);
        v4[idx] = sum;
    }

    for rw in 0..r {
        let mut rv = RowVector4::zeros();
        for (col, val) in v4.iter().enumerate() {
            let ij = m.row(rw).column(col)[0];
            rv[col] = ij / val;
        }
        m.set_row(rw, &rv);
    }
}
fn softmax_naive_by_row(m: &mut Matrix4<f64>) {
    m.apply(|v| v.exp());

    let v4 = RowVector4::<f64>::from_fn(|_, j| m.column(j).iter().sum());
    for rw in 0..m.nrows() {
        m.row_mut(rw).component_div_assign(&v4)
    }
    /*
    let mut v4 = Vector4::zeros();
    let (r, c) = m.shape();
    for idx in 0..c {
        let cm = m.column_mut(idx);
        let sum: f64 = cm.iter().sum();
        v4[idx] = sum;
    }

    for rw in 0..r {
        for (col, val) in v4.iter().enumerate() {
            m[(rw, col)] /= val
        }
    }
    */
}

fn softmax_naive_by_column(m: &mut Matrix4<f64>) {
    m.apply(|v| v.exp());
    let v4 = Vector4::<f64>::from_fn(|i, _| m.column(i).iter().sum());
    for j in 0..m.ncols() {
        m.column_mut(j).div_assign(v4[j])
    }
}

/*
    Matrix<N, R, C, S>
    N: the scalar type, i.e., the type of the matrix components. Typical values are f32 or f64.
    R: a type characterizing the number of rows on this matrix.
    C: a type characterizing the number of columns on this matrix.
    S: the buffer that contains all the matrix components and (if necessary) metadata about its shape at run-time. Of all the type parameters, this is the only element actually instantiated by a Matrix.

*/

// Statically sized and statically allocated 2x3 matrix using 32-bit floats.
type Matrix2x3f32 = Matrix<f32, U2, U3, MatrixArray<f32, U2, U3>>;
type Matrix4x4f32 = Matrix<f32, U4, U4, MatrixArray<f32, U4, U4>>;
// Half-Dynamically sized and allocated matrix with 2 row uinsg 64-bit floats
type Matrix2xxXf64 = Matrix<f64, U2, Dynamic, MatrixVec<f64, U2, Dynamic>>;
//?? let d = Matrix2xxXf64::from_data((11, 12, 21, 22));
//?? let d1 = Matrix2xxXf64::new(11, 12, 21, 22);

#[allow(dead_code)]
fn playplay() {
    let v = Vector2::new(2.0, 3.0);
    //new will be by row comma delimited
    let n = Matrix2x3f32::new(11.666666, 12.4444, 13.333, 21.0, 22.0, 23.88888888);

    let m = Matrix3x4::from_rows(&[
        RowVector4::new(11, 12, 13, 14),
        RowVector4::new(21, 22, 23, 24),
        RowVector4::new(31, 32, 33, 34),
    ]);
    let m4c = Matrix4x4f32::from_rows(&[
        RowVector4::new(1.0, 2.0, 3.0, 6.0),
        RowVector4::new(2.0, 4.0, 5.0, 6.0),
        RowVector4::new(3.0, 8.0, 7.0, 6.0),
        RowVector4::new(3.0, 8.0, 7.0, 6.0),
    ]);
    let m4 = Matrix4::from_columns(&[
        Vector4::new(1.0, 2.0, 3.0, 3.0),
        Vector4::new(2.0, 4.0, 8.0, 8.0),
        Vector4::new(3.0, 5.0, 7.0, 7.0),
        Vector4::new(6.0, 6.0, 6.0, 6.0),
    ]);
    let d1 = Matrix2xxXf64::from_element(2, 1.0);
    /*
    Matrix2x3::from_fn =
        ┌             ┐
        │ 1.1 1.2 1.3 │
        │ 2.1 2.2 2.3 │
        └             ┘
    */
    let m5 = Matrix2x3::from_fn(|r, c| (r + 1) as f64 + (c + 1) as f64 / 10.0);
    let d2 = DMatrix::from_fn(4, 3, |r, c| if r == c { 1.0 } else { 0.0 });
    let d3 = DMatrix::from_fn(10, 5, |r, c| if r == c { 1.0 } else { 0.0 });
    let zs1 = Matrix4x4f32::zeros();

    println!(" Matrix4x4f32::zeros()= {}", zs1);
    println!("Vector3 = {}", v);
    println!("Matrix3x4 = {}", m);
    println!("custom Matrix2x3f32 = {}", n);
    println!("custom Matrix4x4f32 = {}", m4c);
    println!("Matrix4 = {}", m4);
    println!("Matrix2xxXf64 = {}", d1);
    println!("Matrix2x3::from_fn = {}", m5);
    println!("DMatrix::from_fn(4,3, fn()) = {}", d2);
    println!("DMatrix::from_fn(10,5, fn()) = {}", d3);

    println!(
        "len() = {}; shape() = {:?}; strides() = {:?}",
        m4.len(),
        m4.shape(),
        m4.strides()
    );
    println!("nrows() = {}; ncols() = {:?}", m4.nrows(), m4.ncols());

    //let j = m5.component_div(&v);
}

/*
    apply f3.exp() function to each element
        .apply(f)

    in numpy
    np.exp(x) / np.sum(np.exp(x), axis=0)
*/
