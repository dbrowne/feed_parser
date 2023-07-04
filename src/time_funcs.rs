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

const  BILLION: i64 = 1_000_000_000;

pub trait Hhmmss {
    fn sns(&self) -> (i64, i64);
    /// Pretty-prints a chrono::Duration in the form `HH:MM:SS.xxxxxxxxx`
    fn hhmmss(&self) -> String {
        let (s, ns) = self.sns();
        s2hhmmss(s)
    }
    /// Pretty-prints a chrono::Duration in the form `HH:MM:SS.xxxxxxxxx`
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

fn s2hhmmss(s: i64) -> String {
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

#[cfg(test)]
mod tests {
    use super::*;

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