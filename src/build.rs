use super::*;

impl<'a, X: PlotNum + 'a, Y: PlotNum + 'a> Default for Data<'a, X, Y> {
    fn default() -> Self {
        Data {
            plots: vec![],
            xmarkers: vec![],
            ymarkers: vec![],
        }
    }
}

impl<'a, X: PlotNum + 'a, Y: PlotNum + 'a> Data<'a, X, Y> {
    pub fn xmarker(&mut self, a: X) -> &mut Self {
        self.xmarkers.push(a);
        self
    }

    pub fn ymarker(&mut self, a: Y) -> &mut Self {
        self.ymarkers.push(a);
        self
    }

    ///
    /// Write some text in the legend. This doesnt increment the plot number.
    ///
    /// ```
    /// let mut plotter = poloto::data::<f64,f64>();
    /// plotter.text("This is a note");
    /// ```
    pub fn text(&mut self, name: impl Display + 'a) -> &mut Self {
        self.plots.push(Box::new(PlotStruct::new(
            std::iter::empty(),
            name,
            PlotMetaType::Text,
        )));
        self
    }

    /// Create a line from plots using a SVG path element.
    /// The path element belongs to the `.poloto[N]fill` css class.
    ///
    /// ```
    /// let data = [[1.0,4.0], [2.0,5.0], [3.0,6.0]];
    /// let mut plotter = poloto::data();
    /// plotter.line("", &data);
    /// ```
    pub fn line<I>(&mut self, name: impl Display + 'a, plots: I) -> &mut Self
    where
        I: PlotIter + 'a,
        I::Item1: Plottable<Item = (X, Y)>,
        I::Item2: Plottable<Item = (X, Y)>,
    {
        self.plots.push(Box::new(PlotStruct::new(
            plots.map_plot(|x| x.make_plot(), |x| x.make_plot()),
            name,
            PlotMetaType::Plot(PlotType::Line),
        )));
        self
    }

    /// Create a line from plots that will be filled underneath using a SVG path element.
    /// The path element belongs to the `.poloto[N]fill` css class.
    ///
    /// ```
    /// let data = [[1.0,4.0], [2.0,5.0], [3.0,6.0]];
    /// let mut plotter = poloto::data();
    /// plotter.line_fill("", &data);
    /// ```
    pub fn line_fill<I>(&mut self, name: impl Display + 'a, plots: I) -> &mut Self
    where
        I: PlotIter + 'a,
        I::Item1: Plottable<Item = (X, Y)>,
        I::Item2: Plottable<Item = (X, Y)>,
    {
        self.plots.push(Box::new(PlotStruct::new(
            plots.map_plot(|x| x.make_plot(), |x| x.make_plot()),
            name,
            PlotMetaType::Plot(PlotType::LineFill),
        )));
        self
    }

    /// Create a line from plots that will be filled using a SVG path element.
    /// The first and last points will be connected and then filled in.
    /// The path element belongs to the `.poloto[N]fill` css class.
    ///
    /// ```
    /// let data = [[1.0,4.0], [2.0,5.0], [3.0,6.0]];
    /// let mut plotter = poloto::data();
    /// plotter.line_fill_raw("", &data);
    /// ```
    pub fn line_fill_raw<I>(&mut self, name: impl Display + 'a, plots: I) -> &mut Self
    where
        I: PlotIter + 'a,
        I::Item1: Plottable<Item = (X, Y)>,
        I::Item2: Plottable<Item = (X, Y)>,
    {
        self.plots.push(Box::new(PlotStruct::new(
            plots.map_plot(|x| x.make_plot(), |x| x.make_plot()),
            name,
            PlotMetaType::Plot(PlotType::LineFillRaw),
        )));
        self
    }

    /// Create a scatter plot from plots, using a SVG path with lines with zero length.
    /// Each point can be sized using the stroke width.
    /// The path belongs to the CSS classes `poloto_scatter` and `.poloto[N]stroke` css class
    /// with the latter class overriding the former.
    ///
    /// ```
    /// let data = [[1.0,4.0], [2.0,5.0], [3.0,6.0]];
    /// let mut plotter = poloto::data();
    /// plotter.scatter("", &data);
    /// ```
    pub fn scatter<I>(&mut self, name: impl Display + 'a, plots: I) -> &mut Self
    where
        I: PlotIter + 'a,
        I::Item1: Plottable<Item = (X, Y)>,
        I::Item2: Plottable<Item = (X, Y)>,
    {
        self.plots.push(Box::new(PlotStruct::new(
            plots.map_plot(|x| x.make_plot(), |x| x.make_plot()),
            name,
            PlotMetaType::Plot(PlotType::Scatter),
        )));
        self
    }

    /// Create a histogram from plots using SVG rect elements.
    /// Each bar's left side will line up with a point.
    /// Each rect element belongs to the `.poloto[N]fill` css class.
    ///
    /// ```
    /// let data = [[1.0,4.0], [2.0,5.0], [3.0,6.0]];
    /// let mut plotter = poloto::data();
    /// plotter.histogram("", &data);
    /// ```
    pub fn histogram<I>(&mut self, name: impl Display + 'a, plots: I) -> &mut Self
    where
        I: PlotIter + 'a,
        I::Item1: Plottable<Item = (X, Y)>,
        I::Item2: Plottable<Item = (X, Y)>,
    {
        self.plots.push(Box::new(PlotStruct::new(
            plots.map_plot(|x| x.make_plot(), |x| x.make_plot()),
            name,
            PlotMetaType::Plot(PlotType::Histo),
        )));
        self
    }

    ///
    /// use [`gen_bar`] instead.
    ///
    pub(crate) fn bars<I>(&mut self, name: impl Display + 'a, plots: I) -> &mut Self
    where
        I: PlotIter + 'a,
        I::Item1: Plottable<Item = (X, Y)>,
        I::Item2: Plottable<Item = (X, Y)>,
    {
        self.plots.push(Box::new(PlotStruct::new(
            plots.map_plot(|x| x.make_plot(), |x| x.make_plot()),
            name,
            PlotMetaType::Plot(PlotType::Bars),
        )));
        self
    }

    pub fn move_into(&mut self) -> Self {
        let mut val = Data {
            plots: vec![],
            xmarkers: vec![],
            ymarkers: vec![],
        };

        std::mem::swap(&mut val, self);
        val
    }

    ///
    /// Compute min/max bounds and prepare for next stage in pipeline.
    ///
    /// ```
    /// let data = [[1.0,4.0], [2.0,5.0], [3.0,6.0]];
    /// let mut plotter = poloto::data();
    /// plotter.line("", &data);
    /// plotter.build();
    /// ```
    ///
    pub fn build(&mut self) -> DataResult<'a, X, Y> {
        let mut val = self.move_into();

        let (boundx, boundy) = util::find_bounds(
            val.plots.iter_mut().flat_map(|x| x.iter_first()),
            val.xmarkers.clone(),
            val.ymarkers.clone(),
        );

        let boundx = DataBound {
            min: boundx[0],
            max: boundx[1],
        };
        let boundy = DataBound {
            min: boundy[0],
            max: boundy[1],
        };

        DataResult {
            plots: val.plots,
            boundx,
            boundy,
        }
    }
}

impl<'a, X: PlotNum + 'a, Y: PlotNum + 'a> DataResult<'a, X, Y> {
    pub fn boundx(&self) -> &DataBound<X> {
        &self.boundx
    }
    pub fn boundy(&self) -> &DataBound<Y> {
        &self.boundy
    }

    pub fn default_ticks_x(&self, canvas: &Canvas) -> (TickInfo<X::IntoIter>, X::Fmt)
    where
        X: HasDefaultTicks,
    {
        X::generate(self.boundx(), canvas.boundx())
    }

    pub fn default_ticks_y(&self, canvas: &Canvas) -> (TickInfo<Y::IntoIter>, Y::Fmt)
    where
        Y: HasDefaultTicks,
    {
        Y::generate(self.boundy(), canvas.boundy())
    }

    ///
    /// Automatically create a tick distribution using the default
    /// tick generators tied to a [`PlotNum`].
    ///
    pub fn plot(
        self,
        title: impl Display + 'a,
        xname: impl Display + 'a,
        yname: impl Display + 'a,
    ) -> Plotter<impl Disp + 'a>
    where
        X: HasDefaultTicks,
        Y: HasDefaultTicks,
    {
        let canvas = crate::canvas().build();
        self.plot_with_canvas(canvas, title, xname, yname)
    }

    ///
    /// Automatically create a tick distribution using the default
    /// tick generators tied to a [`PlotNum`].
    ///
    pub fn plot_with_canvas(
        self,
        canvas: Canvas,
        title: impl Display + 'a,
        xname: impl Display + 'a,
        yname: impl Display + 'a,
    ) -> Plotter<impl Disp + 'a>
    where
        X: HasDefaultTicks,
        Y: HasDefaultTicks,
    {
        let (x, xt) = self.default_ticks_x(&canvas);
        let (y, yt) = self.default_ticks_y(&canvas);

        let p = plot_fmt(title, xname, yname, xt, yt);
        self.plot_with_ticks_and_canvas(canvas, x, y, p)
    }

    pub fn plot_with_ticks<XI: 'a, YI: 'a, PF: 'a>(
        self,
        xtick: TickInfo<XI>,
        ytick: TickInfo<YI>,
        plot_fmt: PF,
    ) -> Plotter<impl Disp + 'a>
    where
        XI: IntoIterator<Item = X>,
        YI: IntoIterator<Item = Y>,
        PF: BaseFmt<X = X, Y = Y>,
    {
        let canvas = crate::canvas().build();
        self.plot_with_ticks_and_canvas(canvas, xtick, ytick, plot_fmt)
    }
    ///
    /// Move to final stage in pipeline collecting the title/xname/yname.
    /// Unlike [`DataResult::plot`] User must supply own tick distribution.
    ///
    pub fn plot_with_ticks_and_canvas<XI: 'a, YI: 'a, PF: 'a>(
        self,
        canvas: Canvas,
        xtick: TickInfo<XI>,
        ytick: TickInfo<YI>,
        plot_fmt: PF,
    ) -> Plotter<impl Disp + 'a>
    where
        XI: IntoIterator<Item = X>,
        YI: IntoIterator<Item = Y>,
        PF: BaseFmt<X = X, Y = Y>,
    {
        ///
        /// Wrap tick iterators and a [`PlotFmt`] behind the [`PlotFmtAll`] trait.
        ///
        struct PlotAllStruct<XI: IntoIterator, YI: IntoIterator, PF: BaseFmt> {
            xtick: TickInfo<XI>,
            ytick: TickInfo<YI>,
            fmt: PF,
        }

        impl<XI: IntoIterator, YI: IntoIterator, PF: BaseFmt<X = XI::Item, Y = YI::Item>>
            BaseFmtAndTicks for PlotAllStruct<XI, YI, PF>
        where
            XI::Item: PlotNum,
            YI::Item: PlotNum,
        {
            type X = PF::X;
            type Y = PF::Y;
            type Fmt = PF;
            type XI = XI;
            type YI = YI;

            fn gen(self) -> (Self::Fmt, TickInfo<Self::XI>, TickInfo<Self::YI>) {
                (self.fmt, self.xtick, self.ytick)
            }
        }

        self.plot_with_all(
            canvas,
            PlotAllStruct {
                fmt: plot_fmt,
                xtick,
                ytick,
            },
        )
    }

    ///
    /// Create a plotter directly from a [`BaseFmtAndTicks`]
    ///
    fn plot_with_all<PF: BaseFmtAndTicks<X = X, Y = Y> + 'a>(
        self,
        canvas: Canvas,
        p: PF,
    ) -> Plotter<impl Disp + 'a> {
        struct Foo2<'a, X, Y> {
            plots: Vec<Box<dyn PlotTrait<'a, Item = (X, Y)> + 'a>>,
        }

        struct One<'a, X, Y> {
            one: Box<dyn PlotTrait<'a, Item = (X, Y)> + 'a>,
        }
        impl<'a, X, Y> OnePlotFmt for One<'a, X, Y> {
            type It = Box<dyn Iterator<Item = Self::Item> + 'a>;
            type Item = (X, Y);
            fn plot_type(&mut self) -> PlotMetaType {
                self.one.plot_type()
            }

            fn fmt(&mut self, writer: &mut dyn fmt::Write) -> fmt::Result {
                self.one.write_name(writer)
            }

            fn get_iter(&mut self) -> Box<dyn Iterator<Item = Self::Item> + 'a> {
                self.one.iter_second()
            }
        }

        impl<'a, X: 'a, Y: 'a> AllPlotFmt for Foo2<'a, X, Y> {
            type Item2 = (X, Y);
            type It = Box<dyn Iterator<Item = One<'a, X, Y>> + 'a>;
            type InnerIt = One<'a, X, Y>;
            fn iter(self) -> Self::It {
                Box::new(self.plots.into_iter().map(|one| One { one }))
            }
        }

        struct Combine<A: BaseFmtAndTicks, B: AllPlotFmt> {
            pub a: A,
            pub b: B,
        }

        impl<A: BaseFmtAndTicks, B: AllPlotFmt<Item2 = (A::X, A::Y)>> BaseAndPlotsFmt for Combine<A, B> {
            type X = A::X;
            type Y = A::Y;
            type A = A;
            type B = B;
            fn gen(self) -> (Self::A, Self::B) {
                (self.a, self.b)
            }
        }

        struct InnerPlotter<PF: BaseAndPlotsFmt> {
            all: PF,
            boundx: DataBound<PF::X>,
            boundy: DataBound<PF::Y>,
            canvas: Canvas,
        }

        impl<PF: BaseAndPlotsFmt> Disp for InnerPlotter<PF> {
            fn disp<T: std::fmt::Write>(self, mut writer: T) -> fmt::Result {
                render::render(&mut writer, self.all, self.boundx, self.boundy, self.canvas)
            }
        }

        let pp = InnerPlotter {
            all: Combine {
                a: p,
                b: Foo2 { plots: self.plots },
            },
            boundx: self.boundx,
            boundy: self.boundy,
            canvas,
        };

        let dim = pp.canvas.get_dim();
        Plotter {
            inner: Some(pp),
            dim,
        }
    }
}
