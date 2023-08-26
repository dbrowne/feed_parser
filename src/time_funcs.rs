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

// Convenience library to address NYSE source time is in HH:MM:SS.nnnnnnnnn format
// That's right HH:MM:SS.nnn_nnn_nnn  gotta deal with Nanoseconds
// Based on Tiany Shi's hhmmss crate https://crates.io/crates/hhmmss

use rust_decimal::prelude::*;

const  BILLION: i64 = 1_000_000_000;
const  HOUR: f64 = 3_600.0;
const  MINUTE: f64 = 60.0;



/// Trait `Hhmmss` provides methods to represent time durations in `HH:MM:SS` and `HH:MM:SS.xxxxxxxxx` formats.

pub trait Hhmmss {
    /// Returns the duration as a tuple of seconds and nanoseconds.

    fn sns(&self) -> (i64, i64);
    /// Pretty-prints the duration in the format `HH:MM:SS`.
    ///
    /// This method ignores the nanosecond component of the duration.
    fn hhmmss(&self) -> String {
        let (s, _) = self.sns();
        s2hhmmss_64(s)
    }

    /// Pretty-prints the duration in the format `HH:MM:SS.xxxxxxxxx`.
    fn hhmmssnn(&self) -> String {
        let (s, ns) = self.sns();
        sms2hhmmsnn(s, ns)
    }


}

impl Hhmmss for chrono::Duration {
    fn sns(&self) -> (i64, i64) {
        let s = self.num_seconds();
        let ns = match  self.num_nanoseconds(){
            Some(ns) => ns - (s * BILLION),
            None => 0,
        };
        (s, ns)
    }
}

impl Hhmmss for std::time::Duration {
    fn sns(&self) -> (i64, i64) {
        let s = self.as_secs();
        let ns = self.subsec_nanos();
        (s as i64, ns as i64)
    }
}

impl Hhmmss for time::Duration {
    fn sns(&self) -> (i64, i64) {
        let s = self.whole_seconds();
        let ns = self.whole_nanoseconds() - (s as i128 * BILLION as i128)  ;
        (s, ns as i64)
    }
}
/// Converts a duration represented in seconds into its `HH:MM:SS` string representation.
///
/// If the duration is negative, the output string will start with a '-'.
///
/// # Arguments
///
/// * `s` - The duration in seconds.
///
/// # Returns
///
/// * `String` - Formatted string in `HH:MM:SS` format.
pub  fn s2hhmmss_64(s: i64) -> String {
    let mut neg = false;
    let mut s = s;
    if s < 0 {
        neg = true;
        s = -s;
    }
    let (h, s) = (s / 3600, s % 3600);
    let (m, s) = (s / 60, s % 60);
    format!("{}{:02}:{:02}:{:02}", if neg { "-" } else { "" }, h, m, s)
}

/// Converts a duration represented in seconds (as i32) into its `HH:MM:SS` string representation.
///
/// If the duration is negative, the output string will start with a '-'.
///
/// # Arguments
///
/// * `s` - The duration in seconds.
///
/// # Returns
///
/// * `String` - Formatted string in `HH:MM:SS` format.
pub fn s2hhmmss_32(s: i32) -> String {
    let mut neg = false;
    let mut s = s;
    if s < 0 {
        neg = true;
        s = -s;
    }
    let (h, s) = (s / 3600, s % 3600);
    let (m, s) = (s / 60, s % 60);
    format!("{}{:02}:{:02}:{:02}", if neg { "-" } else { "" }, h, m, s)
}

/// Converts a duration represented in seconds and nanoseconds into its `HH:MM:SS.xxxxxxxxx` string representation.
///
/// If the duration is negative, the output string will start with a '-'.
///
/// # Arguments
///
/// * `s` - The duration in seconds.
/// * `ns` - The nanoseconds component of the duration.
///
/// # Returns
///
/// * `String` - Formatted string in `HH:MM:SS.xxxxxxxxx` format.
fn sms2hhmmsnn(s: i64, ns: i64) -> String {
    let mut neg = false;
    let (mut s, mut ns) = (s, ns);
    if s < 0 {
        neg = true;
        s = -s;
        ns = -ns;
    }
    let (h, s) = (s / 3600, s % 3600);
    let (m, s) = (s / 60, s % 60);
    format!(
        "{}{:02}:{:02}:{:02}.{:09}",
        if neg { "-" } else { "" },
        h,
        m,
        s,
        ns
    )
}
/// Converts a time string formatted as "HH:MM:SS.SSSSSSSSS" into a string representation
/// of the total number of seconds followed by a period and nanoseconds.
///
/// # Arguments
///
/// * `time` - A string slice that holds the time formatted as "HH:MM:SS.SSSSSSSSS".
///
/// # Returns
///
/// * `Ok(String)` - If the input is successfully parsed, the function returns a formatted string
///   representation of the total seconds, followed by a period and nanoseconds, in the format
///   "SSSSSSSSS.SSSSSSSSS" where the left side of the period represents seconds and the right
///   side represents nanoseconds (up to 9 digits of precision).
///
/// * `Err(Box<dyn std::error::Error>)` - If there's any issue in parsing the input string or
///   performing calculations.
///
/// # Example
///
/// ```
/// let input = "01:30:30.123456789";
/// let result = time_dec_string(input).unwrap();
/// assert_eq!(result, "5430.123456789");
/// ```
///
/// # Errors
///
/// The function will return an error if:
/// * The input string is not in the expected format.
/// * Any of the time components (hours, minutes, seconds) cannot be parsed into integers.
///
/// # Panics
///
/// This function can panic if the input string does not have the expected number of split elements.
/// This means if you don't provide a correct formatted string like "HH:MM:SS.SSSSSSSSS", it can panic.
///
pub  fn  time_dec_string(time: &str) -> Result<String, Box<dyn std::error::Error>>{
     let t_secs: Vec<&str> = time.split('.').collect();
        let t_hmss: Vec<&str> = t_secs[0].split(':').collect();
        let t_h = t_hmss[0].parse::<i32>()?;
        let t_m = t_hmss[1].parse::<i32>()?;
        let t_s = t_hmss[2].parse::<i32>()?;
        let  seconds = t_h*3600 + t_m*60 + t_s;
       Ok(format!("{}.{:09}", seconds, t_secs[1]))
}

/// Converts a time string in the format "HH:MM:SS.nnnnnnnnn" to its equivalent in decimal seconds.
///
/// # Arguments
///
/// * `time` - A string slice that represents the time in the format "HH:MM:SS.nnnnnnnnn".
///
/// # Returns
///
/// * `Ok(f64)` - If the input is successfully parsed, the function returns the total time
///   in decimal seconds as a floating-point number.
///
/// * `Err(Box<dyn std::error::Error>)` - If there's any issue in parsing the input string
///   or performing calculations.
///
/// # Example
///
/// ```
/// let input = "01:30:30.5";
/// let result = time_to_dec(input).unwrap();
/// assert_eq!(result, 5430.5);
/// ```
///
/// # Errors
///
/// The function will return an error in the following situations:
/// * The input string is not in the expected format.
/// * Any of the time components (hours, minutes, or seconds) cannot be parsed into a float.
///
/// # Panics
///
/// This function does not panic under normal operations, but misuse (like not handling errors)
/// can lead to panic in the calling code.
///
pub  fn time_to_dec(time: &str) -> Result<f64, Box<dyn std::error::Error>> {
    let parts: Vec<&str> = time.split(':').collect();

    if parts.len() != 3 {
        return Err(From::from("Incorrect format. Expected HH:MM:SS.nnnnnnnnn"));
    }

    let hours: f64 = parts[0].parse()?;
    let minutes: f64 = parts[1].parse()?;
    let seconds: f64 = parts[2].parse()?;

    Ok(hours * 3600.0 + minutes * 60.0 + seconds)
}

pub  fn  decimal2hhmmssnnn(inp:Decimal) ->String {
    let  seconds = inp.to_f64().unwrap();
    let hours:i32= (seconds/HOUR) as  i32;
    let minutes:i32 = (seconds%HOUR/MINUTE) as  i32;
    let  sec:f64 = seconds%MINUTE;
    let frac = seconds.fract();
    let  fract_str = format!("{:09.9}", frac);
    let  frac_sub = &fract_str[1..]; // remove the leading 0
    format!("{:02}:{:02}:{:02}{}", hours, minutes, sec as i32,frac_sub)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decimal2hhmmssnnn(){
        let inp = Decimal::from_f64(5430.123456789).unwrap();
        let result = decimal2hhmmssnnn(inp);
        assert_eq!(result, "01:30:30.123456789");
    }
    #[test]
    fn test_time_dec_string(){
        let time = "07:00:00.044382720";
        let dec = time_dec_string(time).unwrap();
        assert_eq!(dec, "25200.044382720");
    }

    #[test]
    fn test_all() {
        let std_duration = std::time::Duration::new(3661, 534_100_100);
        assert_eq!(&std_duration.hhmmss(), "01:01:01");
        assert_eq!(&std_duration.hhmmssnn(), "01:01:01.534100100");
        let chrono_duration = chrono::Duration::from_std(std_duration).unwrap();
        assert_eq!(&chrono_duration.hhmmss(), "01:01:01");
        assert_eq!(&chrono_duration.hhmmssnn(), "01:01:01.534100100");
        let time_duration = time::Duration::new(3661, 534_100_100);
        assert_eq!(&time_duration.hhmmss(), "01:01:01");
        assert_eq!(&time_duration.hhmmssnn(), "01:01:01.534100100");
    }


    #[test]
    fn test_time_to_dec() {
        let time = "09:30:01.00005090000";
        let dec = time_to_dec(time).unwrap();
        assert_eq!(dec, 34201.0000509);
    }
}