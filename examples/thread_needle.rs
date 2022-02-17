///
/// Example where we pass a uncopiable/unclonable object to each formatting function.
///
use poloto::plotnum::PlotFmt;
use std::fmt;

struct Dummy;
impl fmt::Display for Dummy {
    fn fmt(&self, a: &mut fmt::Formatter) -> fmt::Result {
        write!(a, "##")
    }
}
struct Foo {
    dummy: Dummy,
}

impl PlotFmt for Foo {
    type X = i128;
    type Y = i128;

    fn write_title(&mut self, writer: &mut dyn fmt::Write) -> fmt::Result {
        write!(writer, "hello {}", self.dummy)
    }
    fn write_xname(&mut self, writer: &mut dyn fmt::Write) -> fmt::Result {
        write!(writer, "hello {}", self.dummy)
    }
    fn write_yname(&mut self, writer: &mut dyn fmt::Write) -> fmt::Result {
        write!(writer, "hello {}", self.dummy)
    }
    fn write_xwher(&mut self, writer: &mut dyn fmt::Write) -> fmt::Result {
        write!(writer, "hello {}", self.dummy)
    }
    fn write_ywher(&mut self, writer: &mut dyn fmt::Write) -> fmt::Result {
        write!(writer, "hello {}", self.dummy)
    }
    fn write_xtick(&mut self, writer: &mut dyn fmt::Write, val: &Self::X) -> fmt::Result {
        write!(writer, "{}{}", val, self.dummy)
    }
    fn write_ytick(&mut self, writer: &mut dyn fmt::Write, val: &Self::Y) -> fmt::Result {
        write!(writer, "{}{}", val, self.dummy)
    }
}

// PIPE me to a file!
fn main() {
    //Source https://en.wikipedia.org/wiki/Wikipedia:Size_of_Wikipedia
    let data = [
        (2010, 3144000),
        (2011, 3518000),
        (2012, 3835000),
        (2013, 4133000),
        (2014, 4413000),
        (2015, 4682000),
        (2016, 5045000),
        (2017, 5321200),
        (2018, 5541900),
        (2019, 5773600),
        (2020, 5989400),
        (2021, 6219700),
        (2022, 0), //To complete our histogram, we manually specify when 2021 ends.
    ];

    let data = poloto::data().histogram("foo", data).ymarker(0).build();

    let mut plotter = data.inner.plot_with_plotfmt(
        poloto::steps(data.boundx, (2010..).step_by(2)).ticks,
        poloto::ticks_from_default(data.boundy).ticks,
        Foo { dummy: Dummy },
    );

    println!(
        "{}<style>{}{}</style>{}{}",
        poloto::simple_theme::SVG_HEADER,
        poloto::simple_theme::STYLE_CONFIG_DARK_DEFAULT,
        ".poloto_line{stroke-dasharray:2;stroke-width:1;}",
        poloto::disp_mut(|w| plotter.render(w)),
        poloto::simple_theme::SVG_END
    )
}
