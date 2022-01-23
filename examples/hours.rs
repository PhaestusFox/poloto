use poloto::num::timestamp::UnixTime;
use poloto::prelude::*;
// PIPE me to a file!
fn main() {
    let day1 = (2020, 1, 30);
    let day2 = (2020, 1, 31);
    //Source https://en.wikipedia.org/wiki/Wikipedia:Size_of_Wikipedia
    let data = [
        (UnixTime::from_ymd_hms(day1, 12, 59, 59), 3144000),
        (UnixTime::from_ymd_hms(day2, 1, 0, 0), 3518000),
        (UnixTime::from_ymd_hms(day2, 2, 2, 1), 3835000),
        (UnixTime::from_ymd_hms(day2, 3, 59, 59), 2133000),
        (UnixTime::from_ymd_hms(day2, 6, 1, 1), 4133000),
    ];

    let mut s = poloto::plot("Number of Wikipedia Articles", "Year", "Number of Articles")
        .line("", &data)
        .move_into();

    s.yaxis().marker(0);

    println!("{}", poloto::disp(|a| s.simple_theme(a)));
}
