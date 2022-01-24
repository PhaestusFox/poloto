use poloto::prelude::*;
// PIPE me to a file!
fn main() {
    let x = (0..500).map(|x| (x as f64 / 500.0) * 10.0);

    let mut plotter = poloto::plot(
        "Some Trigonometry Plots 🥳",
        formatm!("This is the {} label", 'x'),
        "This is the y label",
        poloto::ctx::<f64>().with_tick_fmt(|v,w,b,s|write!(w,"{} f",v)),
        poloto::ctx::<f64>()
    );

    // Using poloto::Croppable, we can filter out plots and still have discontinuity.
    plotter.line_fill(
        "tan(x)",
        x.clone()
            .map(|x| [x, x.tan()])
            .crop_above(10.0)
            .crop_below(0.0)
            .crop_left(2.0),
    );

    println!("{}", poloto::disp(|a| plotter.simple_theme(a)));
}
