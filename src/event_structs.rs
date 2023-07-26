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



pub const EXPECTED_TICS: usize = 16;

use std::collections::BTreeMap;
use std::f64;
use rust_decimal::prelude::*;
use rust_decimal_macros::dec;
use std::error::Error;
use crate::time_funcs::time_dec_string;


/// `MuEvent` represents a single instance of a market event at microsecond resolution.
///
/// It consists of the following fields:
/// - `string_time`: The time of the event in HH:MM:SS.nnnnnnnnn format. needed for graphs
/// - `seconds`: The time of the event in seconds.
/// - `price`: The price at which the event occurred.
/// - `volume`: The volume of the event.
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct MuEvent {
    pub string_time: String,
    // really in HH:MM:SS.nnnnnnnnn format
    pub seconds: Decimal,
    pub price: Decimal,
    pub volume: i32,
}

/// `Event` is a collection of `MuEvent`s and associated metadata.
///
/// It consists of the following fields:
/// - `tics`: A vector of market events (`MuEvent`).
/// - `total_price`: The total price of all events in `tics`.
/// - `total_volume`: The total volume of all events in `tics`.
/// - `tic_count`: The total number of events in `tics`.
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Event {
    pub tics: Vec<MuEvent>,
    pub total_price: Decimal,
    pub total_volume: i32,
    pub tic_count: i32,
}

/// `EventList` holds a map of `Event`s with the key being the time in seconds of the event.
///
/// It consists of the following fields:
/// - `events`: A BTreeMap where the key is an integer and the value is an `Event`.
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct EventList {
    pub events: BTreeMap<i32, Event>,
}

impl EventList {
    /// Create a new instance of `EventList`.
    pub fn new() -> EventList {
        EventList {
            events: BTreeMap::new(),
        }
    }

    /// Update an `Event` within the `EventList`. If an `Event` with the given index does not exist, a new one is created.
    pub fn update(&mut self, seconds: &str, f_price: &str, volume: i32) -> Result<(), Box<dyn Error>> {
        let f_second = time_dec_string(seconds)?;
        // Convert seconds to Decimal type
        let seconds_decimal = Decimal::from_str(&f_second)
            .map_err(|err| format!("Failed to parse seconds: {}", err))?;

        // Get the index by converting seconds to i32
        let idx = seconds_decimal.to_i32()
            .ok_or("Failed to convert seconds to i32")?;

        // Convert f_price to Decimal type
        let price = Decimal::from_str(f_price)
            .map_err(|err| format!("Failed to parse f_price: {}", err))?;

        // Check if event already exists for the given index
        if let Some(event) = self.events.get_mut(&idx) {
            // If event exists, update it
            event.update(price, volume, seconds_decimal, seconds.to_string());
        } else {
            // If event doesn't exist, create a new one and insert it
            self.events.insert(idx, Event::new(price, volume, seconds_decimal, seconds.to_string()));
        }

        Ok(())
    }

    /// Calculate the total volume of all `Event`s within the `EventList`.
    pub fn get_volume(&self) -> i32 {
        let mut total_volume = 0;
        for (_, event) in self.events.iter() {
            total_volume += event.get_volume();
        }
        total_volume
    }
    /// Calculate the average price across all `Event`s within the `EventList`.
    pub fn get_average_price(&self) -> Decimal {
        let mut total_price: Decimal = dec!(0.0);
        let mut total_tics = 0;
        for (_, event) in self.events.iter() {
            total_price += event.total_price;
            total_tics += event.tic_count;
        }
        if total_tics == 0 {
            return dec!(0.0);
        }
        total_price / Decimal::from_i32(total_tics).unwrap()
    }
    /// Calculate the total count of `MuEvent`s across all `Event`s within the `EventList`.
    pub fn get_event_count(&self) -> usize {
        let mut total_tics: i32 = 0;
        for (_, event) in self.events.iter() {
            total_tics += event.tic_count;
        }
        total_tics as usize
    }
    /// Generate a time series of `MuEvent`s as a vector of tuples (seconds, price, volume).
    pub fn get_time_series(&self) -> Vec<(Decimal, Decimal, i32)> {
        let mut time_series: Vec<(Decimal, Decimal, i32)> = Vec::with_capacity(self.get_event_count());
        for (idx, event) in self.events.iter() {
            for tic in event.tics.iter() {
                time_series.push((tic.seconds, tic.price, tic.volume));
            }
        }
        time_series
    }
    /// Generate a time series of `Event`s as a vector of tuples (idx, average price, volume).
    pub fn get_sec_avg_time_series(&self) -> Vec<(i32, Decimal, i32)> {
        let mut time_series: Vec<(i32, Decimal, i32)> = Vec::with_capacity(self.get_event_count());
        for (idx, event) in self.events.iter() {
            time_series.push((idx.clone(), event.get_avg_price(), event.get_volume()));
        }
        time_series
    }


    /// Generate a time series of `MuEvent`s as a vector of tuples (seconds, price, volume) as float.
    pub fn get_time_series_s(&self) -> Vec<(String, f32, i32)> {
        let mut time_series: Vec<(String, f32, i32)> = Vec::with_capacity(self.get_event_count());
        for (idx, event) in self.events.iter() {
            for tic in event.tics.iter() {
                time_series.push((tic.string_time.clone(), tic.price.to_f32().unwrap(), tic.volume));
            }
        }
        time_series
    }
    /// Generate a time series of `Event`s as a vector of tuples (idx, average price, volume).
    pub fn get_sec_avg_time_series_s(&self) -> Vec<(String, f32, i32)> {
        let mut time_series: Vec<(String, f32, i32)> = Vec::with_capacity(self.get_event_count());
        for (idx, event) in self.events.iter() {
            time_series.push((s2hhmmss(idx.clone()), event.get_avg_price().to_f32().unwrap(), event.get_volume()));
        }
        time_series
    }
}

// THIS IS CUT AND PASTE WILL REMOVE LATER!!! todo!!
fn s2hhmmss(s: i32) -> String {
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

impl Event {
    /// Create a new `Event` with an initial `MuEvent`.
    pub fn new(price: Decimal, volume: i32, seconds: Decimal, string_time: String) -> Event {
        let mut initial_tic = Vec::with_capacity(EXPECTED_TICS);
        initial_tic.push(MuEvent { string_time, seconds: seconds, price, volume });
        Event {
            tics: initial_tic,
            total_price: price,
            total_volume: volume,
            tic_count: 1,
        }
    }
    // would have error checking here if this were production code  but it's not
    /// Update an `Event` with a new `MuEvent` and recalculate the total price, total volume and tic_count.
    pub fn update(&mut self, price: Decimal, volume: i32, u_sec: Decimal, string_time: String) {
        self.total_price += price;
        self.total_volume += volume;
        self.tic_count += 1;
        self.tics.push(MuEvent { string_time, seconds: u_sec, price, volume });
    }
    /// Calculate the average price of all `MuEvent`s within the `Event`.
    pub fn get_avg_price(&self) -> Decimal {
        self.total_price / Decimal::from_i32(self.tic_count).unwrap()
    }
    /// Calculate the total volume of all `MuEvent`s within the `Event`.
    pub fn get_volume(&self) -> i32 {
        self.total_volume
    }
}

#[cfg(test)]
mod test {
    use crate::event_structs::EventList;
    use rust_decimal::prelude::*;
    use rust_decimal_macros::dec;


    #[test]
    fn test_event_list() {
        let el = EventList::new();
        assert_eq!(el.events.len(), 0);
    }

    #[test]
    fn test_event_list_insert() {
        let mut el = EventList::new();
        el.update("07:01:45.491720704", "1.0", 1);
        assert_eq!(el.events.len(), 1);
    }

    #[test]
    fn test_event_list_update() {
        let mut el = EventList::new();
        el.update("07:01:45.491720704", "1.15", 1);
        el.update("07:01:45.491720704", "1.257", 1);
        assert_eq!(el.events.len(), 1);
    }

    #[test]
    fn test_event_list_volume() {
        let mut el = EventList::new();
        el.update("07:01:45.491720704", "1.15", 1);
        el.update("07:01:45.491720704", "1.257", 1);
        assert_eq!(el.get_volume(), 2);
        el.update("07:01:47.493720704", "1.257", 5);
        assert_eq!(el.get_volume(), 7);
    }

    #[test]
    fn test_event_list_average_price() {
        let mut el = EventList::new();
        el.update("07:01:45.491720704", "6.0", 1);
        el.update("07:01:45.491720705", "12.0", 1);
        assert_eq!(el.get_average_price(), dec!(9.0));
    }

    #[test]
    fn test_get_time_series() {
        let mut el = EventList::new();
        el.update("09:20:00.491720704", "6.0", 1);
        el.update("09:20:00.491720705", "12.0", 1);
        el.update("09:20:00.491730704", "11.37", 1);
        let ans: Vec<(Decimal, Decimal, i32)> = vec![
            (Decimal::new(33600491720704, 9), dec!(6.0), 1),
            (Decimal::new(33600491720705, 9), dec!(12.0), 1),
            (Decimal::new(33600491730704, 9), dec!(11.37), 1)];
        assert_eq!(el.get_time_series(), ans);
    }

    #[test]
    fn test_get_time_series2() {
        let mut el = EventList::new();
        el.update("09:20:00.491720704", "6.0", 1);
        el.update("09:20:00.491720705", "12.0", 1);
        el.update("09:20:00.491730704", "11.37", 1);
        el.update("09:21:00.491730707", "11.38", 1);
        let ans: Vec<(Decimal, Decimal, i32)> = vec![
            (Decimal::new(33600491720704, 9), dec!(6.0), 1),
            (Decimal::new(33600491720705, 9), dec!(12.0), 1),
            (Decimal::new(33600491730704, 9), dec!(11.37), 1),
            (Decimal::new(33660491730707, 9), dec!(11.38), 1)];
        assert_eq!(el.get_time_series(), ans);
    }


    #[test]
    fn test_get_time_series4() {
        let mut el = EventList::new();
        el.update("09:20:00.491720704", "6.0", 1);
        el.update("09:20:00.491720705", "12.0", 2);
        el.update("09:20:00.491730704", "11.37", 3);
        el.update("09:21:00.491730707", "11.38", 4);

        let ans: Vec<(String, f32, i32)> = vec![
            ("09:20:00.491720704".to_string(), 6.0, 1),
            ("09:20:00.491720705".to_string(), 12.0, 2),
            ("09:20:00.491730704".to_string(), 11.37, 3),
            ("09:21:00.491730707".to_string(), 11.38, 4),

        ];
        assert_eq!(el.get_time_series_s(), ans);
    }

    #[test]
    fn test_get_sec_avg_time_series() {
        let mut el = EventList::new();
        el.update("09:20:00.491720704", "3.0", 10);
        el.update("09:20:00.491720705", "6.0", 10);
        el.update("09:20:00.491730704", "9.0", 10);
        let ts = el.get_sec_avg_time_series();
        assert_eq!(ts, vec![(33_600, dec!(6.0), 30)]);
    }

    #[test]
    fn test_get_sec_avg_time_series2() {
        let mut el: EventList = EventList::new();
        el.update("09:20:00.491720704", "3.0", 10);
        el.update("09:20:00.496720704", "6.0", 10);
        el.update("09:20:00.491920704", "9.0", 10);
        el.update("09:20:01.496720704", "3.0", 20);
        el.update("09:20:01.496720784", "3.0", 20);
        el.update("09:20:11.496720784", "9.0", 20);
        let ts = el.get_sec_avg_time_series();
        let ans: Vec<(i32, Decimal, i32)> = vec![(33_600, dec!(6.0), 30),
                                                 (33_601, dec!(3.0), 40),
                                                 (33_611, dec!(9.0), 20)];
        assert_eq!(ts, ans)
    }


    #[test]
    fn test_get_sec_avg_time_series3() {
        let mut el: EventList = EventList::new();
        el.update("09:20:00.491720704", "3.0", 10);
        el.update("09:20:00.496720704", "6.0", 10);
        el.update("09:20:00.491920704", "9.0", 10);
        el.update("09:20:01.496720704", "3.0", 20);
        el.update("09:20:01.496720784", "3.0", 20);
        el.update("09:20:11.496720784", "9.0", 20);
        let ts = el.get_sec_avg_time_series_s();
        let ans: Vec<(String, f32, i32)> = vec![("09:20:00".to_string(), 6.0, 30),
                                                 ("09:20:01".to_string(),3.0, 40),
                                                 ("09:20:11".to_string(),9.0, 20)];
        assert_eq!(ts, ans)
    }

}