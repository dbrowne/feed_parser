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
use ndarray::Array;
use plotly::{
    color::{NamedColor, Rgb, Rgba},
    common::{
        ColorScale, ColorScalePalette, DashType, Fill, Font, Line, LineShape, Marker, Mode,
        Orientation, Title,
    },
    layout::{Axis, BarMode, Layout, Legend, TicksDirection, TraceOrder},
    sankey::{Line as SankeyLine, Link, Node},
    Bar, Plot, Sankey, Scatter, ScatterPolar,
};

pub  fn  test_plot_001(time_series:Vec<(String,f32,i32)>) ->Result<(),Box<dyn Error>>{
    let mut x:Vec<String> = Vec::new();
    let mut y:Vec<f32> = Vec::new();
    let mut z:Vec<i32> = Vec::new();
    for (a,b,c) in time_series{
        x.push(a);
        y.push(b);
        z.push(c);
    }
    let trace1 = Scatter::new(x.clone(),y).name("price");
    let trace2 = Scatter::new(x.clone(),z).name("volume");
    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.add_trace(trace2);
    plot.show();
    Ok(())
}


#[cfg(test)]
mod  test{
    use super::*;
    #[test]
    fn t_001() {

        let mut time_series:Vec<(f64,f32,i32)> = Vec::new();
        for i in 1..100{
            time_series.push((i as f64, i as f32, i));
        }
        test_plot_001(time_series).unwrap();
    }
}