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

    let mut data = poloto::data();
    data.line_fill_raw("heart", range.map(heart));
    data.xmarker(-20.0);
    data.xmarker(20.0);
    data.ymarker(-20.0);
    data.ymarker(20.0);

    let plotter = data.plot("Heart Graph", "x", "y");

    plotter.preserve_aspect();

    println!("{}", poloto::disp(|a| plotter.simple_theme_dark(a)));
}
