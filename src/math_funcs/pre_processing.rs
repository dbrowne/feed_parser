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
//
#[macro_use]
use approx;
// For the macro relative_eq!
use rustfft::{FftPlanner, num_complex::Complex};
use std::f32::consts::PI;
use welch_sde::{Build, Periodogram, PowerSpectrum, SpectralDensity};
use std::fmt::Write;
use rand::{thread_rng, Rng};
use std::time::Instant;
/// Extracts and separates the elements from a vector of tuples into individual vectors.
///
/// Given a vector of tuples where each tuple contains a `String`, `f32`, and `i32`,
/// this function separates the elements of each tuple into their respective vectors.
///
/// # Arguments
///
/// * `input` - A reference to a vector of tuples. Each tuple contains:
///   * A `String`
///   * An `f32` floating point number
///   * An `i32` integer
///
/// # Returns
///
/// A tuple containing three vectors:
///   * A `Vec<String>` containing all the `String` elements from the input tuples.
///   * A `Vec<f32>` containing all the `f32` floating point numbers from the input tuples.
///   * A `Vec<i32>` containing all the `i32` integers from the input tuples.
///
/// # Examples
///
/// ```
/// let data = vec![
///     ("apple".to_string(), 1.2, 5),
///     ("banana".to_string(), 3.4, 8),
/// ];
///
/// let (strings, floats, ints) = extract_elements(&data);
///
/// assert_eq!(strings, vec!["apple".to_string(), "banana".to_string()]);
/// assert_eq!(floats, vec![1.2, 3.4]);
/// assert_eq!(ints, vec![5, 8]);
/// ```
pub fn extract_elements(input: &Vec<(String, f32, i32)>) -> (Vec<String>, Vec<f32>, Vec<i32>) {
    let strings: Vec<String> = input.iter().map(|(s, _, _)| s.clone()).collect();
    let floats: Vec<f32> = input.iter().map(|(_, f, _)| *f).collect();
    let ints: Vec<i32> = input.iter().map(|(_, _, i)| *i).collect();

    (strings, floats, ints)
}


pub fn extract_elements_f32(input: Vec<(String, f32, f32)>) -> (Vec<String>, Vec<f32>, Vec<f32>) {
    let strings: Vec<String> = input.iter().map(|(s, _, _)| s.clone()).collect();
    let float1: Vec<f32> = input.iter().map(|(_, f, _)| *f).collect();
    let float2: Vec<f32> = input.iter().map(|(_, _, t)| *t).collect();
    (strings, float1, float2)
}

/// Detrends a time series by removing a linear trend.
///
/// This function takes a vector of stock prices (or any other time series data)
/// and returns a new vector where the linear trend has been removed.
/// It uses ordinary least squares to fit a straight line to the data
/// and then subtracts this line from the original data to remove the trend.
///
/// # Arguments
///
/// * `inp` - A reference to a vector of f32 values representing the time series to detrend.
///
/// # Returns
///
/// * A vector of f32 values representing the detrended time series.
///
/// # Example
///
/// ```
/// let stock_prices = vec![100.0, 101.0, 102.0, 103.0, 104.0];
/// let detrended_prices = detrend(&stock_prices);
/// println!("{:?}", detrended_prices);
/// ```
pub fn detrend(inp: &Vec<f32>) -> Vec<f32> {
    let data: Vec<f64> = inp.iter().map(|x| *x as f64).collect();
    let n = data.len() as f64;
    let x: Vec<f64> = (0..data.len()).map(|i| i as f64).collect();

    // Compute the means of x and y
    let x_mean = x.iter().sum::<f64>() / n;
    let y_mean = data.iter().sum::<f64>() / n;

    // Compute the slope (beta) and intercept (alpha) of the linear trend
    let beta: f64 = x.iter().zip(data.iter())
        .map(|(xi, yi)| (xi - x_mean) * (yi - y_mean))
        .sum::<f64>()
        /
        x.iter().map(|xi| (xi - x_mean).powi(2)).sum::<f64>();

    let alpha = y_mean - beta * x_mean;

    // Subtract the linear trend from the original data
    let detrended: Vec<f32> = x.iter().map(|xi| xi * beta + alpha).zip(data.iter())
        .map(|(trend_i, yi)| (yi - trend_i) as f32)
        .collect();

    detrended
}

/// Calculates the variance of a given list of floating point numbers.
///
/// The variance is a measure of how spread out the numbers in a data set are.
/// This function computes the variance of the provided list of numbers using the formula:
/// \[ \text{variance} = \frac{1}{n} \sum_{i=1}^{n} (x_i - \text{mean})^2 \]
/// where \( n \) is the number of elements in the list, \( x_i \) is an element of the list,
/// and \(\text{mean}\) is the average of all elements in the list.
///
/// # Arguments
///
/// * `inp` - A reference to a vector of `f32` numbers for which the variance will be calculated.
///
/// # Returns
///
/// Returns the variance of the given list of numbers as an `f64`.
///
/// # Panics
///
/// The function will panic if the provided vector `inp` is empty, as variance for an empty set is undefined.
///
/// # Examples
///
/// ```
/// let data = vec![2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
/// let result = variance(&data);
///
/// assert!((result - 4.571428571428571).abs() < 1e-10);
/// ```
pub fn variance(inp: &Vec<f32>) -> f64 {
    let n = inp.len() as f32;
    let mean = inp.iter().sum::<f32>() / n;
    let mut sum: f64 = inp.iter().map(|x| (x - mean).powi(2) as f64).sum();

    sum / n as f64
}

/// Computes the Fast Fourier Transform (FFT) of a given list of floating point numbers.
///
/// The function takes a vector of real numbers and returns their magnitude in the frequency domain after performing an FFT. The input data is assumed to be real, so the imaginary part is set to zero for each entry. The FFT is computed using a generic planner and the `Complex` type to represent complex numbers.
///
/// # Arguments
///
/// * `inp` - A reference to a vector of `f32` numbers which represent the time domain signal.
///
/// # Returns
///
/// Returns a vector of `f32` numbers representing the magnitude of the frequency domain signal.
///
/// # Dependencies
///
/// This function requires rustfft and ndarray to be included in the project's Cargo.toml file.
///
/// # Examples
///
/// ```
/// // Assuming necessary imports and setup
/// let data = vec![...]; // your time domain data
/// let frequency_magnitude = gen_fft(&data);
/// ```
///
///

pub fn gen_fft(inp: &Vec<f32>) -> Vec<f32> {
    let mut planner = FftPlanner::<f32>::new();
    let fft = planner.plan_fft_forward(inp.len());
    let mut complex_buffer: Vec<Complex<f32>> = inp.iter().map(|x| Complex::new(*x, 0.0)).collect();
    fft.process(&mut complex_buffer);
    let out: Vec<f32> = complex_buffer.iter().map(|x| x.norm()).collect();
    out
}


/// Computes the Fast Fourier Transform (FFT) of a given list of tuples, extracting data from the third element of each tuple.
///
/// This function acts as a wrapper around the `gen_fft` function. It expects a vector of tuples where each tuple consists of a `String`, `f32`, and another `f32`. The function then extracts the third element (i.e., the second `f32` value) from each tuple to form a new vector. The FFT is then computed for this extracted vector, and the resulting magnitudes are returned.
///
/// # Arguments
///
/// * `inp` - A reference to a vector of tuples where each tuple contains:
///   * A `String`
///   * An `f32` floating point number (not used in the FFT computation)
///   * Another `f32` floating point number which represents the signal data to be transformed
///
/// # Returns
///
/// Returns a vector of `f32` numbers representing the magnitude of the frequency domain signal after performing an FFT on the third elements of the provided tuples.
///
/// # Dependencies
///
/// This function internally calls the `gen_fft` function. Ensure that both functions are defined in the same module or that appropriate imports are in place.
///
/// # Examples
///
/// ```
/// // Assuming necessary imports and setup
/// let data = vec![
///     ("Sample1".to_string(), 1.0, 2.0),
///     ("Sample2".to_string(), 3.0, 4.0),
///     // ... more data
/// ];
/// let frequency_magnitudes = fft_wrapper(&data);
/// ```
///
/// # Notes
///
/// The first and second elements of each tuple in the input vector are ignored in the FFT computation.
///
pub fn fft_wrapper(inp: &Vec<(String, f32, f32)>) -> Vec<(f32)> {
    let mut out: Vec<(String, f32, f32)> = Vec::with_capacity(inp.len());
    let data: Vec<f32> = inp.iter().map(|x| x.2).collect();
    let mut fft_data = gen_fft(&data);
    fft_data
}


pub fn gen_price_with_fft(inp: &Vec<(String, f32, i32)>) -> Vec<(String, f32, f32)> {
    let mut out: Vec<(String, f32, f32)> = Vec::with_capacity(inp.len());
    let (_, floats, _) = extract_elements(inp);
    let detrended_data = detrend(&floats);
    let fft_data = gen_fft(&detrended_data);
    let output: Vec<(String, f32, f32)> = inp.iter().zip(fft_data.iter()).map(|(x, y)| (x.0.clone(), x.1, *y)).collect();
    output
}


pub fn power_spectrum(inp: &Vec<f32>) -> (Vec<f32>, f32) {
    let welch: PowerSpectrum<f32> = PowerSpectrum::builder(&inp).build();

    let power_spec = welch.periodogram();

    let variance = 2.0 * power_spec.iter().sum::<f32>();
    let  v_ps: Vec<f32> = power_spec.iter().map(|x| x.clone() ).collect();
    (v_ps, variance)
}


pub fn spectral_density(inp: &Vec<f32>) -> (Vec<f32>, f32) {
    let fs = 10e3_f32;
    let welch: SpectralDensity<f32> = SpectralDensity::<f32>::builder(&inp, fs).build();
    let spectral_density = welch.periodogram();
    let noise_floor = spectral_density.iter().clone().sum::<f32>() / spectral_density.len() as f32;
    let  v_sd: Vec<f32> = spectral_density.iter().map(|x| x.clone() ).collect();
    (v_sd, noise_floor)
}


///// INTERNAL TEST FUNCTIONS /////
pub  fn generate_sin_wave(samples: usize, frequency: f32, max_time_seconds: f32) -> Vec<(String, f32, f32)> {
    let mut result = Vec::new();

    for i in 0..samples {
        let t = i as f32 / samples as f32 * max_time_seconds;
        let sin_val = (2.0 * PI * frequency * t).sin();

        // Convert time to HH:MM:SS.sss format
        let hours = (t as u32) / 3600;
        let minutes = (t as u32 % 3600) / 60;
        let seconds = t as u32 % 60;
        let millis = ((t % 1.0) * 1000.0) as u32;
        let timestamp = format!("{:02}:{:02}:{:02}.{:03}", hours, minutes, seconds, millis);

        result.push((timestamp, sin_val * 100.0, i as f32));
    }

    result
}

fn generate_time_series(hours: u32, minutes: u32, seconds: u32, n: u32) -> Vec<String> {
    let mut rng = thread_rng();
    let mut results = Vec::new();

    let mut last_microsecond = 0;

    for h in 0..hours {
        for m in 0..minutes {
            for s in 0..seconds {
                let step = (1_000_000 - last_microsecond) / n;
                for _ in 0..n {
                    let range_start = last_microsecond + 1;
                    let range_end = last_microsecond + step;
                    let microseconds = rng.gen_range(range_start..range_end);
                    last_microsecond = microseconds;

                    let mut timestamp = String::new();
                    write!(&mut timestamp, "{:02}:{:02}:{:02}.{:06}", h, m, s, microseconds).unwrap();
                    results.push(timestamp);
                }
            }
            last_microsecond = 0;  // Reset for the next minute.
        }
    }

    results
}
pub  fn generate_series(max_items: i32) -> Vec<(String, f32, i32)> {
    let mut out: Vec<(String, f32, i32)> = Vec::with_capacity(max_items as usize);
    for i in 0..max_items {
        let time = format!("{}:00:00.000", i);
        let price = (i % 32) as f32 + 0.25;
        let volume = i as i32;
        out.push((time, price, volume));
    }
    out
}

pub  fn  sd_graph(inp:&Vec<f32>){


    let  signal:Vec<f64> = inp.iter().map(|x| *x as f64).collect();
    let fs:f64 = 80.0;
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


#[cfg(test)]
mod test {
    use std::arch::x86_64::__cpuid;
    use approx::assert_relative_eq;
    use crate::math_funcs::pre_processing::*;


    #[test]
    fn t_001() {
        let inp: Vec<f32> = vec![1.0, 2.0, 3.0, 4.0, 5.0, 5.5, 4.5, 4.0, 4.1, 4.2,
                                 4.3, 4.4, 4.5, 4.6, 4.7, 4.0];
        let out: Vec<f32> = super::detrend(&inp);
        let ans: Vec<f32> = vec![-1.9816177, -1.1157353, -0.24985294, 0.61602944, 1.4819118,
                                 1.8477942, 0.7136765, 0.07955884, 0.045441102, 0.011323363,
                                 -0.0227939, -0.05691164, -0.091029376, -0.12514712, -0.15926485,
                                 -0.99338233];
        let sum: f32 = ans.iter().sum();

        assert_relative_eq!(((sum*100_0000.0) as i32) as f32, 0.0);
        for pair in out.iter().zip(ans.iter()) {
            assert_relative_eq!(pair.0, pair.1);
        }
        println!("{:?}", out);
    }

    #[test]
    fn test_price_fft() {
        let inp = super::generate_sin_wave(1440, 1.0, 10.0);
        let out = super::fft_wrapper(&inp);
        assert_eq!(out.len(), inp.len());
        // println!("{:?}",inp);
        // println!("{:?}",out);
    }

    #[test]
    fn test_gen_price_with_fft() {
        let inp = generate_series(33);
        let out = super::gen_price_with_fft(&inp);
        assert_eq!(out.len(), inp.len());
        println!("{:?}", inp);
        let (_, _, floats) = extract_elements_f32(out);

        let comp: Vec<f32> = vec![0.0, 41.54049, 32.647892, 30.720274, 30.01681, 29.686068,
                                  29.505411, 29.396557, 29.326359, 29.278873, 29.245668, 29.221977,
                                  29.204952, 29.192831, 29.18451, 29.179295, 29.176779, 29.176779,
                                  29.179295, 29.18451, 29.192831, 29.204952, 29.221977, 29.245668,
                                  29.278873, 29.326359, 29.396557, 29.505411, 29.686068, 30.01681,
                                  30.720274, 32.647892, 41.54049];

        for i in 0..comp.len() {
            assert_relative_eq!(floats[i],comp[i]);
        }
    }

    #[test]
    fn test_variance() {
        let inp = generate_series(33);
        let (_, floats, _) = extract_elements(&inp);
        let detrended_data = detrend(&floats);
        let out = variance(&detrended_data);
        assert_eq!(out as i32, 27);
    }

    #[test]
    fn test_power_spectrum() {
        let inp = generate_series(365);
        let (_, floats, _) = extract_elements(&inp);
        let detrended_data = detrend(&floats);
        let out = power_spectrum(&detrended_data);
        let  p = out.0;
        let  v = out.1;
        assert_eq!(p.len(), 128);
    }

    #[test]
    fn test_spectral_density() {
        let inp = generate_series(365);
        let (_, floats, _) = extract_elements(&inp);
        let detrended_data = detrend(&floats);
        let out = spectral_density(&detrended_data);
        let  p = out.0;
        let  v = out.1;
        assert_eq!(128,p.len() );;
    }



































}

