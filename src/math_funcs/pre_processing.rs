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
use approx; // For the macro relative_eq!
use nalgebra as na;


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
fn detrend(inp: &Vec<f32>) -> Vec<f32> {
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


#[cfg(test)]
mod  test{
    use approx::assert_relative_eq;

    #[test]
    fn t_001(){

        let  inp:Vec<f32> = vec![1.0,2.0,3.0,4.0,5.0,5.5,4.5,4.0,4.1,4.2,4.3,4.4,4.5,4.6,4.7,4.0];
        let  out:Vec<f32> = super::detrend(&inp);
        let ans:Vec<f32> = vec![-1.9816177, -1.1157353, -0.24985294, 0.61602944, 1.4819118,
                                1.8477942, 0.7136765, 0.07955884, 0.045441102, 0.011323363,
                                -0.0227939, -0.05691164, -0.091029376, -0.12514712, -0.15926485,
                                -0.99338233];
        for pair     in out.iter().zip(ans.iter()) {
            assert_relative_eq!(pair.0, pair.1);
        }
        println!("{:?}",out);
    }
}
