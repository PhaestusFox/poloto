use crate::*;

mod render_helper;
use render_helper::*;

use std::fmt;
struct DrawData {
    width: f64,
    height: f64,
    padding: f64,
    paddingy: f64,
}
struct ScaleData<X: PlotNum, Y: PlotNum> {
    minx: X,
    maxx: X,
    miny: Y,
    maxy: Y,
    scalex: f64,
    scaley: f64,
    preserve_aspect: bool,
    aspect_offset: f64,
}

//Returns error if the user supplied format functions don't work.
//Panics if the element tag writing writes fail
pub fn render<X: PlotNum, Y: PlotNum, T: std::fmt::Write>(
    plotter: &mut Plotter<X, Y>,
    writer: T,
) -> fmt::Result {
    let mut writer = tagger::new(writer);

    let mut plotter = {
        let mut empty = crate::Plotter::new("", "", "");
        core::mem::swap(&mut empty, plotter);
        empty
    };

    let width = crate::WIDTH as f64;
    let height = crate::HEIGHT as f64;
    let padding = 150.0;
    let paddingy = 100.0;

    let ([minx, maxx], [miny, maxy]) = util::find_bounds(
        plotter.plots.iter_mut().flat_map(|x| x.plots.iter_first()),
        plotter.xmarkers.iter().copied(),
        plotter.ymarkers.iter().copied(),
    );

    let preserve_aspect = plotter.preserve_aspect;

    let aspect_offset = if preserve_aspect {
        width / 2.0 - height + paddingy * 2.0
    } else {
        0.0
    };

    //The range over which the data will be scaled to fit
    let scalex2 = if preserve_aspect {
        height - paddingy * 2.0
    } else {
        width - padding * 2.0
    };

    let scaley2 = height - paddingy * 2.0;

    let spacing = padding / 3.0;
    let legendx1 = width - padding / 1.2 + padding / 30.0;

    for (i, mut p) in plotter.plots.drain(..).enumerate() {
        let legendy1 = paddingy - padding / 8.0 + (i as f64) * spacing;

        let name_exists = writer
            .elem("text", |d| {
                d.attr("class", "poloto_text poloto_legend_text")?;
                d.attr("alignment-baseline", "middle")?;
                d.attr("text-anchor", "start")?;
                d.attr("font-size", "large")?;
                d.attr("x", width - padding / 1.2)?;
                d.attr("y", paddingy + (i as f64) * spacing)
            })?
            .build(|d| {
                let mut wc = util::WriteCounter::new(d.writer());
                p.plots.write_name(&mut wc)?;
                Ok(wc.get_counter() != 0)
            })?;

        let aa = minx.scale([minx, maxx], scalex2);
        let bb = miny.scale([miny, maxy], scaley2);

        // Scale all the plots here.
        let it = p.plots.iter_second().map(|(x, y)| {
            [
                aspect_offset + padding + (x.scale([minx, maxx], scalex2) - aa),
                height - paddingy - (y.scale([miny, maxy], scaley2) - bb),
            ]
        });

        let colori = if let Some(nn) = plotter.num_css_classes {
            i % nn
        } else {
            i
        };

        match p.plot_type {
            PlotType::Line => {
                if name_exists {
                    writer.single("line", |d| {
                        d.attr(
                            "class",
                            format_args!(
                                "poloto_line poloto_legend_icon poloto{}stroke poloto{}legend",
                                colori, colori
                            ),
                        )?;
                        d.attr("stroke", "black")?;
                        d.attr("x1", legendx1)?;
                        d.attr("x2", legendx1 + padding / 3.0)?;
                        d.attr("y1", legendy1)?;
                        d.attr("y2", legendy1)
                    })?;
                }

                writer.single("path", |d| {
                    d.attr("class", format_args!("poloto_line poloto{}stroke", colori))?;
                    d.attr("fill", "none")?;
                    d.attr("stroke", "black")?;
                    d.path(|p| line(p, it))
                })?;
            }
            PlotType::Scatter => {
                if name_exists {
                    writer.single("line", |d| {
                        d.attr(
                            "class",
                            format_args!(
                                "poloto_scatter poloto_legend_icon poloto{}stroke poloto{}legend",
                                colori, colori
                            ),
                        )?;
                        d.attr("stroke", "black")?;
                        d.attr("x1", legendx1 + padding / 30.0)?;
                        d.attr("x2", legendx1 + padding / 30.0)?;
                        d.attr("y1", legendy1)?;
                        d.attr("y2", legendy1)
                    })?;
                }

                writer.single("path", |d| {
                    d.attr(
                        "class",
                        format_args!("poloto_scatter poloto{}stroke", colori),
                    )?;
                    d.path(|p| {
                        use tagger::PathCommand::*;
                        for [x, y] in it.filter(|&[x, y]| x.is_finite() && y.is_finite()) {
                            p.put(M(x, y))?;
                            p.put(H_(0))?;
                        }
                        Ok(())
                    })
                })?;
            }
            PlotType::Histo => {
                if name_exists {
                    writer.single("rect", |d| {
                        d.attr(
                            "class",
                            format_args!(
                                "poloto_histo poloto_legend_icon poloto{}fill poloto{}legend",
                                colori, colori
                            ),
                        )?;
                        d.attr("x", legendx1)?;
                        d.attr("y", legendy1 - padding / 30.0)?;
                        d.attr("width", padding / 3.0)?;
                        d.attr("height", padding / 20.0)?;
                        d.attr("rx", padding / 30.0)?;
                        d.attr("ry", padding / 30.0)
                    })?;
                }

                writer
                    .elem("g", |d| {
                        d.attr("class", format_args!("poloto_histo poloto{}fill", colori))
                    })?
                    .build(|writer| {
                        let mut last = None;
                        //TODO dont necesarily filter?
                        for [x, y] in it.filter(|&[x, y]| x.is_finite() && y.is_finite()) {
                            if let Some((lx, ly)) = last {
                                writer.single("rect", |d| {
                                    d.attr("x", lx)?;
                                    d.attr("y", ly)?;
                                    d.attr(
                                        "width",
                                        (padding * 0.02).max((x - lx) - (padding * 0.02)),
                                    )?;
                                    d.attr("height", height - paddingy - ly)
                                })?;
                            }
                            last = Some((x, y))
                        }
                        Ok(())
                    })?;
            }
            PlotType::LineFill => {
                if name_exists {
                    writer.single("rect", |d| {
                        d.attr(
                            "class",
                            format_args!(
                                "poloto_linefill poloto_legend_icon poloto{}fill poloto{}legend",
                                colori, colori
                            ),
                        )?;
                        d.attr("x", legendx1)?;
                        d.attr("y", legendy1 - padding / 30.0)?;
                        d.attr("width", padding / 3.0)?;
                        d.attr("height", padding / 20.0)?;
                        d.attr("rx", padding / 30.0)?;
                        d.attr("ry", padding / 30.0)
                    })?;
                }

                writer.single("path", |d| {
                    d.attr(
                        "class",
                        format_args!("poloto_linefill poloto{}fill", colori),
                    )?;
                    d.path(|path| line_fill(path, it, height - paddingy,true))
                })?;
            }
            PlotType::LineFillRaw => {
                if name_exists {
                    writer.single("rect", |d| {
                        d.attr(
                            "class",
                            format_args!(
                                "poloto_linefillraw poloto_legend_icon poloto{}fill poloto{}legend",
                                colori, colori
                            ),
                        )?;
                        d.attr("x", legendx1)?;
                        d.attr("y", legendy1 - padding / 30.0)?;
                        d.attr("width", padding / 3.0)?;
                        d.attr("height", padding / 20.0)?;
                        d.attr("rx", padding / 30.0)?;
                        d.attr("ry", padding / 30.0)
                    })?;
                }

                writer.single("path", |d| {
                    d.attr(
                        "class",
                        format_args!("poloto_linefillraw poloto{}fill", colori),
                    )?;
                    d.path(|path| line_fill(path, it, height - paddingy,false))
                })?;
            }
        }
    }

    draw_base(
        &mut plotter,
        &mut writer,
        DrawData {
            width: crate::WIDTH as f64,
            height: crate::HEIGHT as f64,
            padding: 150.0,
            paddingy: 100.0,
        },
        ScaleData {
            minx,
            maxx,
            miny,
            maxy,
            scalex: scalex2,
            scaley: scaley2,
            preserve_aspect,
            aspect_offset,
        },
    )?;

    Ok(())
}
