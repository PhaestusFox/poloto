use hypermelon::format_move;

fn main() {
    // hourly trend over one day.
    let trend = vec![
        0, 0, 0, 0, 0, 3, 5, 5, 10, 20, 50, 60, 70, 50, 40, 34, 34, 20, 10, 20, 10, 4, 2, 0,
    ];

    let it = (0..).zip(trend.iter().copied());

    let plots = poloto::plots!(
        poloto::build::plot("").histogram().cloned(it),
        poloto::build::markers([24], [])
    );

    let data = poloto::data(plots);

    let data = data.map_xticks(|orig| {
        poloto::ticks::from_closure(|a, b, c| {
            let orig = poloto::ticks::gen_ticks(orig, a, b, c);
            poloto::ticks::from_iter(orig.iter)
                .with_tick_fmt(|&v| format_move!("{} hr", v))
        })
    });

    data.build_and_label(("title", "x", "y"))
        .append_to(poloto::header().light_theme())
        .render_stdout();
}
