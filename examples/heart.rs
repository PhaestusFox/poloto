// PIPE me to a file!
use poloto::prelude::*;
fn main() {
    // https://mathworld.wolfram.com/HeartCurve.html
    let heart = |t: f64| {
        [
            16.0 * t.sin().powi(3),
            13.0 * t.cos() - 5.0 * (2.0 * t).cos() - 2.0 * (3.0 * t).cos() - (4.0 * t).cos(),
        ]
    };

    let range = (0..100).map(|x| x as f64 / 100.0).map(|x| x * 6.0 - 3.0);

    let mut plotter = poloto::Plotter::new(
        "Heart Graph",
        f64::ctx("x").marker(-20.).marker(20.),
        f64::ctx("y").marker(20.).marker(-20.),
    );

    plotter.line_fill_raw("heart", range.map(heart));
    plotter.preserve_aspect();

    println!("{}", poloto::disp(|a| plotter.simple_theme_dark(a)));
}
