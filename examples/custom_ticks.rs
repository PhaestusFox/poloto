fn main() {
    // hourly trend over one day.
    let trend = vec![
        0, 0, 0, 0, 0, 3, 5, 5, 10, 20, 50, 60, 70, 50, 40, 34, 34, 20, 10, 20, 10, 4, 2, 0,
    ];

    let it = (0..).zip(trend.iter().copied());

    let plots = poloto::plots!(
        poloto::build::cloned_plot(it).histogram(""),
        poloto::build::markers([24], [])
    );

    let ticks = poloto::ticks::TickBuilder::new((0..).step_by(6))
        .with_ticks(|w, v| write!(w, "{} hr", v))
        .build();

    poloto::data(plots)
        .with_xticks(ticks)
        .build_and_label2(("title", "x", "y"))
        .append_to(poloto::simple_light())
        .render_stdout();
}
