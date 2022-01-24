use poloto::num::timestamp::UnixTime;

// PIPE me to a file!
fn main() {
    //Source https://en.wikipedia.org/wiki/Wikipedia:Size_of_Wikipedia
    let data = [
        (UnixTime::from_year(2010), 3144000),
        (UnixTime::from_year(2011), 3518000),
        (UnixTime::from_year(2012), 3835000),
        (UnixTime::from_year(2013), 4133000),
        (UnixTime::from_year(2014), 4413000),
        (UnixTime::from_year(2015), 4682000),
        (UnixTime::from_year(2016), 5045000),
        (UnixTime::from_year(2017), 5321200),
        (UnixTime::from_year(2018), 5541900),
        (UnixTime::from_year(2019), 5773600),
        (UnixTime::from_year(2020), 5989400),
        (UnixTime::from_year(2021), 6219700),
        (UnixTime::from_year(2022), 0), //To complete our histogram, we manually specify when 2021 ends.
    ];

    use poloto::polotofmt;
    let title = polotofmt::name_ext(|w, x: polotofmt::DataSingle<UnixTime>, _| {
        let srt = x.bound[0].dynamic_format(x.step);
        let end = x.bound[1].dynamic_format(x.step);
        write!(w, "Entries from {} to {} in {}", srt, end, x.step)
    });

    let xname = poloto::AxisBuilder::new(polotofmt::name_single_ext(
        |w, p: polotofmt::DataSingle<UnixTime>| {
            let srt = p.bound[0].dynamic_format(p.step);
            let end = p.bound[1].dynamic_format(p.step);
            write!(w, "Entries from {} to {} in {}", srt, end, p.step)
        },
    ));

    let mut plotter = poloto::Plotter::new(title, xname, poloto::AxisBuilder::new("entires"));

    plotter.line("foo", &data);

    plotter.yaxis().marker(0).no_dash();

    println!(
        "{}<style>{}{}</style>{}{}",
        poloto::simple_theme::SVG_HEADER,
        poloto::simple_theme::STYLE_CONFIG_DARK_DEFAULT,
        ".poloto_line{stroke-dasharray:2;stroke-width:1;}",
        poloto::disp(|w| plotter.render(w)),
        poloto::simple_theme::SVG_END
    )
}
