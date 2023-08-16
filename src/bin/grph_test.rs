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

use rand::prelude::*;
use rand_distr::StandardNormal;
use std::time::Instant;
use welch_sde::{SpectralDensity, Build};

fn main() {
    let n = 1e5 as usize;
    let fs = 10e3_f64;
    let amp = 2. * 2f64.sqrt();
    let freq = 1550f64;
    let noise_power = 0.001 * fs / 2.;
    let sigma = noise_power.sqrt();
    let signal: Vec<f64> = (0..n)
        .map(|i| i as f64 / fs)
        .map(|t| {
            amp * (2. * std::f64::consts::PI * freq * t).sin()
                + thread_rng().sample::<f64, StandardNormal>(StandardNormal) * sigma
        })
        .collect();

    let welch: SpectralDensity<f64> =
        SpectralDensity::<f64>::builder(&signal, fs).build();
    println!("{}", welch);
    let now = Instant::now();
    let sd = welch.periodogram();
    println!(
        "Spectral density estimated in {}ms",
        now.elapsed().as_millis()
    );
    let noise_floor = sd.iter().cloned().sum::<f64>() / sd.len() as f64;
    println!("Noise floor: {:.3}", noise_floor);

    let _: complot::LinLog = (
        sd.frequency()
            .into_iter()
            .zip(&(*sd))
            .map(|(x, &y)| (x, vec![y])),
        complot::complot!(
           "spectral_density.png",
           xlabel = "Frequency [Hz]",
           ylabel = "Spectral density [s^2/Hz]"
       ),
    )
        .into();
}