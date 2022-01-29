//! Plot unix timestamps.
//!
//! Does not implement dashes/grid lines because due to leap days, the distance
//! between the dashes can't be constant.
//!
//!
//!
//!  
mod tick_finder;
mod unixtime;

use super::*;
use chrono::prelude::*;
use chrono::DateTime;
pub use unixtime::*;

///
/// Returns a 3 letter string for a month. input must be in the range `[1,12]` or it will panic.
///
pub fn month_str(month: u32) -> &'static str {
    match month {
        1 => "Jan",
        2 => "Feb",
        3 => "Mar",
        4 => "Apr",
        5 => "May",
        6 => "Jun",
        7 => "Jul",
        8 => "Aug",
        9 => "Sep",
        10 => "Oct",
        11 => "Nov",
        12 => "Dec",
        _ => unreachable!(),
    }
}
///
/// Conveys what unit is being used for step sizes.
///
#[derive(Copy, Clone, Debug)]
pub enum TimestampType {
    YR,
    MO,
    DY,
    HR,
    MI,
    SE,
}

impl std::fmt::Display for TimestampType {
    fn fmt(&self, a: &mut std::fmt::Formatter) -> std::fmt::Result {
        use TimestampType::*;
        let val = match &self {
            YR => "Years",
            MO => "Months",
            DY => "Days",
            HR => "Hours",
            MI => "Minutes",
            SE => "Seconds",
        };
        write!(a, "{}", val)
    }
}

///
/// Default [`UnixTime`] context.
///
pub struct UnixTimeContext<T: chrono::TimeZone> {
    timezone: T,
}

impl<T: chrono::TimeZone> UnixTimeContext<T> {
    pub fn new(timezone: &T) -> Self {
        UnixTimeContext {
            timezone: timezone.clone(),
        }
    }
}

impl Default for UnixTimeContext<Utc> {
    fn default() -> Self {
        UnixTimeContext {
            timezone: chrono::Utc,
        }
    }
}

impl<T: chrono::TimeZone> PlotNumContext for UnixTimeContext<T>
where
    T::Offset: Display,
{
    type StepInfo = TimestampType;
    type Num = UnixTime;

    fn scale(&mut self, val: UnixTime, range: [UnixTime; 2], max: f64) -> f64 {
        let [val1, val2] = range;
        let [val1, val2] = [val1.0, val2.0];
        assert!(val1 <= val2);
        let diff = (val2 - val1) as f64;
        let scale = max / diff;
        val.0 as f64 * scale
    }

    fn tick_fmt(
        &mut self,
        writer: &mut dyn fmt::Write,
        val: UnixTime,
        _bound: [UnixTime; 2],
        info: &mut TimestampType,
    ) -> std::fmt::Result {
        write!(writer, "{}", val.dynamic_format(&self.timezone, info))
    }

    fn where_fmt(
        &mut self,
        writer: &mut dyn std::fmt::Write,
        val: UnixTime,
        _bound: [UnixTime; 2],
    ) -> std::fmt::Result {
        write!(writer, "{}", val.datetime(&self.timezone))
    }

    fn compute_ticks(
        &mut self,
        ideal_num_steps: u32,
        range: [UnixTime; 2],
        _info: DashInfo,
    ) -> TickInfo<UnixTime, TimestampType> {
        assert!(range[0] <= range[1]);

        let [start, end] = range;
        let mut t = tick_finder::BestTickFinder::new(end, ideal_num_steps);

        let steps_yr = &[1, 2, 5, 100, 200, 500, 1000, 2000, 5000];
        let steps_mo = &[1, 2, 3, 6];
        let steps_dy = &[1, 2, 4, 5, 7];
        let steps_hr = &[1, 2, 4, 6];
        let steps_mi = &[1, 2, 10, 15, 30];
        let steps_se = &[1, 2, 5, 10, 15, 30];

        t.consider_meta(TimestampType::YR, start.years(&self.timezone), steps_yr);
        t.consider_meta(TimestampType::MO, start.months(&self.timezone), steps_mo);
        t.consider_meta(TimestampType::DY, start.days(&self.timezone), steps_dy);
        t.consider_meta(TimestampType::HR, start.hours(&self.timezone), steps_hr);
        t.consider_meta(TimestampType::MI, start.minutes(&self.timezone), steps_mi);
        t.consider_meta(TimestampType::SE, start.seconds(&self.timezone), steps_se);

        let ret = t.into_best().unwrap();

        let ticks: Vec<_> = ret
            .ticks
            .into_iter()
            .map(|x| Tick {
                position: x,
                value: x,
            })
            .collect();

        assert!(ticks.len() >= 2);

        TickInfo {
            unit_data: ret.unit_data,
            ticks,
            dash_size: None,
            display_relative: None, //Never want to do this for unix time.
        }
    }

    fn unit_range(&mut self, offset: Option<UnixTime>) -> [UnixTime; 2] {
        if let Some(o) = offset {
            [o, UnixTime(o.0 + 1)]
        } else {
            [UnixTime(0), UnixTime(1)]
        }
    }
}

impl HasDefaultContext for UnixTime {
    type DefaultContext = UnixTimeContext<chrono::Utc>;
}

impl PlotNum for UnixTime {}
