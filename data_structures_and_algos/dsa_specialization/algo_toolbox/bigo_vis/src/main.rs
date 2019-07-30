use plotlib::line::Style;
use plotlib::page::Page;
use plotlib::scatter::Scatter;
use plotlib::style::Line;
// use plotlib::style::Marker;
use plotlib::view::ContinuousView;

fn main() {
    let mut x: Vec<f64> = Vec::with_capacity(100);
    for i in 0..100 {
        x.push(f64::from(i));
    }
    println!("x: {:?}", x);
    let y: Vec<f64> = x.iter().map(|i| i.powf(2.0)).collect();
    println!("y: {:?}", y);
    let mut data: Vec<(f64, f64)> = Vec::with_capacity(100);
    for (i, j) in x.iter().zip(&y) {
        data.push((*i, *j));
    }
    println!("data:   {:?}", data);

    let line = plotlib::line::Line::new(&data).style(Style::new().colour("#35C788"));
    let view = ContinuousView::new().add(&line);
    Page::single(&view).save("line.svg").expect("saving svg");

    let line = plotlib::line::Line::new(&data).style(Style::new().colour("#FFFFFF").width(2.0));
    let view = ContinuousView::new().add(&line).x_label("n").y_label("sqr");
    println!(
        "{}",
        Page::single(&view).dimensions(130, 30).to_text().unwrap()
    );

    let s1 = Scatter::from_slice(&data);
    let v = ContinuousView::new()
        .add(&s1)
        .x_label("n")
        .y_label("square");

    println!("{}", Page::single(&v).dimensions(80, 30).to_text().unwrap());
}

/*
use plotlib::page::Page;
use plotlib::scatter::Scatter;
use plotlib::scatter::Style;
// use plotlib::style::Marker;
use plotlib::view::ContinuousView;

fn main() {
    let data = [
        (-3.0, 2.3),
        (-1.6, 5.3),
        (0.3, 0.7),
        (4.3, -1.4),
        (6.4, 4.3),
        (8.5, 3.7),
    ];
    let s1 = Scatter::from_slice(&data);
    let s2 = Scatter::from_slice(&[(-1.4, 2.5), (7.2, -0.3)]).style(&Style::new());

    let v = ContinuousView::new()
        .add(&s1)
        .add(&s2)
        .x_range(-5., 10.)
        .y_range(-2., 6.)
        .x_label("varying variable")
        .y_label("Y");

    println!("{}", Page::single(&v).dimensions(80, 30).to_text().unwrap());
}
*/

/*
fn main() {
    let n_range = std::ops::Range { start: 1, end: 100 };
    let n = 1..100;
    println!("'n_range': {:?}", n_range);
    looper(n_range);
    println!("'(1..100): {:?}'", n);
    looper(n);

    let n_v = (1..100).collect();
    println!("'(1..100).collect()': {:?}", n_v);
    looperv(n_v);
}

fn looper(n: std::ops::Range<i32>) {
    for i in n {
        println!("{}", i);
    }
}
fn looperv(n: Vec<i32>) {
    for i in n {
        println!("{}", i);
    }
}
*/
