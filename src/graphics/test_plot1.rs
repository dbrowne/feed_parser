/*
 *
 *  *
 *  *
 *  *
 *  *
 *  * MIT License
 *  * Copyright (c) 2023. Dwight J. Browne
 *  * dwight[-dot-]browne[-at-]dwightjbrowne[-dot-]com
 *  *
 *  *
 *  * Permission is hereby granted, free of charge, to any person obtaining a copy
 *  * of this software and associated documentation files (the "Software"), to deal
 *  * in the Software without restriction, including without limitation the rights
 *  * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 *  * copies of the Software, and to permit persons to whom the Software is
 *  * furnished to do so, subject to the following conditions:
 *  *
 *  * The above copyright notice and this permission notice shall be included in all
 *  * copies or substantial portions of the Software.
 *  *
 *  * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 *  * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 *  * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 *  * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 *  * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 *  * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 *  * SOFTWARE.
 *
 */

use std::error::Error;
use std::fmt;
use ndarray::Array;
use plotly::{
    color::{NamedColor, Rgb, Rgba},
    common::{
        ColorScale, ColorScalePalette, DashType, Fill, Font, Line, LineShape, Marker, Mode,
        Orientation, Title,AxisSide
    },
    layout::{Axis, BarMode, Layout, Legend, TicksDirection, TraceOrder,RangeSlider,RangeSelector,
             SelectorButton,SelectorStep,StepMode},
    sankey::{Line as SankeyLine, Link, Node},
    Bar, Plot, Sankey, Scatter, ScatterPolar,
};

use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;


pub fn test_plot_001(time_series: Vec<(String, f32, i32)>) -> Result<(), Box<dyn Error>> {
    let mut x: Vec<String> = Vec::new();
    let mut y: Vec<f32> = Vec::new();
    let mut z: Vec<i32> = Vec::new();
    for (a, b, c) in time_series {
        x.push(a);
        y.push(b);
        z.push(c);
    }
    let trace1 = Scatter::new(x.clone(), y).name("price");
    let trace2 = Scatter::new(x.clone(), z).name("volume");
    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.add_trace(trace2);
    plot.show();
    Ok(())
}


pub fn test_plot_002(time_series: Vec<(String, f32, i32)>, min_max: (Decimal, Decimal, i32, i32)) -> Result<(), Box<dyn Error>> {
    let mut time_line: Vec<String> = Vec::new();
    let mut price_line: Vec<f32> = Vec::new();
    let mut volume_line: Vec<i32> = Vec::new();
    for (a, b, c) in time_series {
        time_line.push(a);
        price_line.push(b);
        volume_line.push(c);
    }
    let trace1 = Scatter::new(time_line.clone(), price_line).name("price");
    let trace2 = Scatter::new(time_line.clone(), volume_line).name("volume").y_axis("y2");
    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.add_trace(trace2);

    let min_price = min_max.0.to_f32().unwrap();
    let max_price = min_max.1.to_f32().unwrap();

    let layout = Layout::new()
         .height(2200)
        .width(4200)
        // .paper_background_color(Rgba::new(20, 11, 5, 0.2))
        // .plot_background_color(Rgba::new(20, 11, 5, 0.25))
        .x_axis(
        Axis::new()
            .grid_color(Rgba::new(255, 255, 255, 1.0))

            .range_slider(RangeSlider::new().visible(true))

    ).title(Title::new("TSLA Price Jan 3 2023 "))
        .y_axis(Axis::new().title("price".into())
            .grid_color(Rgba::new(255, 255, 255, 0.25))
            .dtick(0.125)
            .side(AxisSide::Left)
            )
        .y_axis2(Axis::new().title("volume".into())
            .grid_color(Rgba::new(255, 0, 0, 0.25))
            .overlaying("y")
            .dtick(50.0)
            .side(AxisSide::Right)

            );
    plot.set_layout(layout);

    plot.show();
    Ok(())
}

pub fn test_plot_003(ticker:&str, time_series: Vec<(String, f32, i32)>, min_max: (Decimal, Decimal, i32, i32)) -> Result<(), Box<dyn Error>> {
    let mut time_line: Vec<String> = Vec::new();
    let mut price_line: Vec<f32> = Vec::new();
    let mut volume_line: Vec<i32> = Vec::new();
    for (a, b, c) in time_series {
        time_line.push(a);
        price_line.push(b);
        volume_line.push(c);
    }
    let trace1 = Scatter::new(time_line.clone(), price_line).name("price");
    let trace2 = Scatter::new(time_line.clone(), volume_line).name("volume").y_axis("y2");
    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.add_trace(trace2);

    let min_price = min_max.0.to_f32().unwrap();
    let max_price = min_max.1.to_f32().unwrap();

    let mut title = String::new();
    let mut file_name = String::new();

    fmt::write(&mut title, format_args!("{} Combined Jan 3 2023 ",ticker)).unwrap();
    fmt::write(&mut file_name, format_args!("plots/{}-combined.html",ticker)).unwrap();
    let layout = Layout::new()
        .height(2200)
        .width(4200)
        // .paper_background_color(Rgba::new(20, 11, 5, 0.2))
        // .plot_background_color(Rgba::new(20, 11, 5, 0.25))
        .x_axis(
            Axis::new()
                .grid_color(Rgba::new(255, 255, 255, 1.0))

                .range_slider(RangeSlider::new().visible(true))

        ).title(Title::new(&title))
        .y_axis(Axis::new().title("price".into())
            .grid_color(Rgba::new(255, 255, 255, 0.25))
            .dtick(0.125)
            .side(AxisSide::Left)
        )
        .y_axis2(Axis::new().title("volume".into())
            .grid_color(Rgba::new(255, 0, 0, 0.25))
            .overlaying("y")
            .dtick(50.0)
            .side(AxisSide::Right)

        );
    plot.set_layout(layout);
    plot.use_local_plotly();
    println!("writing file {}", &file_name);
    plot.write_html(&file_name);
    Ok(())
}


pub fn test_plot_004(ticker:&str, time_series: Vec<(String, f32, f32)>, min_max: (Decimal, Decimal, i32, i32)) -> Result<(), Box<dyn Error>> {
    let mut time_line: Vec<String> = Vec::with_capacity(time_series.len());
    let mut price_line: Vec<f32> = Vec::with_capacity(time_series.len());
    let mut fft_line: Vec<f32> = Vec::with_capacity(time_series.len());
    for (a, b, c) in time_series {
        time_line.push(a);
        price_line.push(b);
        fft_line.push(c);
    }
    let trace1 = Scatter::new(time_line.clone(), price_line).name("price");
    let trace2 = Scatter::new(time_line.clone(), fft_line).name("fft_price").y_axis("y2");
    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.add_trace(trace2);

    let min_price = min_max.0.to_f32().unwrap();
    let max_price = min_max.1.to_f32().unwrap();

    let mut title = String::new();
    let mut file_name = String::new();

    fmt::write(&mut title, format_args!("{} Price_fft Jan 3 2023 ",ticker)).unwrap();
    fmt::write(&mut file_name, format_args!("plots/{}-price_fft.html",ticker)).unwrap();
    let layout = Layout::new()
        .height(2200)
        .width(4200)
        // .paper_background_color(Rgba::new(20, 11, 5, 0.2))
        // .plot_background_color(Rgba::new(20, 11, 5, 0.25))
        .x_axis(
            Axis::new()
                .grid_color(Rgba::new(255, 255, 255, 1.0))

                .range_slider(RangeSlider::new().visible(true))

        ).title(Title::new(&title))
        .y_axis(Axis::new().title("price".into())
            .grid_color(Rgba::new(255, 255, 255, 0.25))
            .side(AxisSide::Left)
        )
        .y_axis2(Axis::new().title("fft_amplitude".into())
            .grid_color(Rgba::new(255, 0, 0, 0.25))
            .overlaying("y")
            .side(AxisSide::Right)

        );
    plot.set_layout(layout);
    plot.use_local_plotly();
    println!("writing file {}", &file_name);
    plot.write_html(&file_name);
    Ok(())
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn t_001() {
        let mut time_series: Vec<(String, f32, i32)> = Vec::new();
        for i in 1..100 {
            let tt = format!("09:20:00.4917207{:2}", i);
            time_series.push((tt, i as f32, i));
        }
        test_plot_001(time_series).unwrap();
    }
}