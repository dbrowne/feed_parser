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
pub const BILLION: i64 = 1_000_000_000;

use std::collections::BTreeMap;
use std::f64;
use rust_decimal::prelude::*;
use rust_decimal_macros::dec;
use std::error::Error;


/// `MuEvent` represents a single instance of a market event.
///
/// It consists of the following fields:
/// - `seconds`: The time of the event in seconds.
/// - `price`: The price at which the event occurred.
/// - `volume`: The volume of the event./// MuEvent is a struct to hold the microsecond resolution trade event
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct MuEvent {
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

/// `EventList` holds a map of `Event`s with the key being an integer.
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
        // Convert seconds to Decimal type
        let seconds_decimal = Decimal::from_str(seconds)
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
            event.update(price, volume, seconds_decimal);
        } else {
            // If event doesn't exist, create a new one and insert it
            self.events.insert(idx, Event::new(price, volume, seconds_decimal));
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
        if  total_tics ==0 {
            return dec!(0.0);
        }
        total_price / Decimal::from_i32(total_tics).unwrap()
    }
    /// Calculate the total count of `MuEvent`s across all `Event`s within the `EventList`.
    pub fn get_event_count(&self) -> usize {
        let mut total_tics:i32 = 0;
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
}

impl Event {
    /// Create a new `Event` with an initial `MuEvent`.
    pub fn new(price: Decimal, volume: i32, seconds: Decimal) -> Event {
        let mut initial_tic = Vec::with_capacity(EXPECTED_TICS);
        initial_tic.push(MuEvent { seconds: seconds, price, volume });
        Event {
            tics: initial_tic,
            total_price: price,
            total_volume: volume,
            tic_count: 1,
        }
    }
    // would have error checking here if this were production code  but it's not
    /// Update an `Event` with a new `MuEvent` and recalculate the total price, total volume and tic_count.
    pub fn update(&mut self, price: Decimal, volume: i32, u_sec: Decimal) {
        self.total_price += price;
        self.total_volume += volume;
        self.tic_count += 1;
        self.tics.push(MuEvent { seconds: u_sec, price, volume });
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
        el.update("34_200.1001", "1.0", 1);
        assert_eq!(el.events.len(), 1);
    }

    #[test]
    fn test_event_list_update() {
        let mut el = EventList::new();
        el.update("34_200.1001", "1.15", 1);
        el.update("34_200.1201", "1.257", 1);
        assert_eq!(el.events.len(), 1);
    }

    #[test]
    fn test_event_list_volume() {
        let mut el = EventList::new();
        el.update("34_200.1001", "1.15", 1);
        el.update("34_200.1201", "1.257", 1);
        assert_eq!(el.get_volume(), 2);
        el.update("34_220.1201", "1.257", 5);
        assert_eq!(el.get_volume(), 7);
    }

    #[test]
    fn test_event_list_average_price() {
        let mut el = EventList::new();
        el.update("34_200.1001", "6.0", 1);
        el.update("34_200.1201", "12.0", 1);
        assert_eq!(el.get_average_price(), dec!(9.0));
    }

    #[test]
    fn test_get_time_series() {
        let mut el = EventList::new();
        el.update("34_200.1001", "6.0", 1);
        el.update("34_200.1201", "12.0", 1);
        el.update("34_200.1250", "11.37", 1);
        let ans: Vec<(Decimal, Decimal, i32)> = vec![(Decimal::new(34_2001001, 4), dec!(6.0), 1), (Decimal::new(34_2001201, 4), dec!(12.0), 1), (Decimal::new(34_2001250, 4), dec!(11.37), 1)];
        assert_eq!(el.get_time_series(), ans);
    }

    #[test]
    fn test_get_time_series2() {
        let mut el = EventList::new();
        el.update("34_200.1001", "6.0", 1);
        el.update("34_200.1201", "12.5", 2);
        el.update("34_200.1250", "11.0", 3);
        el.update("34_202.1001", "6.0", 4);
        el.update("34_210.1201", "12.0", 5);
        el.update("34_250.1250", "11.0", 6);
        el.update("34_250.22", "11.0", 7);
        el.update("34_250.2450", "11.0", 8);
        el.update("34_350.3450", "11.0", 9);
        el.update("34_351.0", "11.0", 10);
        el.update("34_451.0", "11.0", 12);
        let ans: Vec<(Decimal, Decimal, i32)> = vec![
            (Decimal::new(34_2001001, 4), dec!(6.0), 1),
            (Decimal::new(34_2001201, 4), dec!(12.5), 2),
            (Decimal::new(34_2001250, 4), dec!(11.0), 3),
            (Decimal::new(34_2021001, 4), dec!(6.0), 4),
            (Decimal::new(34_2101201, 4), dec!(12.0), 5),
            (Decimal::new(34_2501250, 4), dec!(11.0), 6),
            (Decimal::new(34_25022, 2), dec!(11.0), 7),
            (Decimal::new(34_2502450, 4), dec!(11.0), 8),
            (Decimal::new(34_3503450, 4), dec!(11.0), 9),
            (Decimal::new(34_3510, 1), dec!(11.0), 10),
            (Decimal::new(34_4510, 1), dec!(11.0), 12),
        ];
        assert_eq!(el.get_time_series(), ans);
    }

    #[test]
    fn test_get_sec_avg_time_series() {
        let mut el = EventList::new();
        el.update("34_200.1001", "3.0", 10);
        el.update("34_200.1201", "6.0", 10);
        el.update("34_200.1250", "9.0", 10);
        let ts = el.get_sec_avg_time_series();
        assert_eq!(ts, vec![(34_200, dec!(6.0), 30)]);
    }

    #[test]
    fn test_get_sec_avg_time_series2() {
        let mut el: EventList = EventList::new();
        el.update("34_200.1001", "3.0", 10);
        el.update("34_200.1201", "6.0", 10);
        el.update("34_200.1250", "9.0", 10);
        el.update("34_202.1001", "3.0", 20);
        el.update("34_202.1201", "3.0", 20);
        el.update("34_203.1250", "9.0", 20);
        let ts = el.get_sec_avg_time_series();
        let ans: Vec<(i32, Decimal, i32)> = vec![(34_200, dec!(6.0), 30),
                                                 (34_202, dec!(3.0), 40),
                                                 (34_203, dec!(9.0), 20)];
        assert_eq!(ts, ans)
    }
}