//!
//! Tools to create tick distributions.
//!
use super::*;

///
/// Tick relevant information of [`Data`]
///
#[derive(Debug, Clone, Copy)]
pub struct DataBound<X> {
    pub min: X,
    pub max: X,
}

///
/// Tick relevant information of [`RenderOptions`]
///
#[derive(Debug, Clone)]
pub struct RenderOptionsBound {
    pub ideal_num_steps: u32,
    pub ideal_dash_size: f64,
    pub max: f64,
    pub axis: Axis,
}

pub fn from_iter_fmt<
    X: PlotNum + Display,
    I: IntoIterator<Item = X>,
    F: FnMut(&mut dyn std::fmt::Write, &X) -> fmt::Result,
    L: FnMut(&mut dyn std::fmt::Write, IndexRequester) -> fmt::Result,
>(
    it: I,
    tick_fmt: F,
    where_fmt: L,
) -> TickGen<I, impl TickFmt<X>> {
    let fmt = DefaultTickFmt.with_ticks(tick_fmt).with_where(where_fmt);
    let k = from_iter(it);
    TickGen {
        it: k.it,
        fmt,
        res: k.res,
    }
}

///
/// Create a [`TickFormat`] from a step iterator.
///
///
pub fn from_iter<X: PlotNum + Display, I: IntoIterator<Item = X>>(
    ticks: I,
) -> TickGen<I, DefaultTickFmt> {
    TickGen {
        it: ticks,
        fmt: DefaultTickFmt,
        res: TickRes { dash_size: None },
    }
}

pub struct DefaultTickFmt;

impl<N: Display> TickFmt<N> for DefaultTickFmt {
    fn write_tick(&mut self, a: &mut dyn std::fmt::Write, val: &N) -> std::fmt::Result {
        write!(a, "{}", val)
    }
    fn write_where(
        &mut self,
        _: &mut dyn std::fmt::Write,
        _req: IndexRequester,
    ) -> std::fmt::Result {
        Ok(())
    }
}

// ///
// /// Used by [`ticks::from_iter`]
// ///
// pub struct TickIterFmt<I: Iterator> {
//     ticks: I,
// }
// impl<I: Iterator> TickFormat<I::Item> for TickIterFmt<I>
// where
//     I::Item: PlotNum + Display,
// {
//     type It = I;
//     type Fmt = DefaultTickFmt;
//     fn generate(
//         self,
//         _: &ticks::DataBound<I::Item>,
//         _: &RenderOptionsBound,
//     ) -> TickGen<Self::It, Self::Fmt> {
//         TickGen {
//             it: self.ticks,
//             fmt: DefaultTickFmt,
//             res: TickRes { dash_size: None },
//         }
//     }
// }

#[derive(Debug, Copy, Clone)]
pub enum Axis {
    X,
    Y,
}

///
/// Useful for numbering footnotes. If one axis uses the number one as a footnote,
/// The second access should use the number two as a footnote.
///
pub struct IndexRequester<'a> {
    counter: &'a mut usize,
}
impl<'a> IndexRequester<'a> {
    #[inline(always)]
    pub fn new(counter: &'a mut usize) -> Self {
        IndexRequester { counter }
    }
    #[inline(always)]
    pub fn request(&mut self) -> usize {
        let val = *self.counter;
        *self.counter += 1;
        val
    }
}

pub trait TickFmt<N> {
    fn write_tick(&mut self, a: &mut dyn std::fmt::Write, val: &N) -> std::fmt::Result;
    fn write_where(
        &mut self,
        _: &mut dyn std::fmt::Write,
        _req: IndexRequester,
    ) -> std::fmt::Result {
        Ok(())
    }
    fn with_ticks<F>(self, func: F) -> WithTicky<Self, F>
    where
        F: FnMut(&mut dyn std::fmt::Write, &N) -> fmt::Result,
        Self: Sized,
    {
        WithTicky { ticks: self, func }
    }
    fn with_where<F>(self, func: F) -> WithWhere<Self, F>
    where
        F: FnMut(&mut dyn std::fmt::Write, IndexRequester) -> fmt::Result,
        Self: Sized,
    {
        WithWhere { ticks: self, func }
    }
}

pub struct WithWhere<D, F> {
    ticks: D,
    func: F,
}

impl<N, D: TickFmt<N>, F> TickFmt<N> for WithWhere<D, F>
where
    F: FnMut(&mut dyn std::fmt::Write, IndexRequester) -> fmt::Result,
{
    fn write_tick(&mut self, a: &mut dyn std::fmt::Write, val: &N) -> std::fmt::Result {
        self.ticks.write_tick(a, val)
    }
    fn write_where(
        &mut self,
        w: &mut dyn std::fmt::Write,
        req: IndexRequester,
    ) -> std::fmt::Result {
        (self.func)(w, req)
    }
}

pub struct WithTicky<D, F> {
    ticks: D,
    func: F,
}
impl<N, D: TickFmt<N>, F> TickFmt<N> for WithTicky<D, F>
where
    F: FnMut(&mut dyn std::fmt::Write, &N) -> fmt::Result,
{
    fn write_tick(&mut self, a: &mut dyn std::fmt::Write, val: &N) -> std::fmt::Result {
        (self.func)(a, val)
    }
    fn write_where(
        &mut self,
        w: &mut dyn std::fmt::Write,
        req: IndexRequester,
    ) -> std::fmt::Result {
        self.ticks.write_where(w, req)
    }
}

pub struct TickRes {
    pub dash_size: Option<f64>,
}

///
/// Formatter for a tick.
///
pub trait TickFormat<N: PlotNum> {
    type It: IntoIterator<Item = N>;
    type Fmt: TickFmt<N>;
    fn generate(
        self,
        data: &ticks::DataBound<N>,
        canvas: &RenderOptionsBound,
    ) -> TickGen<Self::It, Self::Fmt>;

    fn with_fmt<F: TickFmt<N>>(self, fmt: F) -> WithFmt<Self, F>
    where
        Self: Sized,
    {
        WithFmt { ticks: self, fmt }
    }
}

pub struct TickGen<I, F> {
    pub it: I,
    pub fmt: F,
    pub res: TickRes,
}
// impl<I:IntoIterator,F> TickGen<I,F>{
//     pub fn with_ticks<FF:FnMut(&mut dyn fmt::Write,&I::Item)>(self,func:FF)->TickGen<I>{

//     }

//     pub fn with_fmt(self)
// }

impl<X: PlotNum, I: IntoIterator<Item = X>, Fmt: TickFmt<X>> TickFormat<X> for TickGen<I, Fmt> {
    type It = I;
    type Fmt = Fmt;
    fn generate(
        self,
        _: &ticks::DataBound<X>,
        _: &RenderOptionsBound,
    ) -> TickGen<Self::It, Self::Fmt> {
        self
    }
}

pub struct WithFmt<T, F> {
    ticks: T,
    fmt: F,
}
impl<N: PlotNum, T: TickFormat<N>, F: TickFmt<N>> TickFormat<N> for WithFmt<T, F> {
    type It = T::It;
    type Fmt = F;
    fn generate(
        self,
        data: &ticks::DataBound<N>,
        canvas: &RenderOptionsBound,
    ) -> TickGen<Self::It, Self::Fmt> {
        let TickGen { it, res, .. } = self.ticks.generate(data, canvas);
        TickGen {
            it,
            fmt: self.fmt,
            res,
        }
    }
}
